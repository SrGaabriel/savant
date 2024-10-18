use savant_macros::{command_declaration, argument};

#[command_declaration]
pub struct ExitCommand {
    #[argument]
    test: String,
    name: bool,
    hello: i32
}