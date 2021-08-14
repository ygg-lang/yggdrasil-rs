use super::*;

/// Create a new grammar project
#[derive(Parser)]
pub struct CommandCST {
    /// Set the folder name of the project
    #[clap(default_value = "*")]
    grammar_name: Vec<String>,
}

impl CommandCST {
    pub fn run(&self) -> Result<()> {
        if self.grammar_name.iter().next().filter(|s| s.as_str() != "*").is_none() {
            return self.build_all();
        }
        for name in &self.grammar_name {
            self.build_one(name)?
        }
        Ok(())
    }
    pub fn build_one(&self, _name: &str) -> Result<()> {
        unimplemented!()
    }
    pub fn build_all(&self) -> Result<()> {
        unimplemented!()
    }
}
