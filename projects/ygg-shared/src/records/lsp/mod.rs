#[cfg(test)]
mod test;

use crate::records::{CSTNode, LineBreaks};
use lsp_types::{Position, Range};

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, lines: &LineBreaks<'_>) -> Range {
        lines.get_lsp_range(self.start, self.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &LineBreaks<'_>) -> Position {
        lines.get_lsp_position(self.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &LineBreaks<'_>) -> Position {
        lines.get_lsp_position(self.end)
    }
}

impl<'i> LineBreaks<'i> {
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> Range {
        Range { start: self.get_lsp_start(start), end: self.get_lsp_end(end) }
    }
    #[inline]
    pub fn get_lsp_start(&self, start: usize) -> Position {
        self.get_lsp_position(start)
    }
    #[inline]
    pub fn get_lsp_end(&self, end: usize) -> Position {
        self.get_lsp_position(end)
    }
    #[inline]
    fn get_lsp_position(&self, offset: usize) -> Position {
        if offset > self.get_text().len() {
            return Position { line: self.get_newlines().len() as u32, character: 0 };
        }
        let (line, column) = self.get_line_column(offset);
        let character = match self.get_nth_line(line) {
            Some(s) => unsafe { s.get_unchecked(0..column).encode_utf16().count() as u32 },
            None => column as u32,
        };
        Position { line: line as u32, character }
    }
}
