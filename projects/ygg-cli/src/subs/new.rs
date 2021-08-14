use super::*;

/// Create a new grammar project
#[derive(Parser)]
pub struct CommandNew {
    /// Set the folder name of the project
    grammar_name: String,
    /// Overwrite the existing grammar project
    #[clap(short, long)]
    force: bool,
    /// Whether to generate boilerplate grammar
    #[clap(short, long)]
    placeholder: bool,
}

impl CommandNew {
    pub fn run(&self) -> Result<()> {
        let curr = env::current_dir()?;
        let dir_name = curr.join("projects").join(&self.grammar_name);
        if self.force {
            // Command::new("rm").args(["-rf", &dir_name]).spawn()?.wait()?;
            if let Ok(true) = fs::try_exists(&dir_name) {
                fs::remove_dir_all(&dir_name)?
            };
        }
        if self.placeholder {}
        Command::new("cd").args([&dir_name]).spawn()?.wait()?;
        Ok(())
    }
}
