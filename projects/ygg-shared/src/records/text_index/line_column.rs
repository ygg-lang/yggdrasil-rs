use super::*;

/// Defines operations to convert between byte offsets and native [`Pos`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait TextMap {
    fn text(&self) -> String;
    fn count_bytes(&self) -> usize;
    fn count_lines(&self) -> usize;
    fn count_chars(&self) -> usize;
    fn offset_to_position(&self, offset: usize) -> Option<LineColumn>;
    fn offset_range_to_position_range(&self, offsets: Range<usize>) -> Option<Range<LineColumn>> {
        let start = self.offset_to_position(offsets.start)?;
        let end = self.offset_to_position(offsets.end)?;
        Some(start..end)
    }
    fn line_range(&self, line: u32) -> Option<Range<LineColumn>>;
    fn sub_string(&self, range: Range<LineColumn>) -> Option<&str>;
}

impl LineColumn {
    /// Create a new [`Pos`]. This method shouldn't be required to use most of
    /// the time!
    ///
    /// `line` is 0-indexed, `col` is a 0-indexed byte-offset from the beginning
    /// of the line to a **valid char position**.
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn as_offset(&self, text: &TextIndex) -> Option<usize> {
        let line_range = text.line_ranges.get(self.line as usize)?;
        Some(line_range.start as usize + (self.column as usize))
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LineColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        let line_cmp = self.line.cmp(&other.line);
        if line_cmp == Ordering::Equal { self.column.cmp(&other.column) } else { line_cmp }
    }
}
