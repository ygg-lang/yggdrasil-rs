mod offset;

/// Cache all newlines
pub struct LineBreaks<'input> {
    input: &'input str,
    lines: Vec<usize>,
}

impl<'i> LineBreaks<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { input, lines: Self::count_lines(input) }
    }
    pub fn update(&mut self, input: &'i str) {
        self.input = input;
        self.lines = Self::count_lines(input);
    }
    pub fn count_lines(input: &str) -> Vec<usize> {
        let mut counter = 0;
        let mut out = vec![counter];
        for line in input.lines() {
            // TODO: +2 if CRLF?
            counter += line.len() + 1;
            out.push(counter)
        }
        return out;
    }
    pub fn get_text(&self) -> &'i str {
        self.input
    }
    pub fn get_newlines(&self) -> &[usize] {
        self.lines.as_slice()
    }
    pub fn get_nth_line(&self, line: usize) -> Option<&'_ str> {
        self.input.lines().nth(line)
    }
}

impl<'i> LineBreaks<'i> {
    pub fn get_line(&self, offset: usize) -> usize {
        let mut lower = 0;
        let mut upper = self.lines.len();
        while lower < upper {
            let mid = (lower + upper) / 2;
            unsafe {
                let line = *self.lines.get_unchecked(mid);
                // println!("@get_line {}:{}", line, offset);
                match line > offset {
                    true => upper = mid,
                    false => lower = mid + 1,
                }
            };
        }
        return lower;
    }
    pub fn get_line_column(&self, offset: usize) -> (usize, usize) {
        if offset > self.get_text().len() {
            return (self.get_newlines().len(), 0);
        }
        let line = self.get_line(offset);
        let line_break = unsafe { *self.lines.get_unchecked(line.saturating_sub(1)) };
        // println!("@get_column {}:{}", offset, line_break);
        return (line, offset.saturating_sub(line_break));
    }
}
