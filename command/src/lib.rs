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
    pub long: String,
    pub value_type: ArgumentType
}

#[derive(Debug)]
pub enum ArgumentType {
    String,
    Boolean,
    Int
}

impl ArgumentType {
    pub fn name(&self) -> String {
        match self {
            Self::String => "String".to_string(),
            Self::Boolean => "bool".to_string(),
            Self::Int => "i32".to_string()
        }
    }
}


impl Argument {
    pub fn new(simple_name: String, value_type: ArgumentType) -> Self {
        Self {
            short: None,
            long: simple_name,
            value_type
        }
    }
}