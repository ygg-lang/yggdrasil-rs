use lsp_types::DiagnosticSeverity;
use crate::DiagnosticLevel;


impl DiagnosticLevel {
    /// Convert to lsp [`DiagnosticSeverity`]
    pub fn into_severity(self) -> Option<DiagnosticSeverity> {
        match self {
            Self::None => None,
            Self::Error => Some(DiagnosticSeverity::ERROR),
            Self::Warning => Some(DiagnosticSeverity::WARNING),
            Self::Information => Some(DiagnosticSeverity::INFORMATION),
            Self::Hint => Some(DiagnosticSeverity::HINT),
        }
    }
}