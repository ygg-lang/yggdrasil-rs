use crate::io::read_url;
use lspower::lsp::{Url, *};
use state::Storage;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};
use lspower::jsonrpc::{Result,Error};
use tokio::sync::RwLock;
use yggdrasil_bootstript::{
    ast::YGGBuilder, codegen::GrammarState, GrammarManager, GRAMMAR_MANAGER,
};

pub trait FileStateUpdate<T> {
    fn update(&mut self, p: T) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct FileState {
    version: usize,
    text: String,
}

impl Default for FileState {
    fn default() -> Self {
        Self { version: 0, text: String::new() }
    }
}

impl FileStateUpdate<DidOpenTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidOpenTextDocumentParams) -> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_|Error::internal_error())
    }
}

impl FileStateUpdate<DidChangeTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidChangeTextDocumentParams) -> Result<()> {
        // TODO: Incremental update
        // let url = p.text_document.uri;
        // let v = p.text_document.version as usize;
        // let text = p.content_changes.iter().rev().nth(0).map(|e| e.text.clone()).unwrap_or_default();
        // self.update_versioned(&url, v, text)
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_|Error::internal_error())
    }
}

impl FileStateUpdate<DidSaveTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidSaveTextDocumentParams)-> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_|Error::internal_error())
    }
}

impl FileStateUpdate<DidCloseTextDocumentParams> for GrammarManager {
    fn update(&mut self, p: DidCloseTextDocumentParams) -> Result<()> {
        let url = p.text_document.uri;
        self.update_url(url).map_err(|_|Error::internal_error())
    }
}
