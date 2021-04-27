pub use self::{file_store::FileStore, file_wrap::FileType, finger_print::FileFingerprint};
use crate::{
    codegen::{GrammarState, GrammarType},
    ygg::ast::YGGBuilder,
    Result, YGGError,
};
use lsp_types::Url;
use std::{collections::HashMap, fs, lazy::SyncLazy, path::Path};
use tokio::sync::RwLock;
use xxhash_rust::xxh3::xxh3_128;

mod file_store;
mod file_wrap;
mod finger_print;

#[rustfmt::skip]
pub static FILE_MANAGER: SyncLazy<RwLock<FileManager>> = SyncLazy::new(|| {
    RwLock::new(FileManager::new().expect("Manager initialization failed"))
});

//#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct FileManager {
    builder: YGGBuilder,
    file_store: HashMap<Url, FileStore>,
}

impl FileManager {
    pub fn new() -> Result<Self> {
        Ok(Self { builder: YGGBuilder::new()?, file_store: Default::default() })
    }

    pub fn load_cache(&mut self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }
    pub fn save_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }

    pub fn update_url(&mut self, url: Url) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.file_store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = FileStore::load_url(&url, new)?;
        self.file_store.insert(url, file);
        Ok(())
    }
    #[inline]
    pub fn get_file(&self, url: &Url) -> Option<&FileStore> {
        self.file_store.get(&url)
    }
    #[inline]
    pub fn get_file_mut(&mut self, url: &Url) -> Option<&mut FileStore> {
        self.file_store.get_mut(&url)
    }
    pub fn get_symbol(&self, url: &Url, _symbol: &str) -> Result<()> {
        let _f = self.get_file(url);
        unimplemented!()
    }
    pub fn get_type(&self, _ty: &str) -> Result<()> {
        unimplemented!()
    }
}

impl FileManager {
    pub async fn parse_file(&mut self, url: &Url) -> Result<&FileStore> {
        match url.to_file_path()?.extension().and_then(|e| e.to_str()) {
            Some("toml") => {
                self.parse_type(url).await?;
                match self.get_file(url) {
                    Some(s) => Ok(s),
                    None => Err(YGGError::Unreachable),
                }
            }
            Some("ygg") | Some("yg") => {
                self.parse_grammar(url).await?;
                match self.get_file(url) {
                    Some(s) => Ok(s),
                    None => Err(YGGError::Unreachable),
                }
            }
            _ => Err(YGGError::language_error("Unsupported file extension")),
        }
    }

    pub async fn parse_type(&mut self, _url: &Url) -> Result<&GrammarType> {
        unimplemented!()
    }

    pub async fn parse_grammar(&mut self, url: &Url) -> Result<&GrammarState> {
        self.update_url(url.to_owned())?;
        let parser = &mut self.builder;
        let s = match self.file_store.get_mut(url) {
            Some(s) => Ok(s),
            _ => Err(YGGError::language_error("Grammar not found")),
        }?;
        s.parse_ygg(url.to_owned(), parser).await
    }
}
