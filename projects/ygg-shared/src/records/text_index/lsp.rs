use super::*;
use lsp_types::{Position, Range};

impl TextIndex {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> Range {
        let range = self.offset_range_to_range(std::ops::Range { start, end });
        match range.and_then(|f| self.range_to_lsp_range(&f)) {
            Some(s) => s,
            None => Range { start: self.get_lsp_position(start), end: self.get_lsp_position(end) },
        }
    }
    #[inline]
    pub fn get_lsp_position(&self, offset: usize) -> Position {
        let p = self.offset_to_pos(offset.min(self.text.len()));
        match p.and_then(|f| self.pos_to_lsp_pos(&f)) {
            Some(s) => s,
            None => Position { line: self.lines as u32 + 1, character: 0 },
        }
    }
}