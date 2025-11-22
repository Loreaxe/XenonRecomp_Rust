// src/config.rs
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecompilerSwitchTable {
    pub r: u32,
    pub labels: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecompilerMidAsmHook {
    pub name: String,
    #[serde(default)]
    pub registers: Vec<String>,
    #[serde(default)]
    pub ret: bool,
    #[serde(default)]
    pub returnOnTrue: bool,
    #[serde(default)]
    pub returnOnFalse: bool,
    #[serde(default)]
    pub expectZero: bool,
    #[serde(default)]
    pub expectOne: bool,
    #[serde(default)]
    pub jumpAddressOnTrue: u32,
    #[serde(default)]
    pub jumpAddressOnFalse: u32,
    #[serde(default)]
    pub afterInst: bool,
    #[serde(default)]
    pub code: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecompilerConfig {
    // paths
    #[serde(default)] pub file_path: String,
    #[serde(default)] pub patch_file_path: String,
    #[serde(default)] pub patched_file_path: String,
    #[serde(default)] pub out_directory_path: String,
    #[serde(default)] pub switch_table_file_path: String,

    // switches & flags (subset; extend as needed)
    #[serde(default)] pub skip_lr: bool,
    #[serde(default)] pub skip_lr_on_far_calls: bool,
    #[serde(default)] pub skip_update_env_ptr: bool,
    #[serde(default)] pub assume_stack_valid: bool,
    #[serde(default)] pub emit_blr_on_return: bool,
    #[serde(default)] pub enable_retsled: bool,
    #[serde(default)] pub fast_mem: bool,

    // Known helper function addresses (subset)
    #[serde(default)] pub ret_single_wrapper: u32,
    #[serde(default)] pub ret_double_wrapper: u32,
    #[serde(default)] pub saveGpr14Address: u32,
    #[serde(default)] pub restGpr14Address: u32,
    #[serde(default)] pub restFpr14Address: u32,
    #[serde(default)] pub saveFpr14Address: u32,
    #[serde(default)] pub restVmx14Address: u32,
    #[serde(default)] pub saveVmx14Address: u32,
    #[serde(default)] pub restVmx64Address: u32,
    #[serde(default)] pub saveVmx64Address: u32,
    #[serde(default)] pub longJmpAddress: u32,
    #[serde(default)] pub setJmpAddress: u32,

    // maps
    #[serde(default)] pub functions: HashMap<u32, u32>,
    #[serde(default)] pub invalid_instructions: HashMap<u32, u32>,
    #[serde(default)] pub mid_asm_hooks: HashMap<u32, RecompilerMidAsmHook>,

    // derived (not from file)
    #[serde(skip)] pub directory_path: String,
}

impl RecompilerConfig {
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let txt = std::fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&txt)?;
        if let Some(pos) = path.rfind(['/', '\\']) {
            cfg.directory_path = path[..=pos].to_string();
        } else {
            cfg.directory_path = String::new();
        }
        Ok(cfg)
    }
}
