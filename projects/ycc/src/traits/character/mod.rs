pub trait Character {
    /// Tests if byte is ASCII alphabetic: A-Z, a-z
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_alphabetic;
    /// assert_eq!(is_alphabetic(b'9'), false);
    /// assert_eq!(is_alphabetic(b'a'), true);
    /// ```
    fn is_alphabetic(&self) -> bool;

    /// Tests if byte is ASCII digit: 0-9
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_digit;
    /// assert_eq!(is_digit(b'a'), false);
    /// assert_eq!(is_digit(b'9'), true);
    /// ```
    fn is_digit(&self) -> bool;

    /// Tests if byte is ASCII hex digit: 0-9, A-F, a-f
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_hex_digit;
    /// assert_eq!(is_hex_digit(b'a'), true);
    /// assert_eq!(is_hex_digit(b'9'), true);
    /// assert_eq!(is_hex_digit(b'A'), true);
    /// assert_eq!(is_hex_digit(b'x'), false);
    /// ```
    fn is_hex_digit(&self) -> bool;
    /// Tests if byte is ASCII octal digit: 0-7
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_oct_digit;
    /// assert_eq!(b'a'.is_oct_digit(), false);
    /// assert_eq!('a'.is_oct_digit(), false);
    /// assert_eq!('6'.is_oct_digit(), true);
    /// ```
    fn is_oct_digit(&self) -> bool;

    /// Tests if byte is ASCII alphanumeric: A-Z, a-z, 0-9
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_alphanumeric;
    /// assert_eq!(is_alphanumeric(b'-'), false);
    /// assert_eq!(is_alphanumeric(b'a'), true);
    /// assert_eq!(is_alphanumeric(b'9'), true);
    /// assert_eq!(is_alphanumeric(b'A'), true);
    /// ```
    #[inline]
    fn is_alpha_digit(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Tests if byte is ASCII space or tab
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_space;
    /// assert_eq!(is_space(b'\n'), false);
    /// assert_eq!(is_space(b'\r'), false);
    /// assert_eq!(is_space(b' '), true);
    /// assert_eq!(is_space(b'\t'), true);
    /// ```
    fn is_space(&self) -> bool;

    /// Tests if byte is ASCII newline: \n
    ///
    /// # Example
    ///
    /// ```
    /// # use nom::character::is_newline;
    /// assert_eq!(is_newline(b'\n'), true);
    /// assert_eq!(is_newline(b'\r'), false);
    /// assert_eq!(is_newline(b' '), false);
    /// assert_eq!(is_newline(b'\t'), false);
    /// ```
    fn is_newline(&self) -> bool;

    fn as_char(&self) -> char;

    fn length(&self) -> usize;
}

impl Character for u8 {
    #[inline]
    fn is_alphabetic(&self) -> bool {
        (0x41..=0x5A).contains(self) || (0x61..=0x7A).contains(self)
    }
    fn is_digit(&self) -> bool {
        (0x30..=0x39).contains(self)
    }
    fn is_hex_digit(&self) -> bool {
        (0x30..=0x39).contains(self) || (0x41..=0x46).contains(self) || (0x61..=0x66).contains(self)
    }
    #[inline]
    fn is_oct_digit(&self) -> bool {
        (0x30..=0x37).contains(self)
    }
    #[inline]
    fn is_space(&self) -> bool {
        [b' ', b'\t'].contains(self)
    }
    #[inline]
    fn is_newline(&self) -> bool {
        [b'\n', b'\r'].contains(self)
    }

    fn as_char(&self) -> char {
        *self as char
    }

    fn length(&self) -> usize {
        1
    }
}

impl Character for char {
    #[inline]
    fn is_alphabetic(&self) -> bool {
        char::is_alphabetic(*self)
    }
    #[inline]
    fn is_digit(&self) -> bool {
        char::is_digit(*self, 10)
    }
    #[inline]
    fn is_hex_digit(&self) -> bool {
        char::is_digit(*self, 16)
    }
    #[inline]
    fn is_oct_digit(&self) -> bool {
        char::is_digit(*self, 8)
    }
    #[inline]
    fn is_space(&self) -> bool {
        [' ', '\t'].contains(self)
    }
    #[inline]
    fn is_newline(&self) -> bool {
        ['\n', '\r'].contains(self)
    }

    fn as_char(&self) -> char {
        *self
    }

    fn length(&self) -> usize {
        self.len_utf8()
    }
}
