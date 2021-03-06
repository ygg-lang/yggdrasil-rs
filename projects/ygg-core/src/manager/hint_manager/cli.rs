#[test]
fn main() {
    use ariadne::{sources, Color, Config, Fmt, Label, Report, ReportKind};
    let config = Config::default();
    // Generate some colours for each of our elements
    let a = Color::Yellow;
    let b = Color::Red;
    let c = Color::Green;
    Report::build(ReportKind::Error, "b.tao", 10)
        .with_code(3)
        .with_config(config)
        .with_message(format!("Cannot add types Nat and Str"))
        .with_label(Label::new(("b.tao", 10..14)).with_message(format!("This is of type {}", "Nat".fg(a))).with_color(a))
        .with_label(Label::new(("b.tao", 17..20)).with_message(format!("This is of type {}", "Str".fg(b))).with_color(b))
        .with_label(
            Label::new(("b.tao", 15..16))
                .with_message(format!(" {} and {} undergo addition here", "Nat".fg(a), "Str".fg(b)))
                .with_color(c)
                .with_order(10),
        )
        .with_label(
            Label::new(("a.tao", 4..8)).with_message(format!("Original definition of {} is here", "five".fg(a))).with_color(a),
        )
        .with_note(format!("{} is a number and can only be added to other numbers", "Nat".fg(a)))
        .finish()
        .print(sources(vec![("a.tao", include_str!("cli.rs")), ("b.tao", include_str!("item.rs"))]))
        .unwrap();
}
