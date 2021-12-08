use super::*;


impl GrammarInfo {
    pub fn report_meta(&self) -> HintItems {
        let mut document_symbol = vec![];
        document_symbol.push(self.top_area());
        let file = self.file_position();
        for rule in self.rules() {
            document_symbol.push(rule.symbol_item(&file))
        }
        document_symbol.push(test_node());
        HintItems { diagnostic: vec![], code_lens: vec![], document_symbol }
    }
    pub fn file_position(&self) -> GrammarContext<'_> {
        GrammarContext::new(&self.text, &self.url)
    }
    pub fn get_lsp_range(&self, range: &Range<usize>) -> LSPRange {
        self.file_position().get_lsp_range(range)
    }
    fn top_area(&self) -> DocumentSymbol {
        let name = match self.is_grammar {
            true => "Grammar",
            false => "Fragment",
        };
        let range = self.get_lsp_range(&self.name.range);

        let mut terms = vec![];
        if self.is_grammar {
            terms.push(self.extension())
        }
        terms.push(self.ignore());
        DocumentSymbol {
            name: name.to_string(),
            detail: Some(self.name.data.to_owned()),
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: Some(terms),
        }
    }
    fn extension(&self) -> DocumentSymbol {
        let detail;
        let children;
        let file = self.file_position();
        let range = file.get_lsp_range(&self.name.range);
        let n = self.extensions.len();
        match n {
            0 => {
                detail = 0_u8.to_string();
                children = None;
            }
            _ => {
                detail = n.to_string();
                children = Some(
                    self.extensions
                        .iter()
                        .enumerate()
                        .map(|(i, r)| self.extension_item(i, r.data.to_owned(), file.get_lsp_range(&r.range)))
                        .collect(),
                );
            }
        };
        DocumentSymbol {
            name: "extensions".to_owned(),
            detail: Some(detail),
            kind: SymbolKind::ARRAY,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children,
        }
    }
    fn extension_item(&self, index: usize, ext: String, range: LSPRange) -> DocumentSymbol {
        let detail = &ext[1..ext.len() - 1];
        DocumentSymbol {
            name: format!("{}: ", index + 1),
            detail: Some(detail.to_string()),
            kind: SymbolKind::KEY,
            tags: None,
            deprecated: None,
            range,
            selection_range: range,
            children: None,
        }
    }
    fn ignore(&self) -> DocumentSymbol {
        let n = self.ignores.len();
        let children = match n {
            0 => None,
            _ => Some(self.ignores.iter().map(|e| self.ignore_item(e)).collect()),
        };
        DocumentSymbol {
            name: "Ignores".to_string(),
            detail: Some(n.to_string()),
            kind: SymbolKind::NUMBER,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children,
        }
    }
    fn ignore_item(&self, id: &Symbol) -> DocumentSymbol {
        let file = self.file_position();
        let range = file.get_lsp_range(&id.range);
        match self.get(&id.data) {
            Some(s) => {
                let mut s = s.symbol_item(&file);
                s.range = range;
                s.kind = SymbolKind::NUMBER;
                s
            }
            None => DocumentSymbol {
                name: id.data.to_owned(),
                detail: None,
                kind: SymbolKind::NUMBER,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            },
        }
    }
}

impl Rule {
    fn symbol_item(&self, file: &GrammarContext) -> DocumentSymbol {
        let (detail, kind) = self.symbol_detail();
        DocumentSymbol {
            name: self.name.data.to_owned(),
            detail,
            kind,
            tags: None,
            deprecated: None,
            range: file.get_lsp_range(&self.range),
            selection_range: file.get_lsp_range(&self.name.range),
            children: None,
        }
    }
    fn symbol_detail(&self) -> (Option<String>, SymbolKind) {
        if self.force_inline {
            (Some("inlined".to_string()), SymbolKind::VARIABLE)
        }
        else if self.expression.inline_token {
            (Some("token".to_string()), SymbolKind::STRING)
        }
        else if self.eliminate_unnamed {
            (Some("no unnamed".to_string()), SymbolKind::ENUM_MEMBER)
        }
        else if self.eliminate_unmarked {
            (Some("no unmarked".to_string()), SymbolKind::ENUM)
        }
        else if self.expression.is_choice() {
            (None, SymbolKind::CLASS)
        }
        else {
            (None, SymbolKind::FIELD)
        }
    }
}

fn test_node() -> DocumentSymbol {
    DocumentSymbol {
        name: "Testing".to_string(),
        detail: Some("Symbol".to_string()),
        kind: SymbolKind::FILE,
        tags: None,
        deprecated: None,
        range: Default::default(),
        selection_range: Default::default(),
        children: unsafe { Some(test_all_symbol()) },
    }
}

unsafe fn test_all_symbol() -> Vec<DocumentSymbol> {
    (1u32..=26)
        .map(|n| transmute::<u32, SymbolKind>(n))
        .map(|k| DocumentSymbol {
            name: format!("{:?}", k),
            detail: Some(format!("{:?}", k)),
            kind: k,
            tags: None,
            deprecated: None,
            range: Default::default(),
            selection_range: Default::default(),
            children: None,
        })
        .collect()
}
