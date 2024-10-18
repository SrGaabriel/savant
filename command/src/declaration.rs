use crate::Argument;

pub trait CommandDeclaration {
    fn arguments(&self) -> Vec<Argument>;
}