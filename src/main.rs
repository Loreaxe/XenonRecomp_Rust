// src/main.rs
#![allow(clippy::needless_return)]

use anyhow::*;
use clap::{Parser, Subcommand};
use memmap2::Mmap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use xenon_recomp::db::{AnalysisDb, SymbolInfo};
use xenon_recomp::image::Image;
use xenon_recomp::pipeline::{Ctx, Pipeline};
use xenon_recomp::{config::RecompilerConfig, recompiler::Recompiler};
use xenon_recomp::passes::{
    analyse_function::AnalyseFunctions,
    analyse_switch_scan::AnalyseSwitchScan,
    analyse_switch_bind::AnalyseSwitchBind,
};

use xenon_recomp::disasm::PpcCs;
// Rust XEX loader + import symbol collector
use xenon_recomp::xex::{load_xex2, collect_import_symbols};

use xenon_recomp::log::Phase;
use xenon_recomp::xlog;

/// Xenon static recompiler CLI (pure Rust)
#[derive(Parser)]
#[command(name = "xenon_recomp")]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run analysis passes and print summary (no TOML output)
    Analyse {
        /// Input XEX (mapped directly)
        input: PathBuf,
    },

    /// Recompile using a TOML config, but functions come from fresh analysis
    Recompile {
        /// Path to recompiler config TOML (knobs only; no function list needed)
        config: PathBuf,
    },

    /// Analyse and then recompile in one go (in-memory bridge)
    All {
        /// Input XEX for analysis + recompile
        input: PathBuf,
        /// Optional: config TOML; if omitted, defaults are used
        config: Option<PathBuf>,
    },

    /// Auto mode: look for ./xex/default.xex and run analyse + recompile_all
    Auto,
}

fn populate_function_symbols(db: &mut AnalysisDb) {
    db.symbols.clear();

    for f in &db.functions {
        let name = format!("sub_{:08X}", f.base);
        db.symbols.push(SymbolInfo {
            addr: f.base,
            name,
        });
    }
}

