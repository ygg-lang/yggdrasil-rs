use super::*;
use crate::{CSTNode, YggdrasilError};
use lsp_types::{Position, Range, TextDocumentContentChangeEvent};

/// Defines operations to convert between native text types and [`lsp_types`].
/// The trait is automatically derived for any type that implements [`TextMap`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait LspTextAdapter: TextMap {
    fn position_to_lsp_position(&self, pos: &LineColumn) -> Option<Position>;
    fn lsp_position_to_position(&self, lsp_pos: &Position) -> Option<LineColumn>;
    fn position_range_to_lsp_range(&self, range: &std::ops::Range<LineColumn>) -> Option<Range>;
    fn lsp_range_to_position_range(&self, lsp_range: &Range) -> Option<std::ops::Range<LineColumn>>;
    fn change_to_lsp_change(&self, change: TextChange) -> Option<TextDocumentContentChangeEvent>;
    fn lsp_change_to_change(&self, lsp_change: TextDocumentContentChangeEvent) -> Option<TextChange>;

    fn range_to_lsp_range(&self, range: &std::ops::Range<usize>) -> Option<Range>;
}

impl<T: TextMap> LspTextAdapter for T {
    fn position_to_lsp_position(&self, pos: &LineColumn) -> Option<Position> {
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
        Some(Position::new(line_num as u32, u16_offset as u32))
    }

    fn lsp_position_to_position(&self, lsp_pos: &Position) -> Option<LineColumn> {
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
        Some(LineColumn::new(lsp_pos.line, u8_offset as u32))
    }

    fn position_range_to_lsp_range(&self, range: &std::ops::Range<LineColumn>) -> Option<Range> {
        Some(Range::new(self.position_to_lsp_position(&range.start)?, self.position_to_lsp_position(&range.end)?))
    }

    fn lsp_range_to_position_range(&self, lsp_range: &Range) -> Option<std::ops::Range<LineColumn>> {
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
    pub fn apply_lsp_change(&mut self, lsp_change: TextDocumentContentChangeEvent) -> Result<()> {
        match self.lsp_change_to_change(lsp_change) {
            Some(s) => self.apply_change(s),
            None => Err(YggdrasilError::unexpected_token("TODO: apply_lsp_change")),
        }
    }
}

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> Option<Range> {
        let p = text.offset_range_to_position_range(self.range.clone())?;
        text.position_range_to_lsp_range(&p)
    }
    #[inline]
    pub fn get_lsp_start(&self, text: &TextIndex) -> Option<Position> {
        let p = text.offset_to_position(self.range.start)?;
        text.position_to_lsp_position(&p)
    }
    #[inline]
    pub fn get_lsp_end(&self, text: &TextIndex) -> Option<Position> {
        let p = text.offset_to_position(self.range.end)?;
        text.position_to_lsp_position(&p)
    }
}
