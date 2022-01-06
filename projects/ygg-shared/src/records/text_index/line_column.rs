use super::*;
use std::cmp::Ordering;

impl LineColumn {
    /// Create a new [`Pos`]. This method shouldn't be required to use most of
    /// the time!
    ///
    /// `line` is 0-indexed, `col` is a 0-indexed byte-offset from the beginning
    /// of the line to a **valid char position**.
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
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
