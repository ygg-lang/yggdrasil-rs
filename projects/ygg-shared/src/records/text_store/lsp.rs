use super::*;
use lsp_types::{
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
};

impl TextStore {
    pub fn did_open_text_document(&mut self, input: DidOpenTextDocumentParams) -> Option<Rope> {
        self.insert(input.text_document.uri, &input.text_document.text)
    }
    pub fn did_close_text_document(&mut self, input: DidCloseTextDocumentParams) -> Option<Rope> {
        self.force_update(input.text_document.uri)
    }

    pub fn did_change_text_document(&mut self, input: DidChangeTextDocumentParams) {
        let _url = input.text_document.uri;
    }
    pub fn did_change_watched_files(&mut self, input: DidChangeWatchedFilesParams) {
        for file in input.changes {
            let _url = file.uri;
        }
    }
}
