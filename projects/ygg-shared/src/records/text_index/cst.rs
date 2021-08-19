use super::*;
use crate::records::CSTNode;

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> LSPRange {
        text.get_lsp_range(self.range.start, self.range.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &TextIndex) -> LSPPosition {
        lines.get_lsp_position(self.range.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &TextIndex) -> LSPPosition {
        lines.get_lsp_position(self.range.end)
    }
}
