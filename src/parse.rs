use crate::token::{LiteralVariation, OperatorVariation};

pub struct AbstractSyntaxTree<'a> {
    pub root: Box<AbstractSyntaxTreeNode<'a>>
}

pub enum AbstractSyntaxTreeNode<'a> {
    Invariant(Statement<'a>),
}

pub enum Statement<'a> {
    UnaryOperation(OperatorVariation, Box<Statement<'a>>),
    BinaryOperation(Box<Statement<'a>>, OperatorVariation, Box<Statement<'a>>),
    RValue(RValueVariant<'a>)
}

pub enum RValueVariant<'a> {
    Literal(LiteralVariation<'a>),
    Identifier(&'a str),
    FunctionCall(ParameterList<'a>)
}

pub struct ParameterList<'a> (
    Vec<RValueVariant<'a>>
);


impl<'a> AbstractSyntaxTree<'a> {
    pub fn from(input: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::parse::AbstractSyntaxTree;
    use crate::parsing_error::ParsingError;
    use crate::parse::AbstractSyntaxTreeNode::{*};
    use crate::parse::Statement::{*};

    #[test]
    fn parse_simple_invariant() -> Result<(), ParsingError<'static>> {
        let simple_invariant = "self.numberOfEmployees > 50";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant);

        let node = tree.root.deref();
        assert!(matches!(node, Invariant(BinaryOperation(_, _, _))));


        Ok(())
    }
}
