use savant_macros::{command_declaration, argument};

#[command_declaration]
pub struct ExitCommand {
    test: String,
    name: bool,
    hello: i32
}

impl ExitCommand {
    pub fn nothing() {}
}