fn print_cpp_style_symbols(db: &AnalysisDb) {
    for sym in &db.symbols {
        // Example for sub_82CC1E98:
        // __attribute__((alias("__imp__sub_82CC1E98"))) PPC_WEAK_FUNC(sub_82CC1E98);
        // PPC_FUNC_IMPL(__imp__sub_82CC1E98) {
        let base_name = &sym.name;
        let imp_name  = format!("__imp__{}", base_name);

        println!(
            "__attribute__((alias(\"{imp_name}\"))) PPC_WEAK_FUNC({base_name});",
        );
        println!("PPC_FUNC_IMPL({imp_name});");
        println!();
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Cmd::Analyse { input } => {
            let _p = Phase::new("cmd::Analyse");
            let (img, cs) = open_xex_and_cs(&input)
                .with_context(|| format!("failed to open XEX {:?}", input))?;

            let mut db = AnalysisDb::default();
            let cfg = RecompilerConfig::default();

            let pipeline = Pipeline::new()
                .add(AnalyseFunctions)
                // No TOML dump: keep it in-memory only
                .add(AnalyseSwitchScan { dump_to: None })
                .add(AnalyseSwitchBind);

            xlog!("CLI: running pipeline on '{}'", input.display());
            pipeline.run(Ctx { img: &img, cs: &cs, cfg: &cfg, db: &mut db })?;
            xlog!(
                "CLI: Analyse complete; functions={}, switches={}",
                db.functions.len(),
                db.switches.len()
            );
        }

        Cmd::Recompile { config } => {
            let _p = Phase::new("cmd::Recompile");

            // 0) Load config (for knobs like directories, hooks, etc.)
            let mut cfg = RecompilerConfig::load_from_file(&config.to_string_lossy())?;
            xlog!("CLI: loaded config '{}'", config.display());

            // 1) Resolve XEX path from config
            let xex_path = resolve_xex_path(&cfg)
                .context("config.file_path (and/or directory_path) not set")?;

            // 2) Open XEX, get blob (extern handling is done by the recompiler itself)
            let (img, _cs_loader, blob) = open_xex_cs_and_blob(Path::new(&xex_path))?;

            // 3) Run analysis pipeline → AnalysisDb (in-memory only)
            let mut db = AnalysisDb::default();
            let cs_for_pass = build_ppc_cs()?;
            let pipeline = Pipeline::new()
                .add(AnalyseFunctions)
                .add(AnalyseSwitchScan { dump_to: None })
                .add(AnalyseSwitchBind);
            pipeline.run(Ctx { img: &img, cs: &cs_for_pass, cfg: &cfg, db: &mut db })?;
            xlog!(
                "CLI: Recompile Analyse→ functions={} switches={}",
                db.functions.len(),
                db.switches.len()
            );

            // 4) Resolve output dir based on config + XEX path
            let (output_dir, _output_file) = derive_output_paths(&cfg, &xex_path);
            fs::create_dir_all(&output_dir)
                .with_context(|| format!("failed to create {}", output_dir.display()))?;

            // Point recompiler at the output directory
            cfg.out_directory_path = output_dir.to_string_lossy().into_owned();

            // 5) Build recompiler and seed DIRECTLY from AnalysisDb
            let mut r = Recompiler::new_with_blob(img, cfg.clone(), blob);
            r.seed_from_analysis_db(&db); // ignore any [functions] in TOML, trust analysis

            // 6) Emit everything (including xam.rs/xboxkrnl.rs via emit_externs_if_any)
            r.recompile_all()?;
            r.write_mod_index("lib.rs")?;

            println!("✅ Module index written to {}", output_dir.join("lib.rs").display());
        }

        Cmd::All { input, config } => {
            let _p = Phase::new("cmd::All");
            let (img, cs, blob) = open_xex_cs_and_blob(&input)
                .with_context(|| format!("failed to open XEX {:?}", input))?;

            let mut cfg = if let Some(p) = config {
                RecompilerConfig::load_from_file(&p.to_string_lossy())?
            } else {
                RecompilerConfig::default()
            };

            // 1) Run analysis pipeline → AnalysisDb (in-memory only)
            let mut db = AnalysisDb::default();
            let pipeline = Pipeline::new()
                .add(AnalyseFunctions)
                .add(AnalyseSwitchScan { dump_to: None })
                .add(AnalyseSwitchBind);
            pipeline.run(Ctx { img: &img, cs: &cs, cfg: &cfg, db: &mut db })?;
            xlog!(
                "CLI: Analyse→ functions={} switches={}",
                db.functions.len(),
                db.switches.len()
            );

            // 2) Resolve output dir
            let xex_path =
                resolve_xex_path(&cfg).unwrap_or_else(|| input.to_string_lossy().into_owned());
            let (output_dir, _output_file) = derive_output_paths(&cfg, &xex_path);
            fs::create_dir_all(&output_dir)
                .with_context(|| format!("failed to create {}", output_dir.display()))?;

            // 3) Point recompiler at the output directory
            cfg.out_directory_path = output_dir.to_string_lossy().into_owned();

            // 4) Build recompiler and seed DIRECTLY from AnalysisDb
            let mut r = Recompiler::new_with_blob(img, cfg.clone(), blob);
            r.seed_from_analysis_db(&db); // trust the analysis DB
            r.recompile_all()?;
            r.write_mod_index("lib.rs")?;

            println!("✅ Module index written to {}", output_dir.join("lib.rs").display());
        }

        Cmd::Auto => {
            let _p = Phase::new("cmd::Auto");
            let default_path = default_xex_path();
            if !default_path.exists() {
                bail!(
                    "auto mode: expected XEX at {} but it does not exist",
                    default_path.display()
                );
            }

            let output_dir = default_output_dir();
            if !output_dir.exists() {
                fs::create_dir_all(&output_dir)
                    .with_context(|| format!("failed to create {}", output_dir.display()))?;
            }

            let (img, _cs_for_loader, blob) = open_xex_cs_and_blob(&default_path)
                .with_context(|| format!("failed to open XEX {}", default_path.display()))?;

            // Base config for auto-mode
            let mut cfg = RecompilerConfig::default();
            cfg.directory_path = "xex/".to_string();
            cfg.file_path = "default.xex".to_string();
            cfg.out_directory_path = output_dir.to_string_lossy().into_owned();

            // 1) Run analysis passes with a dedicated Capstone handle
            let mut db = AnalysisDb::default();
            let cs_for_pass = build_ppc_cs()?;
            let pipeline = Pipeline::new()
                .add(AnalyseFunctions)
                .add(AnalyseSwitchScan { dump_to: None })
                .add(AnalyseSwitchBind);
            pipeline.run(Ctx { img: &img, cs: &cs_for_pass, cfg: &cfg, db: &mut db })?;
            xlog!(
                "CLI: Auto Analyse→ functions={} switches={}",
                db.functions.len(),
                db.switches.len()
            );

            // 2) Recompiler seeded from AnalysisDb (no re-carving)
            let mut r = Recompiler::new_with_blob(img, cfg, blob);
            r.seed_from_analysis_db(&db); // trust the analysis DB
            r.recompile_all()?;
            r.write_mod_index("lib.rs")?;

            println!("✅ Module index written to {}", output_dir.join("lib.rs").display());
        }
    }

    Ok(())
}

