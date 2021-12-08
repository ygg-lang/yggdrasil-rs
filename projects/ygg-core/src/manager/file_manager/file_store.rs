use super::*;

#[derive(Clone, Debug)]
pub struct FileStore {
    pub fingerprint: u128,
    pub data: FileType,
}

impl FileStore {
    pub fn new_grammar(key: u128, value: String) -> Self {
        Self { fingerprint: key, data: FileType::GrammarString(value) }
    }
    pub fn new_type(key: u128, value: String) -> Self {
        Self { fingerprint: key, data: FileType::TypeString(value) }
    }
    pub fn load_url(url: &Url, f: FileFingerprint) -> Result<Self> {
        let FileFingerprint { fingerprint, text } = f;
        let path = url.to_file_path()?;
        let data = match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Ok(FileType::TypeString(text)),
            Some("ygg") | Some("yg") => Ok(FileType::GrammarString(text)),
            _ => Err(YggdrasilError::language_error("Unsupported file extension")),
        }?;
        Ok(Self { fingerprint, data })
    }
    pub async fn parse_ygg(&mut self, url: Url) -> Result<GrammarInfo> {
        self.data.parse_ygg(url).await
    }
    pub fn parse_toml(_input: String) -> Result<FileType> {
        unimplemented!()
    }
}
