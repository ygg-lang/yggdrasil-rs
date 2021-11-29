use super::*;

#[test]
fn test_duplicate1() -> Result<()> {
    assert_diagnostic(include_str!("duplicate1.ygg"), include_str!("duplicate1.yaml"))
}

#[test]
fn test_duplicate2() -> Result<()> {
    assert_diagnostic(include_str!("duplicate2.ygg"), include_str!("duplicate2.yaml"))
}

#[test]
fn test_bootstrap() -> Result<()> {
    assert_diagnostic(include_str!("../bootstrap.ygg"), include_str!("bootstrap.yaml"))
}
