use crate::errors::Error;

pub struct ASTBuilder {
    pub input: String,
    pub error: Vec<YggdrasilError>,
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self { input: "".to_string(), error: vec![] }
    }
}

impl ASTBuilder {}
