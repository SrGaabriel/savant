use savant_macros::{command_declaration, argument, subcommand};



#[command_declaration]
pub struct ExitCommand {
    test: String,
    name: bool,
    hello: i32,
    sub: Subcommands
}

#[subcommand]
pub enum Subcommands {
    Test {
        exit: String
    }
}

impl ExitCommand {
    pub fn nothing() {}
}