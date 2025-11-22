// src/pipeline.rs
use anyhow::*;

use crate::{image::Image, disasm::PpcCs};
use crate::config::RecompilerConfig;
use crate::db::AnalysisDb;

pub struct Ctx<'a> {
    pub img: &'a Image,
    pub cs: &'a PpcCs,
    pub cfg: &'a RecompilerConfig,
    pub db: &'a mut AnalysisDb,
}

pub trait Pass { fn name(&self) -> &'static str; fn run(&self, ctx: &mut Ctx) -> Result<()>; }

pub struct Pipeline { passes: Vec<Box<dyn Pass>> }
impl Pipeline {
    pub fn new() -> Self { Self { passes: vec![] } }
    pub fn add<P: Pass + 'static>(mut self, p: P) -> Self { self.passes.push(Box::new(p)); self }
    pub fn run(&self, mut ctx: Ctx) -> Result<()> {
        for p in &self.passes { p.run(&mut ctx)?; }
        Ok(())
    }
}
