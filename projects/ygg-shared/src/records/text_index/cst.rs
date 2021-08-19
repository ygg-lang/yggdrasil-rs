use super::*;
use crate::records::CSTNode;

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, lines: &TextIndex) -> LSPRange {
        lines.get_lsp_range(self.start, self.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &TextIndex) -> LSPPosition {
        lines.get_lsp_position(self.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &TextIndex) -> LSPPosition {
        lines.get_lsp_position(self.end)
    }
}
