use crate::lex::lex;
use crate::parse::RValueVariant::Identifier;
use crate::parse::Statement::{BinaryOperation, RValue};
use crate::token::{KeywordVariation, LiteralVariation, OperatorVariation, Token};
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::ptr::null_mut;

type SyntaxError = String;

pub struct AbstractSyntaxTree<'a> {
    pub root: AbstractSyntaxTreeNode<'a>,
}

pub struct AbstractSyntaxTreeNode<'a> {
    statement: Statement<'a>,
}

impl<'a> AbstractSyntaxTreeNode<'a> {
    fn new(statement: Statement<'a>) -> Self {
        AbstractSyntaxTreeNode {
            statement,
        }
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
    pub fn from(input: &'a str) -> Result<Self, SyntaxError> {
        let tokens = lex(input).expect(format!("'{input}' is not a valid ocl string.").as_str());
        let mut stack: VecDeque<AbstractSyntaxTreeNode> = VecDeque::from([]);

        let mut i = 0;
        while i < tokens.len() {
            let token = unsafe { tokens.get_unchecked(i) };

            match token {
                Token::Identifier(name) => match name {
                    _ => stack.push_back(AbstractSyntaxTreeNode {
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
                    OperatorVariation::Plus => {
                        push_binary_operation(&mut stack, OperatorVariation::Plus)?;
                    }
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
                Token::Literal(token) => push_rvalue(&mut stack, *token)?,
            }

            i += 1;
        }

        Ok(AbstractSyntaxTree {
            root: stack.pop_back().expect("Stack sollte nicht leer sein."),
        })
    }

}

fn push_binary_operation(
    stack: &mut VecDeque<AbstractSyntaxTreeNode>,
    variation: OperatorVariation,
) -> Result<(), SyntaxError> {
    if let Some(mut left_hand_operand) = stack.pop_back() {
        let operation = AbstractSyntaxTreeNode::new(Statement::BinaryOperation(
            &mut left_hand_operand,
            variation,
            null_mut(),
        ));
        stack.push_back(operation);
        Ok(())
    } else {
        Err("Excpected left hand operand but found nothing.".to_string())
    }
}


fn push_rvalue<'a>(stack: & mut VecDeque<AbstractSyntaxTreeNode<'a>>, token: LiteralVariation<'a>) -> Result<(), SyntaxError> {
    if let Some(mut node) = stack.back_mut() {
        match node.statement {
            BinaryOperation(lhs, op, right_hand_value) => {
                if right_hand_value.is_null() {
                    node.statement = BinaryOperation(lhs, op, &mut AbstractSyntaxTreeNode::new(RValue(RValueVariant::Literal(token))));
                    Ok(())
                }
                else { Err("A RValue cannot be applied to another RValue. TODO: Bessere Beschreibung.".to_string()) }
            }
            _ => Err("A RValue cannot be applied to another RValue. TODO: Bessere Beschreibung.".to_string())
        }
    } else {
        stack.push_back(AbstractSyntaxTreeNode {
            statement: RValue(RValueVariant::Literal(token)),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::parse::AbstractSyntaxTree;
    use crate::parse::RValueVariant;
    use crate::parse::Statement::*;
    use crate::parsing_error::ParsingError;
    use crate::token::LiteralVariation;

    use super::SyntaxError;

    #[test]
    fn parsed_string_literal() -> Result<(), SyntaxError> {
        let simple_invariant = "\"I am a String\"";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        let node = tree.root.statement;

        assert!(matches!(
            node,
            RValue(RValueVariant::Literal(LiteralVariation::String(
                simple_invariant
            )))
        ));

        Ok(())
    }

    #[test]
    fn parse_int_literal() -> Result<(), SyntaxError> {
        let simple_invariant = "42";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        let node = tree.root.statement;

        assert!(matches!(
            node,
            RValue(RValueVariant::Literal(LiteralVariation::Integer(42)))
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

    #[test]
    fn parse_plus() -> Result<(), SyntaxError> {
        let simple_invariant = "10 + 20";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        let node = tree.root;
        assert!(matches!(node.statement, BinaryOperation(_, _, _)));

        Ok(())
    }
}
