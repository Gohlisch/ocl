use crate::lex::lex;
use crate::parse::RValueVariant::Identifier;
use crate::parse::Statement::RValue;
use crate::token::{KeywordVariation, LiteralVariation, OperatorVariation, Token};
use std::collections::VecDeque;
use std::ptr::null_mut;

pub struct AbstractSyntaxTree<'a> {
    pub root: &'a AbstractSyntaxTreeNode<'a>,
}

pub struct AbstractSyntaxTreeNode<'a> {
    parent: *mut AbstractSyntaxTreeNode<'a>,
    statement: Statement<'a>,
}

impl<'a> AbstractSyntaxTreeNode<'a> {
    fn new(statement: Statement<'a>) -> Self {
        AbstractSyntaxTreeNode {
            parent: null_mut(),
            statement,
        }
    }

    fn on_statement(self, run: fn(&Statement<'a>)) {
        run(&self.statement)
    }
}

pub enum Statement<'a> {
    UnaryOperation(OperatorVariation, *mut AbstractSyntaxTreeNode<'a>),
    BinaryOperation(
        *mut AbstractSyntaxTreeNode<'a>,
        OperatorVariation,
        *mut AbstractSyntaxTreeNode<'a>,
    ),
    RValue(RValueVariant<'a>),
}

pub enum RValueVariant<'a> {
    Literal(LiteralVariation<'a>),
    Identifier(&'a str),
    FunctionCall(ParameterList<'a>),
}

pub struct ParameterList<'a>(Vec<RValueVariant<'a>>);

impl<'a> AbstractSyntaxTree<'a> {
    pub fn from(input: &str) -> Self {
        let tokens = lex(input).expect(format!("'{input}' is not a valid ocl string.").as_str());
        let mut stack: VecDeque<AbstractSyntaxTreeNode> = VecDeque::from([]);

        let mut i = 0;
        while i < tokens.len() {
            let token = unsafe { tokens.get_unchecked(i) };

            match token {
                Token::Identifier(name) => match name {
                    _ => stack.push_back(AbstractSyntaxTreeNode {
                        parent: null_mut(),
                        statement: RValue(Identifier(name)),
                    }),
                },
                Token::Keyword(token) => todo!(),
                // Token::Keyword(token) => match token {
                //     KeywordVariation::And => {}
                //     KeywordVariation::Body => {}
                //     KeywordVariation::Context => {}
                //     KeywordVariation::Def => {}
                //     KeywordVariation::Derive => {}
                //     KeywordVariation::Else => {}
                //     KeywordVariation::Endif => {}
                //     KeywordVariation::Endpackage => {}
                //     KeywordVariation::False => {}
                //     KeywordVariation::If => {}
                //     KeywordVariation::Implies => {}
                //     KeywordVariation::In => {}
                //     KeywordVariation::Init => {}
                //     KeywordVariation::Inv => {}
                //     KeywordVariation::Invalid => {}
                //     KeywordVariation::Let => {}
                //     KeywordVariation::Not => {}
                //     KeywordVariation::Null => {}
                //     KeywordVariation::Or => {}
                //     KeywordVariation::Package => {}
                //     KeywordVariation::Post => {}
                //     KeywordVariation::Pre => {}
                //     KeywordVariation::SelfReference => {}
                //     KeywordVariation::Static => {}
                //     KeywordVariation::Then => {}
                //     KeywordVariation::True => {}
                //     KeywordVariation::Xor => {}
                // },
                Token::Operator(token) => match token {
                    OperatorVariation::Point => {
                        todo!()
                        // let next_token = tokens
                        //     .get(i + 1)
                        //     .expect("Expected an identifier after the navigation operator '.', but nothing was given.");

                        // match next_token {
                        //     Token::Identifier(identifier) => match stack.back().expect("The point operator '.' can only be used on a litaral, identifier or the result of another operation.") {
                        //                         Statement::UnaryOperation(operator, operand) => {
                        //                             let unary_operator_node = &mut AbstractSyntaxTreeNode{
                        //                                 parent: null_mut(),
                        //                                 statement: RValue(RValueVariant::Identifier(identifier))
                        //                             };

                        //                             let navigation_operation_node = &mut AbstractSyntaxTreeNode{
                        //                                 parent: null_mut(),
                        //                                 statement: Statement::BinaryOperation(*operand, *token, unary_operator_node),
                        //                             };
                        //                             stack.push_back()
                        //                         },
                        //                         Statement::BinaryOperation(_, _, _) => todo!(),
                        //                         RValue(_) => todo!(),
                        //                     },
                        //                     token => panic!("Expected an identifier after the navigation operator '.', but found {token:?} instead.")
                        // }
                    }
                    OperatorVariation::DoublePoint => {}
                    OperatorVariation::Arrow => {}
                    OperatorVariation::Star => {}
                    OperatorVariation::Plus => {}
                    OperatorVariation::Minus => {}
                    OperatorVariation::Slash => {}
                    OperatorVariation::LessThan => {}
                    OperatorVariation::GreaterThan => {}
                    OperatorVariation::LessThanEqual => {}
                    OperatorVariation::GreaterThanEqual => {}
                    OperatorVariation::Equals => {}
                    OperatorVariation::Unequals => {}
                    OperatorVariation::Colon => {}
                    OperatorVariation::DoubleColon => {}
                    OperatorVariation::Comma => {}
                    OperatorVariation::OpeningRoundBracket => {}
                    OperatorVariation::ClosingRoundBracket => {}
                    OperatorVariation::OpeningCurlyBracket => {}
                    OperatorVariation::ClosingCurlyBracket => {}
                    OperatorVariation::OpeningSquareBracket => {}
                    OperatorVariation::ClosingSquareBracket => {}
                    OperatorVariation::Pipe => {}
                    OperatorVariation::AtPre => {}
                },
                Token::Literal(token) => stack.push_back(AbstractSyntaxTreeNode {
                    parent: null_mut(),
                    statement: RValue(RValueVariant::Literal(*token)),
                }),
            }
        }

        AbstractSyntaxTree {
            root: stack.back().expect("Stack sollte nicht leer sein."),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::parse::AbstractSyntaxTree;
    use crate::parse::RValueVariant;
    use crate::parse::Statement::*;
    use crate::parsing_error::ParsingError;
    use crate::token::LiteralVariation;

    #[test]
    fn parsed_string_literal() -> Result<(), ParsingError<'static>> {
        let simple_invariant = "'I am a String'";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant);

        let node = unsafe { &(&*tree.root).statement };

        assert!(matches!(
            node,
            RValue(RValueVariant::Literal(LiteralVariation::String(
                "'I am a String'"
            )))
        ));

        Ok(())
    }
    // #[test]
    // fn parse_simple_invariant() -> Result<(), ParsingError<'static>> {
    //     let simple_invariant = "self.numberOfEmployees > 50";

    //     let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant);

    //     let node = tree.root;
    //     assert!(matches!(node.statement, BinaryOperation(_, _, _)));

    //     Ok(())
    // }

    // #[test]
    // fn parse_simple_invariant_2() -> Result<(), ParsingError<'static>> {
    //     let simple_invariant = "10 + 20";

    //     let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant);

    //     let node = tree.root;
    //     assert!(matches!(node.statement, BinaryOperation(_, _, _)));

    //     Ok(())
    // }
}
