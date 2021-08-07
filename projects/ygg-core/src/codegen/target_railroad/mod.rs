use railroad::{self, svg, Diagram, End, NonTerminal, Sequence, Start, Terminal, DEFAULT_CSS};

fn test() {
    let mut seq = Sequence::default();
    seq.push(Box::new(Start))
        .push(Box::new(Terminal::new("BEGIN".to_owned())))
        .push(Box::new(NonTerminal::new("syntax".to_owned())))
        .push(Box::new(End));

    let mut dia = Diagram::new(seq);

    dia.add_element(svg::Element::new("style").set("type", "text/css").text(DEFAULT_CSS));

    println!("{}", dia);
}