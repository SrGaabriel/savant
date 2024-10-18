use crate::parser::{Cli, Commands};

pub fn handle_command(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::ChangeDir { path } => {
                println!("Changing directory to {}", path);
            }
            Commands::Exit => {
                std::process::exit(0);
            }
            _ => {
                println!("Command {:?} not implemented", command);
            }
        }
    } else {
        println!("No command provided");
    }
}