// src/recompiler/symbol.rs
//
// Unified symbol utilities for the recompiler:
//
//  - SymbolKind / Symbol / SymbolTable: used by analysis + recompiler
//    to track function symbols (and, optionally, import thunks / data).
//
//  - XAM / XboxKrnl export ordinal tables and `build_export_maps()`,
//    used by XEX import thunk collection to resolve names.
//

use std::collections::{BTreeMap, HashMap};

// ===============================
// Generic symbol table
// ===============================

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    // Future:
    // ImportThunk,
    // Data,
    // Export,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub address: u32,
    pub size: u32,
    pub kind: SymbolKind,
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    pub by_addr: BTreeMap<u32, Symbol>,
}

impl SymbolTable {
    /// Insert a function symbol.
    pub fn insert_func(
        &mut self,
        name: impl Into<String>,
        address: u32,
        size: u32,
    ) {
        self.by_addr.insert(
            address,
            Symbol {
                name: name.into(),
                address,
                size,
                kind: SymbolKind::Function,
            },
        );
    }

    /// Generic insert if you ever add more kinds.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        address: u32,
        size: u32,
        kind: SymbolKind,
    ) {
        self.by_addr.insert(
            address,
            Symbol {
                name: name.into(),
                address,
                size,
                kind,
            },
        );
    }

    /// Lookup by exact VA.
    pub fn get(&self, address: u32) -> Option<&Symbol> {
        self.by_addr.get(&address)
    }

    /// Iterate in address order.
    pub fn iter(&self) -> impl Iterator<Item = &Symbol> {
        self.by_addr.values()
    }

    /// Bulk insert thunk imports `(VA, name)` with a fixed 16-byte size.
    pub fn insert_import_thunks<I: IntoIterator<Item = (u32, String)>>(
        &mut self,
        pairs: I,
    ) {
        for (addr, name) in pairs {
            self.insert_func(name, addr, 16);
        }
    }

    /// Convenience: create `sub_XXXXXXXX` names from the function DB.
    pub fn populate_from_functions<'a, I>(&mut self, funcs: I)
    where
        I: IntoIterator<Item = &'a crate::db::FunctionInfo>,
    {
        for f in funcs {
            let name = format!("sub_{:08X}", f.base);
            self.insert_func(name, f.base, f.size);
        }
    }
}

// ===============================
// XAM / XboxKrnl export tables
// ===============================
//
// These used to live in src/xbox_symbols.rs. They are here now so all symbol
// logic (including XEX import resolution) is in one module.
//

// Paths are relative to this file. The `.inc` files still live under src/xbox/.
const XAM_INC: &str  = include_str!("../xbox/xam_table.inc");
const KRNL_INC: &str = include_str!("../xbox/xboxkrnl_table.inc");

fn parse_exports_inc(inc: &'static str) -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();

    for line in inc.lines() {
        let line = line.trim();
        if !line.starts_with("XE_EXPORT(") {
            continue;
        }

        if let (Some(l), Some(r)) = (line.find('('), line.rfind(')')) {
            let args = &line[l + 1 .. r];
            let mut parts = args.split(',').map(|s| s.trim());
            let _module = parts.next().unwrap_or_default();
            let ord_str = parts.next().unwrap_or_default();
            let name    = parts.next().unwrap_or_default();

            let ord = if let Some(s) = ord_str.strip_prefix("0x") {
                u32::from_str_radix(s, 16).unwrap_or(0)
            } else {
                ord_str.parse::<u32>().unwrap_or(0)
            };

            if ord != 0 && !name.is_empty() {
                map.insert(ord, name);
            }
        }
    }

    map
}

/// Return `(xam_exports, xboxkrnl_exports)` where each map is
/// `ordinal -> symbol_name`.
pub fn build_export_maps() -> (
    HashMap<u32, &'static str>,
    HashMap<u32, &'static str>,
) {
    (parse_exports_inc(XAM_INC), parse_exports_inc(KRNL_INC))
}
