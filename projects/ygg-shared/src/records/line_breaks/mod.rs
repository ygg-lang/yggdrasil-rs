#[cfg(test)]
mod test;
mod cst;

use std::ops::Range;
use lsp_document::{IndexedText, Pos, TextAdapter, TextMap};
use lsp_types::{Position, Range as LSPRange};



/// Cache all newlines
pub struct PositionSystem<'input> {
    inner: IndexedText<&'input str>,
}

impl<'i> PositionSystem<'i> {
    #[inline]
    pub fn new(input: &'i str) -> PositionSystem {
        Self { inner: IndexedText::new(input) }
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
    pub fn get_nth_line(&self, line: usize) -> Option<&'_ str> {
        self.get_text().lines().nth(line)
    }
}

impl<'i> PositionSystem<'i> {
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        match self.inner.offset_to_pos(offset) {
            Some(s) => { (s.line, s.col) }
            None => { (0, 0) }
        }
    }
    #[inline]
    pub fn get_range(&self, start: usize, end: usize) -> Range<(u32, u32)> {
        match self.inner.offset_range_to_range(Range { start, end }) {
            Some(s) => {
                Range {
                    start: (s.start.line,s.start.col),
                    end: (s.end.line,s.end.col)
                }
            },
            None => {unimplemented!()}
        }
    }

}

impl<'i> PositionSystem<'i> {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> LSPRange {
        let range = self.inner.offset_range_to_range(Range { start, end });
        match range.and_then(|f| self.inner.range_to_lsp_range(&f)) {
            Some(s) => {s},
            None => {unimplemented!()}
        }
    }
    #[inline]
    fn get_lsp_line_column(&self, offset: usize) -> Position {
        let p = self.inner.offset_to_pos(offset);
        match p.and_then(|f| self.inner.pos_to_lsp_pos(&f)) {
            Some(s) => {s}
            None => {unimplemented!()}
        }
    }
}
