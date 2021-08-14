use super::*;

/// Initialize the project
#[derive(Parser)]
pub struct CommandInit {
    /// Overwrite the existing project
    #[clap(short, long)]
    force: bool,
    /// Set the folder name of the project
    dir_name: Option<String>,
}

impl CommandInit {
    pub fn run(&self) -> Result<()> {
        let dir_name = self.dir_name.clone().unwrap_or(String::from("project-yggdrasil-template"));
        if self.force {
            // Command::new("rm").args(["-rf", &dir_name]).spawn()?.wait()?;
            if let Ok(true) = fs::try_exists(&dir_name) {
                fs::remove_dir_all(&dir_name)?
            };
        }
        Command::new("git").args(["clone", "https://github.com/ygg-lang/project-yggdrasil-template.git", &dir_name]).spawn()?.wait()?;
        //let curr =  env::current_dir()?;
        //env::set_current_dir(&curr)?;
        //let curr = curr.join(&dir_name);
        Command::new("cd").args([&dir_name]).spawn()?.wait()?;
        Ok(())
    }
}
