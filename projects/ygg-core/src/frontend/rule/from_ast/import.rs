use super::*;

impl<'i> GrammarContext<'i> {
    pub(super) fn read_import(&mut self, call: ImportStatement) {
        let url = match Self::resolve_url(self.url, &call.path.data) {
            Ok(o) => o,
            Err(_) => {
                // TODO: report error
                return;
            }
        };
        let range = self.import.get(&url).map(|f| f.0.to_owned());
        if let Some(r) = range {
            self.must_not_duplicate("Import", format!("{:?} already imported", call.path), &call.range, &r)
        }
        self.import.insert(url, (call.range.to_owned(), call.symbol_alias.to_owned()));
    }
    fn resolve_url(url: &Url, path: &str) -> Result<Url> {
        match path {
            path if path.starts_with(">") => Ok(WORKSPACE_ROOT.read()?.join(&path[1..path.len()])?),
            path if path.starts_with("@") => Ok(GLOBAL_ROOT.read()?.join(&path[1..path.len()])?),
            _ => Ok(url.clone()),
        }
    }
}
