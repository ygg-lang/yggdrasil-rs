pub use self::{file_store::FileStore, file_wrap::FileType, finger_print::FileFingerprint};
use crate::{
    frontend::{GrammarContext, GrammarInfo, GrammarType, Translator},
    manager::{global_parser::PARSER_MANAGER, HintItems},
    Result, YggdrasilError, HINT_MANAGER,
};
use dashmap::{mapref::one::Ref, DashMap};
use lsp_types::{Diagnostic, Url};
use std::{fs, lazy::SyncLazy, path::Path};
use xxhash_rust::xxh3::xxh3_128;

mod file_store;
mod file_wrap;
mod finger_print;

pub static FILE_MANAGER: SyncLazy<FileManager> = SyncLazy::new(|| FileManager::default());

//#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct FileManager {
    store: DashMap<Url, FileStore>,
}

impl Default for FileManager {
    fn default() -> Self {
        Self { store: Default::default() }
    }
}

impl FileManager {
    pub fn load_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }
    pub fn save_cache(&self, _path: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }

    pub fn update_url(&self, url: Url) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = FileStore::load_url(&url, new)?;
        self.store.insert(url, file);
        Ok(())
    }
    pub fn update_url_text(&self, url: Url, text: String) -> Result<()> {
        let new = FileFingerprint::new(&url)?;
        match self.store.get(&url) {
            Some(old) if old.eq(&new) => return Ok(()),
            _ => (),
        }
        let file = match url.to_file_path()?.extension().and_then(|e| e.to_str()) {
            Some("toml") => Ok(FileStore::new_type(new.fingerprint, text)),
            Some("ygg") | Some("yg") => Ok(FileStore::new_grammar(new.fingerprint, text)),
            _ => Err(YggdrasilError::language_error("Unsupported file extension")),
        }?;
        self.store.insert(url, file);
        Ok(())
    }
    #[inline]
    pub fn get_file(&self, url: &Url) -> Option<Ref<'_, Url, FileStore>> {
        self.store.get(&url)
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
    pub async fn parse_file(&self, url: &Url) -> Result<Ref<'_, Url, FileStore>> {
        match url.to_file_path()?.extension().and_then(|e| e.to_str()) {
            Some("toml") => {
                self.parse_type(url).await?;
                Ok(())
            }
            Some("ygg") | Some("yg") => {
                self.parse_grammar(url).await?;
                Ok(())
            }
            _ => Err(YggdrasilError::language_error("Unsupported file extension")),
        }?;
        match self.get_file(url) {
            Some(s) => Ok(s),
            None => Err(YggdrasilError::unreachable()),
        }
    }

    pub async fn parse_type(&self, _url: &Url) -> Result<&GrammarType> {
        unimplemented!()
    }

    pub async fn parse_grammar(&self, url: &Url) -> Result<GrammarInfo> {
        self.update_url(url.to_owned())?;
        self.store
            .get_mut(url)
            .ok_or(YggdrasilError::language_error("Grammar not found"))?
            .value_mut()
            .parse_ygg(url.to_owned())
            .await
    }
}
