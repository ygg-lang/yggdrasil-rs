use super::*;

const UNARY: &str = r#"
test1 = ^e1;
test2 = e1?;
test3 = e1+;
test4 = e1*;
"#;

#[test]
fn test_unary() -> Result<()> {
    assert_ast(UNARY, include_str!("unary.yaml"))
}

const CONCAT_SIMPLE: &str = r#"
test1 = e1 ~ e2
test2 = e1 ~ e2 ~ e3 ~ e4
test3 = e1 ~ ((e2 ~ e3) ~ e4)
test4 = (e1 ~ e2) ~ (e3 ~ e4)
test5 = (e1 ~ e2) (e3 ~ e4)
test6 = (e1 e2) ~ (e3 e4)
"#;

#[test]
fn test_concat_simple() -> Result<()> {
    assert_ast(CONCAT_SIMPLE, include_str!("concat_simple.yaml"))
}

const OR_SIMPLE: &str = r#"
test1 = e1 | e2
test2 =
    | e1
    | e2
test3 = e1 | e2 | e3 | e4
test4 =
    | e1
    | ((e2 | e3) | e4);
test5 =
    | (e1 | e2)
    | (e3 | e4);
"#;

#[test]
fn test_or_simple() -> Result<()> {
    assert_ast(OR_SIMPLE, include_str!("or_simple.yaml"))
}

const OR_TAGGED: &str = r#"
test1 = e1 | e2 #B
test2 =
    | e1 #A
    | e2
test3 =
    | e1 #A
    | e2 #B
    | e3 #C
    | e4 #D
test4 =
    | e1 #A
    | ((e2 | e3) | e4) #B
test5 =
    | (e1 | e2) #A
    | (e3 | e4) #B
"#;

#[test]
fn test_or_tagged() -> Result<()> {
    assert_ast(OR_TAGGED, include_str!("or_tagged.yaml"))
}

const INCOMPLETE: &str = r#"
grammar! test1

grammar! test2 {"a"}

rule1 = a ~ b

rule2 =
"#;

#[test]
#[should_panic]
fn test_incomplete() {
    assert_ast(INCOMPLETE, include_str!("incomplete.yaml")).unwrap()
}

#[test]
fn test_bootstrap() -> Result<()> {
    assert_optimize(include_str!("../bootstrap.ygg"), include_str!("bootstrap.yaml"))
}
