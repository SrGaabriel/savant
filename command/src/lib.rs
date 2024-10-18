pub struct Command {
    pub name: String,
    pub subcommands: Vec<Command>,
    pub arguments: Vec<Argument>
}

pub struct Argument {
    pub short: Option<String>,
    pub long: String,
    pub value_type: ArgumentType
}

pub enum ArgumentType {
    String,
    Boolean,
    Int
}