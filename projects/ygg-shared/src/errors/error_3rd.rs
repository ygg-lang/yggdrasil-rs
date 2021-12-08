use super::*;

impl From<url::ParseError> for YggdrasilError {
    fn from(_: url::ParseError) -> Self {
        Self { kind: IOError(std::io::Error::from_raw_os_error(10022)), file: None, range: None }
    }
}

impl From<ropey::Error> for YggdrasilError {
    fn from(e: ropey::Error) -> Self {
        Self::language_error(e.to_string())
    }
}
