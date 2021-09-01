use super::*;

/// Defines operations to convert between byte offsets and native [`Pos`].
///
/// Most operations return an [`Option`] where [`None`] signals that the
/// conversion wasn't successful.
pub trait TextAdaptor {
    /// Count bytes in text
    /// TODO: return &str
    fn text(&self) -> String;
    /// Count bytes in text
    fn count_bytes(&self) -> usize;
    /// Count lines in text
    fn count_lines(&self) -> usize;
    /// Count characters in text
    fn count_chars(&self) -> usize;
    /// Convert [`Offset`] to [`Position`]
    fn offset_to_position(&self, offset: Offset) -> Option<Position>;
    fn line_range(&self, line: u32) -> Option<Range<Position>>;
    fn sub_string(&self, range: Range<Position>) -> Option<&str>;

    /// Convert [`OffsetRange`] to [`PositionRange`]
    fn offset_range_to_position_range(&self, offsets: &OffsetRange) -> Option<PositionRange> {
        let start = self.offset_to_position(offsets.start)?;
        let end = self.offset_to_position(offsets.end)?;
        Some(start..end)
    }
    /// Convert [`OffsetRange`] to [`PositionRange`]
    fn offset_pair_to_position_range(&self, start: usize, end: usize) -> Option<Range<Position>> {
        self.offset_range_to_position_range(&Range { start, end })
    }
}

impl Position {
    /// Create a new [`Pos`]. This method shouldn't be required to use most of
    /// the time!
    ///
    /// `line` is 0-indexed, `col` is a 0-indexed byte-offset from the beginning
    /// of the line to a **valid char position**.
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    /// Convert [`LineColumn`] to [`Offset`]
    pub fn as_offset(&self, text: &TextIndex) -> Option<Offset> {
        let line_range = text.line_ranges.get(self.line as usize)?;
        Some(line_range.start as usize + (self.column as usize))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        let line_cmp = self.line.cmp(&other.line);
        if line_cmp == Ordering::Equal { self.column.cmp(&other.column) } else { line_cmp }
    }
}
