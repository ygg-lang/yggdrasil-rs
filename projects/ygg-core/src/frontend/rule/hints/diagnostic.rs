use super::*;

pub fn top_area_error(src: &str, msg: &str, range: &Range<usize>, file: &GrammarContext) -> Diagnostic {
    Diagnostic {
        range: file.get_lsp_range(range),
        severity: Some(DiagnosticSeverity::WARNING),
        code: None,
        code_description: None,
        source: Some(String::from(src)),
        message: String::from(msg),
        related_information: None,
        tags: None,
        data: None,
    }
}

#[rustfmt::skip]
pub fn duplicate_declaration_error(
    src: &str,
    msg: impl Into<String>,
    this: &Range<usize>,
    last: &Range<usize>,
    file: &GrammarContext
) -> Diagnostic {
    let info = DiagnosticRelatedInformation { location: Location { uri: file.get_url().to_owned(), range: file.get_lsp_range(last) }, message: msg.into() };
    Diagnostic {
        range: file.get_lsp_range(this),
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some(String::from(src)),
        message: String::from("Duplicate declaration detected"),
        related_information: Some(vec![info]),
        tags: None,
        data: None,
    }
}
