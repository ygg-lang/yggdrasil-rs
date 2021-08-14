use super::*;

/// A subcommand for controlling testing
#[derive(Parser)]
pub struct CommandTest {
    /// Print debug info
    #[clap(short)]
    debug: bool,
    ///
    grammar: String,
    ///
    tests: String,
}

impl CommandTest {
    pub fn run(&self) -> Result<()> {
        println!("ygg ast does not been unimplemented!");
        Ok(())
    }
}
