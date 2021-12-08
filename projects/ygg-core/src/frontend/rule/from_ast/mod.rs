mod import;
mod macros;

use super::{
    hints::{duplicate_declaration_error, name_missing, top_area_error},
    *,
};
use crate::manager::{GLOBAL_ROOT, WORKSPACE_ROOT};
use std::{mem::take, ops::Range};
use yggdrasil_bootstrap::ast::{ImportStatement, MacroCall, StringLiteral, SymbolAlias};
use indexmap::map::IndexMap;

pub struct GrammarContext<'i> {
    url: &'i Url,
    text: &'i str,
    name: Option<String>,
    name_position: Range<usize>,
    is_top_area: bool,
    is_grammar: Option<bool>,
    hints: HintItems,
    ignore: (Range<usize>, Vec<Symbol>),
    import: IndexMap<Url, (Range<usize>, Vec<SymbolAlias>)>,
    extensions: Vec<StringLiteral>,
    rule_map: IndexMap<String, Rule>,
}

impl<'i> GrammarContext<'i> {
    #[inline]
    pub fn new(text: &'i str, url: &'i Url) -> Self {
        Self {
            text,
            url,
            name: None,
            name_position: Default::default(),
            import: Default::default(),
            is_top_area: true,
            is_grammar: None,
            hints: Default::default(),
            ignore: Default::default(),
            extensions: vec![],
            rule_map: Default::default(),
        }
    }
    #[inline]
    pub fn reset(&mut self, text: &'i str, url: &'i Url) {
        self.text = text;
        self.url = url;
        self.is_grammar = None;
        self.hints = Default::default();
    }
    #[inline]
    pub fn get_text(&self) -> &'i str {
        self.text
    }
    #[inline]
    pub fn get_url(&self) -> &'i Url {
        self.url
    }
    #[inline]
    pub fn get_text_index(&self) -> TextIndex {
        TextIndex::new(self.text)
    }
    #[inline]
    pub fn get_lsp_range(&self, offsets: &Range<usize>) -> LSPRange {
        self.get_text_index().get_lsp_range(offsets.start, offsets.end)
    }
    #[inline]
    pub fn get_hints(&self) -> &HintItems {
        &self.hints
    }
}

impl<'i> GrammarContext<'i> {
    #[inline]
    fn must_at_top_area(&mut self, source: &str, message: &str, range: &Range<usize>) {
        if !self.is_top_area {
            self.hints.diagnostic.push(top_area_error(source, message, range, self))
        }
    }
    #[inline]
    fn must_not_duplicate(
        &mut self,
        source: &str,
        message: impl Into<String>,
        this_range: &Range<usize>,
        last_range: &Range<usize>,
    ) {
        self.hints.diagnostic.push(duplicate_declaration_error(source, message, this_range, last_range, self))
    }
    fn set_ignores(&mut self, rules: Vec<Symbol>, this_range: Range<usize>) {
        let last_range = self.ignore.0.to_owned();
        if self.ignore.1.is_empty() {
            self.ignore = (this_range, rules);
        }
        else {
            self.must_not_duplicate("Ignore", "Already declaration ignore statement", &this_range, &last_range)
        }
    }
    fn set_grammar(&mut self, range: Range<usize>, name: String, extensions: Vec<StringLiteral>) {
        let last_range = self.name_position.to_owned();
        match self.is_grammar {
            Some(true) => self.must_not_duplicate("Grammar", "Already declaration as `grammar!`", &range, &last_range),
            Some(false) => self.must_not_duplicate("Grammar", "Already declaration as `fragment!`", &range, &last_range),
            None => {
                self.is_grammar = Some(true);
                self.name_position = range;
                self.extensions = extensions;
                self.name = Some(name)
            }
        }
    }
    fn set_fragment(&mut self, range: Range<usize>, name: String) {
        let last_range = self.name_position.to_owned();
        match self.is_grammar {
            Some(true) => self.must_not_duplicate("Fragment", "Already declaration as `grammar!`", &range, &last_range),
            Some(false) => self.must_not_duplicate("Fragment", "Already declaration as `fragment!`", &range, &last_range),
            None => {
                self.is_grammar = Some(false);
                self.name_position = range;
                self.name = Some(name)
            }
        }
    }
    fn build_grammar_info(&mut self) -> GrammarInfo {
        let name = Symbol {
            data: match &self.name {
                Some(s) => s.to_owned(),
                None => {
                    self.hints.code_lens.push(name_missing());
                    String::from("<anonymous>")
                }
            },
            range: self.name_position.to_owned(),
        };
        let mut imports = IndexMap::with_capacity(self.import.len());
        for (k, (_, v)) in take(&mut self.import) {
            imports.insert(k, v);
        }
        let ignores = take(&mut self.ignore).1;
        GrammarInfo {
            name,
            url: self.url.to_owned(),
            text: self.get_text().to_owned(),
            is_grammar: self.is_grammar.unwrap_or(false),
            extensions: take(&mut self.extensions),
            rule_map: take(&mut self.rule_map),
            ignores,
            imports,
        }
    }
}

