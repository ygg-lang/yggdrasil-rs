mod ast;
mod cst;
mod init;
mod new;
mod publish;
mod test;

use self::{ast::CommandAST, cst::CommandCST, init::CommandInit, new::CommandNew, publish::CommandPub, test::CommandTest};
use crate::Options;
use anyhow::Result;
use clap::Parser;
use std::{env, fs, path::Path, process::Command};

#[derive(Parser)]
pub enum SubCommand {
    Init(CommandInit),
    New(CommandNew),
    CST(CommandCST),
    AST(CommandAST),
    Test(CommandTest),
    Pub(CommandPub),
}

impl SubCommand {
    pub fn run(&self, _: &Options) -> Result<()> {
        Self::calibrate_current_dir()?;
        match self {
            SubCommand::Init(cmd) => cmd.run(),
            SubCommand::New(cmd) => cmd.run(),
            SubCommand::CST(cmd) => cmd.run(),
            SubCommand::AST(cmd) => cmd.run(),
            SubCommand::Test(cmd) => cmd.run(),
            SubCommand::Pub(cmd) => cmd.run(),
        }
    }
    pub fn calibrate_current_dir() -> Result<()> {
        let curr = env::current_dir()?;
        env::set_current_dir(curr)?;
        Ok(())
    }
}
