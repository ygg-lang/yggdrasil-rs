use super::*;

use std::{borrow::Borrow, cmp::Ordering, ops::Range};

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

/// Defines operations to convert between byte offsets and native [`Pos`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait TextMap {
    fn text(&self) -> &str;
    fn offset_to_pos(&self, offset: usize) -> Option<LineColumn>;
    fn offset_range_to_range(&self, offsets: Range<usize>) -> Option<Range<LineColumn>> {
        let start = self.offset_to_pos(offsets.start)?;
        let end = self.offset_to_pos(offsets.end)?;
        Some(start..end)
    }
    fn line_range(&self, line: u32) -> Option<Range<LineColumn>>;
    fn substr(&self, range: Range<LineColumn>) -> Option<&str>;
}

/// Defines operations to convert between native text types and [`lsp_types`].
/// The trait is automatically derived for any type that implements [`TextMap`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait TextAdapter {
    fn pos_to_lsp_pos(&self, pos: &LineColumn) -> Option<lsp_types::Position>;
    fn lsp_pos_to_pos(&self, lsp_pos: &lsp_types::Position) -> Option<LineColumn>;
    fn range_to_lsp_range(&self, range: &Range<LineColumn>) -> Option<lsp_types::Range>;
    fn lsp_range_to_range(&self, lsp_range: &lsp_types::Range) -> Option<Range<LineColumn>>;
    fn change_to_lsp_change(&self, change: TextChange) -> Option<lsp_types::TextDocumentContentChangeEvent>;
    fn lsp_change_to_change(&self, lsp_change: lsp_types::TextDocumentContentChangeEvent) -> Option<TextChange>;
}

impl<T: TextMap> TextAdapter for T {
    fn pos_to_lsp_pos(&self, pos: &LineColumn) -> Option<lsp_types::Position> {
        let line_num = pos.line;
        let line_range = self.line_range(line_num)?;
        let line = self.substr(line_range)?;

        let target_u8_offset = pos.column as usize;

        let mut u8_offset: usize = 0;
        let mut u16_offset: usize = 0;
        let mut found = false;

        for c in line.chars() {
            if u8_offset == target_u8_offset {
                found = true;
                break;
            }
            else {
                u8_offset += c.len_utf8();
                u16_offset += c.len_utf16();
            }
        }

        // Handle "append"/"after eol" case
        if !found && u8_offset == target_u8_offset {
            found = true;
        }

        assert!(found, "Offset not found in line");
        Some(lsp_types::Position::new(line_num as u32, u16_offset as u32))
    }

    fn lsp_pos_to_pos(&self, lsp_pos: &lsp_types::Position) -> Option<LineColumn> {
        let line_range = self.line_range(lsp_pos.line)?;
        let line = self.substr(line_range)?;

        let mut u8_offset: usize = 0;
        let mut u16_offset: usize = 0;
        let mut found = false;

        // Handle the case of artificial blank line
        if lsp_pos.character == 0 {
            found = true;
        }

        for c in line.chars() {
            if u16_offset == lsp_pos.character as usize {
                found = true;
                break;
            }
            else {
                u16_offset += c.len_utf16();
                u8_offset += c.len_utf8();
            }
        }

        // Handle "append" case
        if !found && u16_offset == lsp_pos.character as usize {
            found = true;
        }

        assert!(found, "LSP pos not found in line");
        Some(LineColumn::new(lsp_pos.line, u8_offset as u32))
    }

    fn range_to_lsp_range(&self, range: &Range<LineColumn>) -> Option<lsp_types::Range> {
        Some(lsp_types::Range::new(self.pos_to_lsp_pos(&range.start)?, self.pos_to_lsp_pos(&range.end)?))
    }

    fn lsp_range_to_range(&self, lsp_range: &lsp_types::Range) -> Option<Range<LineColumn>> {
        Some(self.lsp_pos_to_pos(&lsp_range.start)?..self.lsp_pos_to_pos(&lsp_range.end)?)
    }

    fn change_to_lsp_change(&self, change: TextChange) -> Option<lsp_types::TextDocumentContentChangeEvent> {
        if let Some(range) = change.range {
            let lsp_range = self.range_to_lsp_range(&range)?;
            Some(lsp_types::TextDocumentContentChangeEvent { range: Some(lsp_range), range_length: None, text: change.patch })
        }
        else {
            Some(lsp_types::TextDocumentContentChangeEvent { range: None, range_length: None, text: change.patch })
        }
    }

    fn lsp_change_to_change(&self, lsp_change: lsp_types::TextDocumentContentChangeEvent) -> Option<TextChange> {
        if let Some(lsp_range) = lsp_change.range {
            let range = self.lsp_range_to_range(&lsp_range)?;
            Some(TextChange { range: Some(range), patch: lsp_change.text })
        }
        else {
            Some(TextChange { range: None, patch: lsp_change.text })
        }
    }
}

