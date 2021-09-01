use super::*;
use crate::{CSTNode, YggdrasilError};
use lsp_types::{Position as LspPosition, Range as LspRange, TextDocumentContentChangeEvent};

/// Defines operations to convert between native text types and [`lsp_types`].
/// The trait is automatically derived for any type that implements [`TextMap`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait LspTextAdaptor: TextAdaptor {
    /// Convert [`Position`] to [`LspPosition`]
    fn position_to_lsp_position(&self, pos: &Position) -> Option<LspPosition>;
    /// Convert [`LspPosition`] to [`Position`]
    fn lsp_position_to_position(&self, lsp_pos: &LspPosition) -> Option<Position>;
    /// Convert [`PositionRange`] to [`LspRange`]
    fn position_range_to_lsp_range(&self, range: &PositionRange) -> Option<LspRange>;
    /// Convert [`LspRange`] to [`PositionRange`]
    fn lsp_range_to_position_range(&self, lsp_range: &LspRange) -> Option<PositionRange>;
    /// Convert [`TextChange`] to [`TextDocumentContentChangeEvent`]
    fn change_to_lsp_change(&self, change: TextChange) -> Option<TextDocumentContentChangeEvent>;
    /// Convert [`TextDocumentContentChangeEvent`] to [`TextChange`]
    fn lsp_change_to_change(&self, lsp_change: TextDocumentContentChangeEvent) -> Option<TextChange>;
    /// Convert [`Offset`] to [`LspPosition`]
    fn offset_to_lsp_position(&self, offset: Offset) -> Option<LspPosition> {
        let p = self.offset_to_position(offset)?;
        self.position_to_lsp_position(&p)
    }
    /// Convert [`OffsetRange`] to [`LspRange`]
    fn offset_range_to_lsp_range(&self, range: &OffsetRange) -> Option<LspRange> {
        let p = self.offset_range_to_position_range(range)?;
        self.position_range_to_lsp_range(&p)
    }
    /// Convert [`OffsetRange`] to [`LspRange`]
    fn offset_pair_to_lsp_range(&self, start: usize, end: usize) -> Option<LspRange> {
        let p = self.offset_pair_to_position_range(start, end)?;
        self.position_range_to_lsp_range(&p)
    }
}

impl<T: TextAdaptor> LspTextAdaptor for T {
    fn position_to_lsp_position(&self, pos: &Position) -> Option<LspPosition> {
        let line_num = pos.line;
        let line_range = self.line_range(line_num)?;
        let line = self.sub_string(line_range)?;

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
        Some(LspPosition::new(line_num as u32, u16_offset as u32))
    }

    fn lsp_position_to_position(&self, lsp_pos: &LspPosition) -> Option<Position> {
        let line_range = self.line_range(lsp_pos.line)?;
        let line = self.sub_string(line_range)?;

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
        Some(Position::new(lsp_pos.line, u8_offset as u32))
    }

    fn position_range_to_lsp_range(&self, range: &PositionRange) -> Option<LspRange> {
        Some(LspRange::new(self.position_to_lsp_position(&range.start)?, self.position_to_lsp_position(&range.end)?))
    }

    fn lsp_range_to_position_range(&self, lsp_range: &LspRange) -> Option<PositionRange> {
        Some(self.lsp_position_to_position(&lsp_range.start)?..self.lsp_position_to_position(&lsp_range.end)?)
    }

    fn change_to_lsp_change(&self, change: TextChange) -> Option<TextDocumentContentChangeEvent> {
        if let Some(range) = change.range {
            let lsp_range = self.position_range_to_lsp_range(&range)?;
            Some(TextDocumentContentChangeEvent { range: Some(lsp_range), range_length: None, text: change.patch })
        }
        else {
            Some(TextDocumentContentChangeEvent { range: None, range_length: None, text: change.patch })
        }
    }

    fn lsp_change_to_change(&self, lsp_change: TextDocumentContentChangeEvent) -> Option<TextChange> {
        if let Some(lsp_range) = lsp_change.range {
            let range = self.lsp_range_to_position_range(&lsp_range)?;
            Some(TextChange { range: Some(range), patch: lsp_change.text })
        }
        else {
            Some(TextChange { range: None, patch: lsp_change.text })
        }
    }
}

impl TextIndex {
    /// Apply Edit event from LSP
    pub fn apply_lsp_change(&mut self, lsp_change: TextDocumentContentChangeEvent) -> Result<()> {
        match self.lsp_change_to_change(lsp_change) {
            Some(s) => self.apply_change(s),
            None => Err(YggdrasilError::unexpected_token("TODO: apply_lsp_change")),
        }
    }
}

impl<R> CSTNode<R> {
    /// Get [`LspRange`] with text index info
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> Option<LspRange> {
        let p = text.offset_range_to_position_range(&self.range)?;
        text.position_range_to_lsp_range(&p)
    }
    /// Get start [`LspPosition`] with text index info
    #[inline]
    pub fn get_lsp_start(&self, text: &TextIndex) -> Option<LspPosition> {
        let p = text.offset_to_position(self.range.start)?;
        text.position_to_lsp_position(&p)
    }
    /// Get end [`LspPosition`] with text index info
    #[inline]
    pub fn get_lsp_end(&self, text: &TextIndex) -> Option<LspPosition> {
        let p = text.offset_to_position(self.range.end)?;
        text.position_to_lsp_position(&p)
    }
}
