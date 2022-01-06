/// DiagnosticLevel
#[derive(Debug, Copy, Clone)]
pub enum DiagnosticLevel {
    /// No special diagnostic
    None = 0,
    /// Error Message, red
    Error = 1,
    /// Warning Message, yellow
    Warning = 2,
    /// Notice Message, magenta
    Information = 3,
    /// Hint Message, dots
    Hint = 4,
}