/// A combo of [`TextMap`] + [`TextAdapter`]. Wraps the original text and
/// provides all the conversion methods.
///
/// Generic over the type of the text it wraps. Can be used with e.g. `&str`,
/// `String`, or `Arc<str>`, depending on whether ownership is needed and if it
/// needs to be unique or shared.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndexedText {
    /// The original text
    text: String,
    /// Range of start-end offsets for all lines in the `text`. [`u32`] should be
    /// enough for upto 4GB files; show me a source file like this!
    line_ranges: Vec<Range<u32>>,
}

impl IndexedText {
    pub fn new(text: String) -> Self {
        let mut line_ranges: Vec<Range<u32>> = Vec::new();

        let mut line_start: Option<usize> = None;
        let mut last_char: Option<(usize, char)> = None;

        let mut char_iter = text.char_indices().peekable();

        while let Some((pos, c)) = char_iter.next() {
            if line_start.is_none() {
                line_start = Some(pos);
            }
            last_char = Some((pos, c));

            let mut is_newline = false;

            if c == '\n' {
                is_newline = true;
            }
            else if c == '\r' {
                if char_iter.peek().filter(|(_, pc)| *pc == '\n').is_some() {
                    continue;
                }
                is_newline = true;
            }

            if is_newline {
                let start = line_start.expect("line_start should be always initialized");
                debug_assert!(text.is_char_boundary(start), "Start is not at char boundary");
                let end = pos + c.len_utf8();
                debug_assert!(text.is_char_boundary(end), "End is not at char boundary");

                line_ranges.push(start as u32..end as u32);
                line_start = None;
            }
        }

        // Handle a situation when there's no newline at the end
        if let (Some(start), Some((pos, c))) = (line_start, last_char) {
            line_ranges.push(start as u32..(pos + c.len_utf8()) as u32);
        }

        // Insert an artificial blank line with an empty range
        if let Some((pos, c)) = last_char {
            line_ranges.push((pos + c.len_utf8()) as u32..(pos + c.len_utf8()) as u32);
        }

        // Insert an artificial blank line for an empty string
        if text.is_empty() {
            line_ranges.push(0..0);
        }

        IndexedText { text, line_ranges }
    }

    fn offset_to_line(&self, offset: usize) -> Option<u32> {
        match offset.cmp(&self.text.len()) {
            Ordering::Greater => None,
            Ordering::Equal => Some((self.line_ranges.len().max(2) - 2) as u32),
            Ordering::Less => {
                let line = self.line_ranges.binary_search_by(|r| {
                    if offset < r.start as usize {
                        Ordering::Greater
                    }
                    else if offset >= r.end as usize {
                        Ordering::Less
                    }
                    else if offset >= r.start as usize && offset < r.end as usize {
                        Ordering::Equal
                    }
                    else {
                        panic!("Impossible case: offset={} and range={:?}", offset, r)
                    }
                });
                Some(line.unwrap_or_else(|_| panic!("Couldn't translate u8 offset {} to line", offset)) as u32)
            }
        }
    }

    fn position_to_offset(&self, pos: &LineColumn) -> Option<usize> {
        let line_range = self.line_ranges.get(pos.line as usize)?;
        Some(line_range.start as usize + (pos.column as usize))
    }
}

impl TextMap for IndexedText {
    fn text(&self) -> &str {
        self.text.borrow()
    }

    fn offset_to_pos(&self, offset: usize) -> Option<LineColumn> {
        let line = self.offset_to_line(offset)?;
        let range = &self.line_ranges[line as usize];
        let char = offset - (range.start as usize);
        Some(LineColumn { line, column: char as u32 })
    }

    fn line_range(&self, line: u32) -> Option<Range<LineColumn>> {
        let offset = self.line_ranges.get(line as usize)?;
        Some(LineColumn::new(line, 0)..LineColumn::new(line, offset.end - offset.start))
    }

    fn substr(&self, range: Range<LineColumn>) -> Option<&str> {
        let start_line = self.line_ranges.get(range.start.line as usize)?;
        let end_line = self.line_ranges.get(range.end.line as usize)?;
        let start_offset = start_line.start + range.start.column;
        let end_offset = end_line.start + range.end.column;

        Some(&self.text()[start_offset as usize..end_offset as usize])
    }
}

/// Applies a [`TextChange`] to [`IndexedText`] returning a new text as [`String`].
pub fn apply_change(text: &IndexedText, change: TextChange) -> String {
    match change.range {
        None => change.patch,
        Some(range) => {
            let orig = text.text();

            let offset_start = text.position_to_offset(&range.start).unwrap();
            let offset_end = text.position_to_offset(&range.end).unwrap();
            debug_assert!(offset_start <= offset_end, "Expected start <= end, got {}..{}", offset_start, offset_end);
            debug_assert!(offset_end <= orig.len(), "Expected end <= text.len(), got {} > {}", offset_end, orig.len());

            let mut new = orig.to_string();

            if offset_start == text.text().len() {
                new.push_str(&change.patch);
            }
            else {
                new.replace_range(offset_start..offset_end, &change.patch)
            }
            new
        }
    }
}
