use super::*;

#[derive(Clone, Debug)]
pub enum FileType {
    GrammarString(String),
    Grammar(GrammarInfo),
    TypeString(String),
    Type(GrammarType),
}

impl FileType {
    pub fn parse_toml(&mut self) -> Result<FileType> {
        unimplemented!()
    }
    pub async fn parse_ygg(&mut self, url: Url) -> Result<GrammarInfo> {
        match self {
            FileType::Grammar(g) => {
                // TODO: no clone
                Ok(g.clone())
            }
            FileType::GrammarString(s) => {
                let mut ctx = GrammarContext::new(s, &url);
                let mut hints = HintItems::default();
                let program = {
                    let mut parser = PARSER_MANAGER.write()?;
                    let program = parser.parse_program(s)?;
                    parse_error_to_hints(&ctx, parser.errors(), &mut hints);
                    program
                };
                let mut grammar = program.translate(&mut ctx)?;
                hints += ctx.get_hints().to_owned();
                hints += grammar.optimize().await?;
                hints += grammar.report_meta();
                *self = Self::Grammar(grammar.to_owned());
                HINT_MANAGER.set(url, hints);
                Ok(grammar)
            }
            _ => Err(YggdrasilError::language_error("Not a grammar file")),
        }
    }
}

fn parse_error_to_hints(file: &GrammarContext, es: &[YggdrasilError], hint: &mut HintItems) {
    for e in es {
        let diag = match e {
            YggdrasilError::StructureError { error, start, end } => {
                let range = match (start, end) {
                    (Some(s), Some(e)) => file.get_lsp_range((*s, *e)),
                    _ => (Default::default()),
                };
                Diagnostic {
                    range,
                    severity: None,
                    code: None,
                    code_description: None,
                    source: None,
                    message: error.to_owned(),
                    related_information: None,
                    tags: None,
                    data: None,
                }
            }
            _ => unreachable!(),
        };
        hint.diagnostic.push(diag)
    }
}
