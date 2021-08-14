use super::*;

#[test]
fn test_line() {
    let counter = LineBreaks::new(
        r"
1一二三四五
==========
==========
==========
5一二三四五
==========
==========
==========
10========
==========
==========
",
    );
    println!("{:?}", counter.get_newlines());
    assert_eq!(
        counter.get_lsp_range(0, 10),
        Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 5 } }
    );
    assert_eq!(
        counter.get_lsp_range(10, 20),
        Range { start: Position { line: 0, character: 5 }, end: Position { line: 1, character: 2 } }
    );
    assert_eq!(
        counter.get_lsp_range(100, 110),
        Range { start: Position { line: 8, character: 8 }, end: Position { line: 9, character: 8 } }
    );
}
