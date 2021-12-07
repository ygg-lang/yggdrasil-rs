mod cst;
#[cfg(test)]
mod test;

use lsp_document::{IndexedText, TextAdapter, TextMap};
use lsp_types::{Position, Range as LSPRange};
use std::ops::Range;

/// Cache all newlines
pub struct PositionSystem<'input> {
    inner: IndexedText<&'input str>,
    lines: usize,
    count: usize,
}

impl<'i> PositionSystem<'i> {
    #[inline]
    pub fn new(input: &'i str) -> PositionSystem {
        Self { inner: IndexedText::new(input), lines: input.lines().count(), count: input.chars().count() }
    }
    #[inline]
    pub fn update(&mut self, input: &'i str) {
        self.inner = IndexedText::new(input);
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

impl<'i> PositionSystem<'i> {
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        match self.inner.offset_to_pos(offset) {
            Some(s) => (s.line, s.col),
            None => {
                unimplemented!()
            }
        }
    }
    #[inline]
    pub fn get_range(&self, start: usize, end: usize) -> Range<(u32, u32)> {
        match self.inner.offset_range_to_range(Range { start, end }) {
            Some(s) => Range { start: (s.start.line, s.start.col), end: (s.end.line, s.end.col) },
            None => {
                unimplemented!()
            }
        }
    }
}

impl<'i> PositionSystem<'i> {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> LSPRange {
        let range = self.inner.offset_range_to_range(Range { start, end });
        match range.and_then(|f| self.inner.range_to_lsp_range(&f)) {
            Some(s) => s,
            None => LSPRange { start: self.get_lsp_line_column(start), end: self.get_lsp_line_column(end) },
        }
    }
    #[inline]
    fn get_lsp_line_column(&self, offset: usize) -> Position {
        let p = self.inner.offset_to_pos(offset.min(self.inner.text().len()));
        match p.and_then(|f| self.inner.pos_to_lsp_pos(&f)) {
            Some(s) => s,
            None => Position { line: self.lines as u32 + 1, character: 0 },
        }
    }
}
