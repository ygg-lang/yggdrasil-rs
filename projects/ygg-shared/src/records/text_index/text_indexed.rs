use super::*;

impl TextIndex {
    pub fn new(text: &str) -> Self {
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
        // count chars in O(n)
        let text = Rope::from_str(&text);
        Self { text, line_ranges }
    }

    pub fn offset_to_line(&self, offset: usize) -> Option<u32> {
        match offset.cmp(&self.count_bytes()) {
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
}

impl TextIndex {
    #[inline]
    pub fn apply_change_full(&mut self, input: &str) {
        *self = Self::new(input)
    }
    #[inline]
    pub fn get_nth_line(&self, line: usize) -> Option<&'_ str> {
        self.text.lines().nth(line).and_then(|f| f.as_str())
    }
    /// Applies a [`TextChange`] to [`IndexedText`] returning a new text as [`String`].
    pub fn apply_change(&mut self, change: TextChange) -> Result<()> {
        match change.range {
            None => self.apply_change_full(&change.patch),
            Some(range) => {
                let orig = self.text();
                let offset_start = range.start.as_offset(self).unwrap();
                let offset_end = range.end.as_offset(self).unwrap();
                debug_assert!(offset_start <= offset_end, "Expected start <= end, got {}..{}", offset_start, offset_end);
                debug_assert!(offset_end <= orig.len(), "Expected end <= text.len(), got {} > {}", offset_end, orig.len());
                self.text.remove(offset_start..offset_end);
                self.text.insert(offset_start, &change.patch)
            }
        }
        Ok(())
    }
}

impl TextMap for TextIndex {
    fn text(&self) -> String {
        self.text.to_string()
    }

    /// Total number of bytes in the [`TextIndex`].
    ///
    /// Runs in O(1) time.
    #[inline]
    fn count_bytes(&self) -> usize {
        self.text.len_bytes()
    }

    /// Total number of lines in the [`TextIndex`].
    ///
    /// Runs in O(1) time.
    #[inline]
    fn count_lines(&self) -> usize {
        self.text.len_lines()
    }
    /// Total number of chars in the [`TextIndex`].
    ///
    /// Runs in O(1) time.
    #[inline]
    fn count_chars(&self) -> usize {
        self.text.len_chars()
    }

    fn offset_to_position(&self, offset: usize) -> Option<LineColumn> {
        let line = self.offset_to_line(offset)?;
        let range = &self.line_ranges[line as usize];
        let char = offset - (range.start as usize);
        Some(LineColumn { line, column: char as u32 })
    }

    fn line_range(&self, line: u32) -> Option<Range<LineColumn>> {
        let offset = self.line_ranges.get(line as usize)?;
        Some(LineColumn::new(line, 0)..LineColumn::new(line, offset.end - offset.start))
    }

    fn sub_string(&self, range: Range<LineColumn>) -> Option<&str> {
        let start_line = self.line_ranges.get(range.start.line as usize)?;
        let end_line = self.line_ranges.get(range.end.line as usize)?;
        let start_offset = start_line.start + range.start.column;
        let end_offset = end_line.start + range.end.column;
        let range = Range { start: start_offset as usize, end: end_offset as usize };
        self.text.slice(range).as_str()
    }
}
