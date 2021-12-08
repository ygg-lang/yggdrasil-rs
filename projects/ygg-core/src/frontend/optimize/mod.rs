use crate::{
    frontend::{rule::*, GrammarInfo, Rule},
    manager::HintItems,
    Result,
};
use lsp_types::{DocumentSymbol, Range as LSPRange, SymbolKind};
use std::{mem::transmute, ops::Range};
use yggdrasil_bootstrap::ast::Symbol;

mod builtin;
mod fuse;
mod inline;
#[allow(deprecated)]
mod meta_info;

impl GrammarInfo {
    pub async fn optimize(&mut self) -> Result<HintItems> {
        let mut hint = HintItems::default();
        self.link_external().await?;
        hint += self.fusion()?;
        hint += self.inline()?;
        Ok(hint)
    }
    /// optimize without import
    pub fn optimize_local(&mut self) -> Result<HintItems> {
        let mut hint = HintItems::default();
        hint += self.fusion()?;
        hint += self.inline()?;
        Ok(hint)
    }
    async fn link_external(&mut self) -> Result<()> {
        Ok(())
    }
    fn inline(&mut self) -> Result<HintItems> {
        Ok(HintItems::default())
    }
    fn fusion(&mut self) -> Result<HintItems> {
        self.rule_map.values_mut().for_each(|e| e.fuse());
        Ok(HintItems::default())
    }
}

impl Rule {
    fn merge_regex(&mut self) {}
}