fn open_xex_and_cs(path: &Path) -> Result<(Image, PpcCs)> {
    let _p = Phase::new("open_xex_and_cs");
    let f = File::open(path)?;
    let map = unsafe { Mmap::map(&f)? };

    let img = load_xex2(&map).with_context(|| "load_xex2 failed")?;

    if let std::result::Result::Ok(imports) = collect_import_symbols(&map, &img) {
        if !imports.is_empty() {
            eprintln!("ℹ️  collected {} import thunk names", imports.len());
        }
    }

    let cs = build_ppc_cs()?;
    Ok((img, cs))
}

/// Same as open_xex_and_cs, but also returns a Vec<u8> copy of the XEX for
/// recompiler use (e.g. import/external handling inside Recompiler).
fn open_xex_cs_and_blob(path: &Path) -> Result<(Image, PpcCs, Vec<u8>)> {
    let _p = Phase::new("open_xex_cs_and_blob");
    let f = File::open(path)?;
    let map = unsafe { Mmap::map(&f)? };

    let img = load_xex2(&map).with_context(|| "load_xex2 failed")?;

    // Still collect imports for info, but we no longer emit externs.rs here.
    if let std::result::Result::Ok(imports) = collect_import_symbols(&map, &img) {
        if !imports.is_empty() {
            eprintln!("ℹ️  collected {} import thunk names", imports.len());
        }
    }

    let cs = build_ppc_cs()?;
    let blob: Vec<u8> = map[..].to_vec();
    Ok((img, cs, blob))
}

/// Build a PowerPC Capstone engine for Xenon and enable detail decoding.
fn build_ppc_cs() -> Result<PpcCs> {
    let cs = PpcCs::new()?;
    Ok(cs)
}

fn resolve_xex_path(cfg: &RecompilerConfig) -> Option<String> {
    if cfg.file_path.is_empty() {
        return None;
    }
    if cfg.directory_path.is_empty() {
        Some(cfg.file_path.clone())
    } else {
        Some(format!("{}{}", cfg.directory_path, cfg.file_path))
    }
}

fn default_xex_path() -> PathBuf {
    let mut base = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    base.pop(); // remove the binary name (target/debug)
    base.push("xex/default.xex");
    base
}

fn default_output_dir() -> PathBuf {
    let mut base = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    base.pop();
    base.push("xex/output");
    base
}

/// Build `xex/output/<stem>.rs` alongside the XEX (or cfg dir if set)
fn derive_output_paths(cfg: &RecompilerConfig, xex_path: &str) -> (PathBuf, PathBuf) {
    let base_dir = if !cfg.directory_path.is_empty() {
        PathBuf::from(&cfg.directory_path)
    } else {
        PathBuf::from(xex_path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    };
    let out_dir = base_dir.join("output");

    let stem = Path::new(xex_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("recompiled");

    let file = out_dir.join(format!("{stem}.rs"));
    (out_dir, file)
}
