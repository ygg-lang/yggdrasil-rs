use crate::records::TextIndex;
use lsp_types::{Position, Range};

#[test]
fn test_line() {
    let counter = TextIndex::new(include_str!("lines.txt"));
    // println!("{:?}", counter.get_newlines());
    assert_eq!(
        counter.get_lsp_range(0, 10),
        Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 4 } }
    );
    assert_eq!(
        counter.get_lsp_range(10, 20),
        Range { start: Position { line: 0, character: 4 }, end: Position { line: 1, character: 3 } }
    );
    assert_eq!(
        counter.get_lsp_range(100, 110),
        Range { start: Position { line: 8, character: 0 }, end: Position { line: 8, character: 10 } }
    );
    assert_eq!(
        counter.get_lsp_range(200, 300),
        Range { start: Position { line: 10, character: 10 }, end: Position { line: 10, character: 10 } }
    );
}
