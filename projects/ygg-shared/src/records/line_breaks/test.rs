use lsp_types::{Position, Range};
use crate::records::PositionSystem;

#[test]
fn test_line() {
    let counter = PositionSystem::new(include_str!("../line_breaks/lines.txt"));
    // println!("{:?}", counter.get_newlines());
    assert_eq!(
        counter.get_lsp_range(0, 10),
        Range { start: Position { line: 1, character: 0 }, end: Position { line: 1, character: 10 } }
    );
    assert_eq!(
        counter.get_lsp_range(10, 20),
        Range { start: Position { line: 1, character: 10 }, end: Position { line: 2, character: 3 } }
    );
    assert_eq!(
        counter.get_lsp_range(100, 110),
        Range { start: Position { line: 9, character: 0 }, end: Position { line: 9, character: 10 } }
    );
    assert_eq!(
        counter.get_lsp_range(200, 300),
        Range { start: Position { line: 12, character: 0 }, end: Position { line: 12, character: 0 } }
    );
}
