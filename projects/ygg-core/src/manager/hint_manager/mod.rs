pub use self::item::HintItems;
use crate::{Result, YggdrasilError, FILE_MANAGER};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use lsp_types::{CodeLens, Diagnostic, DocumentSymbol, Url};
use std::{
    lazy::SyncLazy,
    ops::{Add, AddAssign},
};

mod cli;
mod item;

pub static HINT_MANAGER: SyncLazy<HintManager> = SyncLazy::new(|| HintManager::default());

pub type HintRef<'a> = Ref<'a, Url, HintItems>;

impl Default for HintManager {
    fn default() -> Self {
        Self { hint_store: Default::default() }
    }
}

#[derive(Clone, Debug)]
pub struct HintManager {
    pub hint_store: DashMap<Url, HintItems>,
}

impl HintManager {
    #[inline]
    pub fn get(&self, url: &Url) -> Option<Ref<'_, Url, HintItems>> {
        self.hint_store.get(url)
    }
    #[inline]
    pub fn get_mut(&self, url: &Url) -> Option<RefMut<'_, Url, HintItems>> {
        self.hint_store.get_mut(url)
    }
    #[inline]
    pub fn set(&self, url: Url, hint: HintItems) -> Option<HintItems> {
        self.hint_store.insert(url, hint)
    }
    pub async fn update(&self, url: &Url) -> Result<HintRef<'_>> {
        FILE_MANAGER.parse_file(&url).await?;
        HINT_MANAGER.hint_store.get(url).ok_or(YggdrasilError::unreachable())

        // if let Some(s) = FILE_MANAGER.parse_file(&url).await? {
        //     self.hint_store.insert(url.to_owned(), s);
        // }
        // self.get(url).ok_or(YGGError::Unreachable)
    }
}
