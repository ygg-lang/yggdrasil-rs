mod cst;
#[cfg(test)]
mod test;
mod traits;

pub use lsp_types::{Position as LSPPosition, Range as LSPRange};

use lsp_document::{IndexedText, TextAdapter, TextMap};
use std::ops::Range;

/// Cache all newlines
pub struct TextIndex {
    inner: IndexedText<String>,
    lines: usize,
    count: usize,
}

impl TextIndex {
    #[inline]
    pub fn new(input: impl Into<String>) -> Self {
        let input = input.into();
        let lines = input.lines().count();
        let count = input.chars().count();
        Self { inner: IndexedText::new(input), lines, count }
    }
    #[inline]
    pub fn update(&mut self, input: impl Into<String>) {
        self.inner = IndexedText::new(input.into());
    }
    #[inline]
    pub fn get_text(&self) -> &'_ str {
        self.inner.text()
    }
    #[inline]
    pub fn count_text(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn get_nth_line(&self, line: usize) -> Option<&'_ str> {
        self.get_text().lines().nth(line)
    }
}

impl TextIndex {
    #[inline]
    pub fn get_range(&self, start: usize, end: usize) -> Range<(u32, u32)> {
        match self.inner.offset_range_to_range(Range { start, end }) {
            Some(s) => Range { start: (s.start.line, s.start.col), end: (s.end.line, s.end.col) },
            None => Range { start: self.get_line_column(start), end: self.get_line_column(end) },
        }
    }
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        match self.inner.offset_to_pos(offset) {
            Some(s) => (s.line, s.col),
            None => (self.lines as u32 + 1, 0),
        }
    }
}

impl TextIndex {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> LSPRange {
        let range = self.inner.offset_range_to_range(Range { start, end });
        match range.and_then(|f| self.inner.range_to_lsp_range(&f)) {
            Some(s) => s,
            None => LSPRange { start: self.get_lsp_position(start), end: self.get_lsp_position(end) },
        }
    }
    #[inline]
    fn get_lsp_position(&self, offset: usize) -> LSPPosition {
        let p = self.inner.offset_to_pos(offset.min(self.inner.text().len()));
        match p.and_then(|f| self.inner.pos_to_lsp_pos(&f)) {
            Some(s) => s,
            None => LSPPosition { line: self.lines as u32 + 1, character: 0 },
        }
    }
}
