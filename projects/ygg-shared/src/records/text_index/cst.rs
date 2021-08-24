use super::*;
use lsp_types::{Position, Range};

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> Range {
        text.get_lsp_range(self.range.start, self.range.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &TextIndex) -> Position {
        lines.get_lsp_position(self.range.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &TextIndex) -> Position {
        lines.get_lsp_position(self.range.end)
    }
}
