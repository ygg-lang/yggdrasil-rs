#[cfg(test)]
mod test;
use lsp_document::{IndexedText, Pos, TextMap};


mod offset;

/// Cache all newlines
pub struct LineBreaks<'input> {
    inner: IndexedText<&'input str>,
}

impl<'i> LineBreaks<'i> {
    #[inline]
    pub fn new(input: &'i str) -> LineBreaks {
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

impl<'i> LineBreaks<'i> {
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        match self.inner.offset_to_pos(offset) {
            Some(s) => { (s.line, s.col) }
            None => { (0, 0) }
        }
    }
    #[inline]
    pub fn get_lsp_range(&self, start: usize, end: usize) -> Range {
        Range { start: self.get_lsp_start(start), end: self.get_lsp_end(end) }
    }
}

impl<'i> LineBreaks<'i> {

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
