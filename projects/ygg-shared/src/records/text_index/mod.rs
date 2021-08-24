pub(crate) mod cst;
pub(crate) mod line_column;
pub(crate) mod lsp;
pub(crate) mod lsp_document;
pub(crate) mod traits;

#[cfg(test)]
mod test;

use crate::CSTNode;
use lsp_document::{IndexedText, TextAdapter, TextMap};
use std::ops::Range;

/// Cache all newlines
pub struct TextIndex {
    inner: IndexedText,
    /// Lines count
    lines: usize,
    /// Characters count
    count: usize,
}

/// Native position inside a text document/string. Points to a valid position
/// **before** the character inside a UTF8-encoded string.
///
/// ## Why use [`Pos`] instead of raw `usize` offset
///
/// This depends on the use-case. Often raw `usize` or a newtype wrapper around
/// `usize` is sufficient. However, raw byte offsets are not stable at all when a
/// text document changes.
///
/// Usually, a text document is an input to later stages of the pipelines. Let's
/// take a simple incremental pipeline:
/// ```text
/// text: string ->
///  symbols: Symbol { ..., start: usize, end: usize } ->
///  diagnostics: Diag { ..., start: usize, end: usize }
/// ```
///
/// Now, any change to `text` on line N will shift all `start` and `end` offsets,
/// which will invalidate all symbols and diagnostics following the change and
/// require recomputation.
///
/// However, if `start` and `end` are [`Pos`]es then only the line where the
/// change was made is affected. Symbols and diagnostic for other lines won't be
/// invalidated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineColumn {
    /// 0-indexed line inside the text document.
    pub line: u32,
    /// 0-indexed byte offset from the beginning of the.
    /// The offset is at a valid char boundary.
    pub column: u32,
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
            Some(s) => Range { start: (s.start.line, s.start.column), end: (s.end.line, s.end.column) },
            None => Range { start: self.get_line_column(start), end: self.get_line_column(end) },
        }
    }
    pub fn get_line_column(&self, offset: usize) -> (u32, u32) {
        match self.inner.offset_to_pos(offset) {
            Some(s) => (s.line, s.column),
            None => (self.lines as u32 + 1, 0),
        }
    }
}
