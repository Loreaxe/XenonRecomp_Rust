// @generated - externs table (addresses from import thunks)
use crate::recompiler::ppc_context::PPCContext;

pub static EXTERNS: &[(u32, &str)] = &[
];

pub fn call(_ctx: &mut PPCContext, _base: *mut u8, addr: u32) {
    // NOTE: this is a stub dispatcher. Implement host bindings here.
    match EXTERNS.binary_search_by_key(&addr, |e| e.0) {
        Ok(i) => { let (_a, name) = EXTERNS[i]; eprintln!("[extern] 0x{:08X} => {} (unimplemented)", addr, name); }
        Err(_) => eprintln!("[extern] 0x{:08X} => <unknown> (unimplemented)", addr),
    }
}
