use super::*;
use crate::records::CSTNode;

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, lines: &PositionSystem<'_>) -> LSPRange {
        lines.get_lsp_range(self.start, self.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &PositionSystem<'_>) -> LSPPosition {
        lines.get_lsp_line_column(self.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &PositionSystem<'_>) -> LSPPosition {
        lines.get_lsp_line_column(self.end)
    }
}
