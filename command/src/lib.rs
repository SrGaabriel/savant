use crate::parser::Parseable;

pub mod parser;

pub struct Command {
    pub name: String,
    pub subcommands: Vec<Command>,
    pub arguments: Vec<Argument>
}

#[derive(Debug)]
pub struct Argument {
    pub short: Option<String>,
    pub long: String
}

impl Argument {
    pub fn new(simple_name: String) -> Self {
        Self {
            short: None,
            long: simple_name
        }
    }
}