pub trait Translator {
    fn translate(self, ctx: &mut GrammarContext) -> Result<GrammarInfo>;
}

impl Translator for Program {
    fn translate(self, ctx: &mut GrammarContext) -> Result<GrammarInfo> {
        let mut doc_buffer = String::new();
        for stmt in self.statement {
            match stmt {
                Statement::Grammar(s) => {
                    ctx.must_at_top_area("Grammar", "Grammar statement must be declared at the top", &s.range);
                    ctx.set_grammar(s.range, s.id.data, s.ext);
                }
                Statement::Fragment(s) => {
                    ctx.must_at_top_area("Fragment", "Fragment statement must be declared at the top", &s.range);
                    ctx.set_fragment(s.range, s.id.data);
                }
                Statement::Ignore(s) => {
                    ctx.must_at_top_area("Ignore", "Ignore statement must be declared at the top", &s.range);
                    ctx.set_ignores(s.rules, s.range)
                }
                Statement::Assign(s) => {
                    ctx.is_top_area = false;
                    let mut rule = Rule::from(*s);
                    swap(&mut rule.doc, &mut doc_buffer);
                    match ctx.rule_map.get(&rule.name.data) {
                        Some(old) => {
                            let msg = format!("Already declaration as Rule `{}`", old.name.data);
                            let last_range = old.name.range.to_owned();
                            ctx.must_not_duplicate("Rule", msg, &rule.range, &last_range)
                        }
                        None => {
                            ctx.rule_map.insert(rule.name.data.to_owned(), rule);
                        }
                    }
                }
                Statement::CommentDocument(text) => doc_buffer.extend(text.doc.chars().chain("\n".chars())),
                Statement::Import(i) => ctx.read_import(*i),
                Statement::MacroCall(m) => ctx.read_macros(*m),
            }
        }
        Ok(ctx.build_grammar_info())
    }
}

impl From<AssignStatement> for Rule {
    fn from(s: AssignStatement) -> Self {
        let name = &s.id.data;
        let ty = Symbol { data: name.to_case(Case::UpperCamel), range: s.id.range.to_owned() };
        let force_inline = name.starts_with("_");
        // if !force_inline {
        //     ty = Some(Symbol { data: name.to_case(Case::UpperCamel), range: s.id.range })
        // }
        let mut try_inline_token = false;
        let mut eliminate_unmarked = false;
        let mut eliminate_unnamed = false;
        for char in s.eq.chars() {
            match char {
                '_' => eliminate_unnamed = true,
                '^' => eliminate_unmarked = true,
                '@' => try_inline_token = true,
                _ => (),
            }
        }
        let mut expression = ExpressionNode::from(s.rhs);
        expression.inline_token = try_inline_token;
        Self {
            name: s.id,
            ty,
            doc: "".to_string(),
            custom_methods: Default::default(),
            force_inline,
            force_box: false,
            already_inline: false,
            eliminate_unmarked,
            eliminate_unnamed,
            expression,
            range: s.range,
        }
    }
}
