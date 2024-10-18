use std::io::{Write, self};

use clap::{Command, CommandFactory};
use clap_complete::{Generator, Shell};

use crate::parser::Cli;

pub fn cli_completions() {
    let mut app = Cli::command();
    let shell = Shell::Bash;
    write_completions(shell, &mut app, &mut io::stdout())
}

pub fn write_completions<G : Generator>(
    generator: G,
    command: &mut Command,
    mut buffer: &mut dyn Write
) {
    clap_complete::generate(
        generator,
        command,
        command.get_name().to_string(),
        &mut buffer
    );
}