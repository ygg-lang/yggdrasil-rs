pub(crate) mod cst;
pub(crate) mod line_column;
pub(crate) mod lsp;
pub(crate) mod text_change;
pub(crate) mod text_indexed;

#[cfg(test)]
mod test;

use crate::CSTNode;
use std::ops::Range;
use text_indexed::TextMap;

/// A combo of [`TextMap`] + [`TextAdapter`]. Wraps the original text and
/// provides all the conversion methods.
///
/// Generic over the type of the text it wraps. Can be used with e.g. `&str`,
/// `String`, or `Arc<str>`, depending on whether ownership is needed and if it
/// needs to be unique or shared.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TextIndex {
    /// The original text
    text: String,
    /// Range of start-end offsets for all lines in the `text`. [`u32`] should be
    /// enough for upto 4GB files; show me a source file like this!
    line_ranges: Vec<Range<u32>>,
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

/// Native representation of a change that replaces a part of the target text.
///
/// Can be converted to and from [`lsp_types::TextDocumentContentChangeEvent`] by
/// [`TextAdapter`].
pub struct TextChange {
    /// Specifies the part of the text that needs to be replaced. When `None` the
    /// whole text needs to be replaced.
    pub range: Option<Range<LineColumn>>,
    /// The replacement text.
    pub patch: String,
}
