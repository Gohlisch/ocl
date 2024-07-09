use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::mem;

use crate::lex::lex;
use crate::parse::RValueVariant::Identifier;
use crate::parse::Statement::{BinaryOperation, RValue};
use crate::token::{LiteralVariation, OperatorVariation, Token};

type SyntaxError = String;


pub struct AbstractSyntaxTree<'a> {
    pub root: Option<Statement<'a>>,
}

pub enum Statement<'a> {
    UnaryOperation(OperatorVariation, Option<Box<Statement<'a>>>),
    BinaryOperation {
        lhs: Option<Box<Statement<'a>>>,
        operator: OperatorVariation,
        rhs: Option<Box<Statement<'a>>>,
    },
    RValue(RValueVariant<'a>),
}

impl<'a> Statement<'a> {
    fn is_int_literal(self: &Self, expected: i64) -> bool {
        match self {
            RValue(RValueVariant::Literal(LiteralVariation::Integer(actual))) => *actual == expected,
            _ => false
        }
    }

    fn is_string_literal(self: &Self, expected: &str) -> bool {
        match self {
            RValue(RValueVariant::Literal(LiteralVariation::String(actual))) => *actual == expected,
            _ => false
        }
    }
}

pub enum RValueVariant<'a> {
    Literal(LiteralVariation<'a>),
    Identifier(&'a str),
    FunctionCall(ParameterList<'a>),
}

pub struct ParameterList<'a>(Vec<RValueVariant<'a>>);

impl<'a> AbstractSyntaxTree<'a> {
    pub fn from(input: &'a str) -> Result<Self, SyntaxError> {
        let tokens = lex(input).unwrap_or_else(|_| panic!("'{input}' is not a valid ocl string."));
        let mut stack: VecDeque<Statement<'a>> = VecDeque::from([]);

        let mut i = 0;
        while i < tokens.len() {
            let token = tokens.get(i).expect("Popped more tokens than were given!");

            match token {
                Token::Identifier(name) => stack.push_back(RValue(Identifier(name))),
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
                        //                             let unary_operator_statement = &mut AbstractSyntaxTreeNode{
                        //                                 parent: null_mut(),
                        //                                 statement: RValue(RValueVariant::Identifier(identifier))
                        //                             };

                        //                             let navigation_operation_statement = &mut AbstractSyntaxTreeNode{
                        //                                 parent: null_mut(),
                        //                                 statement: Statement::BinaryOperation(*operand, *token, unary_operator_statement),
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
                    operator @ (OperatorVariation::Star
                    | OperatorVariation::Plus
                    | OperatorVariation::Minus
                    | OperatorVariation::Slash
                    | OperatorVariation::LessThan
                    | OperatorVariation::GreaterThan
                    | OperatorVariation::LessThanEqual
                    | OperatorVariation::GreaterThanEqual
                    | OperatorVariation::Equals
                    | OperatorVariation::Unequals)  => {
                        push_binary_operation(&mut stack, *operator)?;
                    }
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
            root: stack.pop_back(),
        })
    }

}

fn push_binary_operation(
    stack: &mut VecDeque<Statement>,
    variation: OperatorVariation,
) -> Result<(), SyntaxError> {
    if let Some(left_hand_operand) = stack.pop_back() {
        let operation = BinaryOperation{
            lhs: Some(Box::new(left_hand_operand)),
            operator: variation,
            rhs: None,
        };
        stack.push_back(operation);
        Ok(())
    } else {
        Err("Excpected left hand operand but found nothing.".to_string())
    }
}


fn push_rvalue<'a>(stack: & mut VecDeque<Statement<'a>>, token: LiteralVariation<'a>) -> Result<(), SyntaxError> {
    if let Some(prior_statement) = stack.back_mut() {
        match prior_statement {
            BinaryOperation { rhs, .. } => {
                match rhs {
                    None => {
                        let _ = mem::replace(rhs, Some(Box::new(RValue(RValueVariant::Literal(token)))));
                        Ok(())
                    }
                    _ => { Err("A RValue cannot be applied to another RValue. TODO: Bessere Beschreibung.".to_string()) }
                }
            }
            _ => Err("A RValue cannot be applied to another RValue. TODO: Bessere Beschreibung.".to_string())
        }
    } else {
        stack.push_back(RValue(RValueVariant::Literal(token)));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::AbstractSyntaxTree;
    use crate::parse::RValueVariant::Literal;
    use crate::parse::Statement::*;
    use crate::token::{LiteralVariation, OperatorVariation};

    use super::SyntaxError;

    #[test]
    fn parsed_string_literal() -> Result<(), SyntaxError> {
        let simple_invariant = "\"I am a String\"";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        if let Some(statement) = tree.root {
            assert!(statement.is_string_literal("I am a String"));
            Ok(())
        } else {
            Err("No root statement found!".to_string())
        }
    }

    #[test]
    fn parse_int_literal() -> Result<(), SyntaxError> {
        let simple_invariant = "42";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        if let Some(statement) = tree.root {
            assert!(statement.is_int_literal(42));
            Ok(())
        } else {
            Err("No root statement found!".to_string())
        }
    }
    
    #[test]
    fn parse_plus() -> Result<(), SyntaxError> {
        let simple_invariant = "10 + 20";

        let tree: AbstractSyntaxTree = AbstractSyntaxTree::from(simple_invariant)?;

        if let Some(statement) = tree.root {
            assert!(match statement {
                BinaryOperation { lhs, operator, rhs } => {
                    (match lhs {
                        Some(statement) => matches!(*statement, RValue(Literal(LiteralVariation::Integer(10)))),
                        _ => false
                    })
                    && matches!(operator, OperatorVariation::Plus)
                    && match rhs {
                        Some(statement) => matches!(*statement, RValue(Literal(LiteralVariation::Integer(20)))),
                        _ => false
                    }
                }
                _ => false
            });
            Ok(())
        } else {
            Err("No root statement found!".to_string())
        }
    }
}
