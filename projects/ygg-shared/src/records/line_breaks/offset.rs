

use crate::records::{CSTNode, LineBreaks};
use lsp_types::{Position, Range};




use lsp_document::{IndexedText, TextAdapter, TextMap};

impl<R> CSTNode<R> {
    #[inline]
    pub fn get_lsp_range(&self, lines: &LineBreaks<'_>) -> Range {
        lines.get_lsp_range(self.start, self.end)
    }
    #[inline]
    pub fn get_lsp_start(&self, lines: &LineBreaks<'_>) -> Position {
        lines.get_lsp_position(self.start)
    }
    #[inline]
    pub fn get_lsp_end(&self, lines: &LineBreaks<'_>) -> Position {
        lines.get_lsp_position(self.end)
    }
}


#[test]
fn test() {
    use lsp_document::{IndexedText, Pos, TextAdapter, TextMap};
    use lsp_types::Position;

    // Character width
    // U16:     1111111111111 1111111111 1 11 1 1 111111111 21
    // U8:      1111111111111 1222122221 1 13 3 3 111111111 41
    // U8 offset
    //          0         1       2      3       4          5
    //          0123456789012 3468013579 0 12 5 8 123456789 04
    let text = "Hello, world!\nКак дела?\r\n做得好\nThis is 💣!";
    let text = IndexedText::new(text);
    // Examples of using TextMap methods
    //
    // Pos of 💣 from its offset

    assert_eq!(text.offset_to_pos(50).unwrap(), Pos::new(3, 8));
    // Raw line range info
    assert_eq!(text.line_range(2).unwrap(), Pos::new(2, 0)..Pos::new(2, 10));
    // Extracting part of text between two positions
    assert_eq!(text.substr(Pos::new(1, 7)..Pos::new(1, 15)).unwrap(), "дела");

    // Example of using TextAdapter methods
    //
    // Pos of `!` after 💣
    assert_eq!(text.lsp_pos_to_pos(&Position::new(3, 10)).unwrap(), Pos::new(3, 12));
    assert_eq!(text.pos_to_lsp_pos(&Pos::new(3, 12)).unwrap(), Position::new(3, 10));
}
