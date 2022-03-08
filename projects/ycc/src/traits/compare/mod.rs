/// Indicates whether a comparison was successful, an error, or
/// if more data was needed
#[derive(Debug, PartialEq)]
pub enum CompareResult {
    /// Comparison was successful
    Ok,
    /// We need more data to be sure
    Incomplete,
    /// Comparison failed
    Error,
}

/// Abstracts comparison operations
pub trait Compare<T> {
    /// Compares self to another value for equality
    fn compare(&self, t: T) -> CompareResult;
    /// Compares self to another value for equality
    /// independently of the case.
    ///
    /// Warning: for `&str`, the comparison is done
    /// by lowercasing both strings and comparing
    /// the result. This is a temporary solution until
    /// a better one appears
    fn compare_no_case(&self, t: T) -> CompareResult;
}
