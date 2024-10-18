use std::any::Any;
use clap::{Parser, Subcommand, CommandFactory, Command};
use clap_complete::Shell;

use crate::test::ExitCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub name: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "cd", about = "Change directory")]
    ChangeDir {
        path: String
    },
    Exit
}

pub fn parse_command(text: &String) -> Option<Cli> {
    let args = std::iter::once("savant").chain(text.split_whitespace()).collect::<Vec<_>>();

    Cli::try_parse_from(args)
        .ok()
}
