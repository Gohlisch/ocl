
use std::ops::Range;
use crate::parsing_error::ParsingError;
use crate::token::{KeywordVariation, OperatorVariation, Token};
use crate::token::KeywordVariation::{*};
use crate::token::LiteralVariation::{Integer, Real, String};
use crate::token::OperatorVariation::{*};
use crate::token::Token::Operator;

const NUMBER_LITERAL_CHARS: Range<char> = '0'..'9';
const OPERATOR_OR_COMMENT_CHARS: [char; 20] = ['.' , '*' ,'+' ,'-' ,'/' ,'<' ,'>' ,'<' ,'>' ,'=' ,':' ,',' ,'(' ,')' ,'{' ,'}' ,'[' ,']' ,'|' ,'@'];
const OPERATOR_FOLLOW_UP: [char; 4] = ['.', '>', '=' ,':'];
const IGNORED_WHITE_SPACE_CHARACTERS: [char; 4] = [' ', '\n', '\t', '\r'];

pub fn lex(input: &str) -> Result<Vec<Token<'_>>, ParsingError<'_>> {
    let mut start_of_token: usize = 0;
    let mut end_of_token: usize = 0;
    let mut tokens: Vec<Token> = vec![];

    'outer: loop  {
        if end_of_token >= input.len() {break 'outer;}
        start_of_token = end_of_token;
        let char = input.chars().nth(start_of_token).unwrap();

        if IGNORED_WHITE_SPACE_CHARACTERS.contains(&char) {
            end_of_token += 1;
            continue 'outer;
        }

        if (NUMBER_LITERAL_CHARS).contains(&char) {
            let mut encountered_point = false;

            'inner_look_for_end_of_number: loop {
                end_of_token += 1;
                if end_of_token == input.len() {break 'inner_look_for_end_of_number;}

                let current_char = input.chars().nth(end_of_token).unwrap();

                if current_char == '.' {
                    if encountered_point { continue 'outer; }
                    encountered_point = true;
                    continue 'inner_look_for_end_of_number;
                }
                if !(NUMBER_LITERAL_CHARS).contains(&current_char) { break 'inner_look_for_end_of_number; }
            }

            let to_be_parsed = &input[start_of_token..end_of_token];
            if encountered_point {
                tokens.push(Token::Literal(Real(to_be_parsed.parse().unwrap())));
            } else {
                tokens.push(Token::Literal(Integer(to_be_parsed.parse().unwrap())));
            }
            continue 'outer;
        }

        if char == '"' {
            'inner_look_for_end_of_string: loop {
                end_of_token += 1;
                if end_of_token >= input.len() { return Err(ParsingError{msg: "String was not terminated. Awaited '\"'.", from: start_of_token, to: end_of_token} )}

                let current_char = input.chars().nth(end_of_token).unwrap();
                if current_char == '"' { break 'inner_look_for_end_of_string; }
            }
            tokens.push(Token::Literal(String(&input[start_of_token .. end_of_token])));
            end_of_token += 1;
            continue 'outer;
        }

        if OPERATOR_OR_COMMENT_CHARS.contains(&char) {
            if end_of_token < input.len() {
                let second_char = input.chars().nth(end_of_token).unwrap();
                if char == '-' && second_char == '-' {
                    loop { // ignore comment
                        end_of_token += 1;
                        if end_of_token == input.len() { break 'outer; }

                        let current_char = input.chars().nth(end_of_token).unwrap();
                        if current_char == '\n' {
                            end_of_token += 1;
                            continue 'outer
                        }
                    }
                }
                if let Some(two_char_operator) = check_operator(&input[start_of_token .. end_of_token+1]) {
                    tokens.push(Operator(two_char_operator));
                    end_of_token += 1;
                    continue 'outer;
                }
            }

            unsafe { // because OPERATOR_OR_COMMENT_CHARS contains the char it is a valid operator
                tokens.push(Operator(check_operator(&input[start_of_token..end_of_token+1]).unwrap_unchecked()));
            }

            end_of_token += 1;
            continue 'outer;
        }

        if is_part_of_keyword_or_identifier(char) {
            'inner_look_for_end_of_identifier_or_keyword: loop {
                end_of_token += 1;
                if end_of_token >= input.len() {break 'outer;}

                let current_char = input.chars().nth(end_of_token).unwrap();
                if !is_part_of_keyword_or_identifier(current_char) { break 'inner_look_for_end_of_identifier_or_keyword; }
            }
            if let Some(keyword) = check_keyword(&input[start_of_token .. end_of_token]) {
                tokens.push(Token::Keyword(keyword));
            } else {
                tokens.push(Token::Identifier(&input[start_of_token .. end_of_token]));
            }
            continue 'outer;
        }

        return Err(ParsingError{msg: "Invalid character", from: start_of_token, to: end_of_token} );
    }

    Ok(tokens)
}

fn check_keyword(input: &str) -> Option<KeywordVariation> {
    match input {
        "and" => Some(And),
        "body" => Some(Body),
        "context" => Some(Context),
        "def" => Some(Def),
        "derive" => Some(Derive),
        "else" => Some(Else),
        "endif" => Some(Endif),
        "endpackage" => Some(Endpackage),
        "false" => Some(False),
        "if" => Some(If),
        "implies" => Some(Implies),
        "in" => Some(In),
        "init" => Some(Init),
        "inv" => Some(Inv),
        "invalid" => Some(Invalid),
        "let" => Some(Let),
        "not" => Some(Not),
        "null" => Some(Null),
        "or" => Some(Or),
        "package" => Some(Package),
        "post" => Some(Post),
        "pre" => Some(Pre),
        "self" => Some(SelfReference),
        "static" => Some(Static),
        "then" => Some(Then),
        "true" => Some(True),
        "xor" => Some(Xor),
        _ => None
    }
}

fn check_operator(input: &str) -> Option<OperatorVariation> {
    match input {
        "." => Some(Point),
        ".." => Some(DoublePoint),
        "->" => Some(Arrow),
        "*" => Some(Star),
        "+" => Some(Plus),
        "-" => Some(Minus),
        "/" => Some(Slash),
        "<" => Some(LessThan),
        ">" => Some(GreaterThan),
        "<=" => Some(LessThanEqual),
        ">=" => Some(GreaterThanEqual),
        "=" => Some(Equals),
        "<>" => Some(Unequals),
        ":" => Some(Colon),
        "::" => Some(DoubleColon),
        "," => Some(Comma),
        "(" => Some(OpeningRoundBracket),
        ")" => Some(ClosingRoundBracket),
        "{" => Some(OpeningCurlyBracket),
        "}" => Some(ClosingCurlyBracket),
        "[" => Some(OpeningSquareBracket),
        "]" => Some(ClosingSquareBracket),
        "|" => Some(Pipe),
        "@" => Some(AtPre),
        _ => None
    }
}

fn is_part_of_keyword_or_identifier(char: char) -> bool {
    ('a'..'z').contains(&char) || ('A'..'Z').contains(&char)
}

#[cfg(test)]
mod tests {
    use crate::token::{KeywordVariation, LiteralVariation, OperatorVariation};
    use super::*;

    #[test]
    fn lex_simple_invariant() -> Result<(), ParsingError<'static>> {
        let simple_invariant = "self.numberOfEmployees > 50";

        let tokens = lex(simple_invariant)?;

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens.get(0), Some(Token::Keyword(KeywordVariation::SelfReference))));
        assert!(matches!(tokens.get(1), Some(Token::Operator(OperatorVariation::Point))));
        assert!(matches!(tokens.get(2), Some(Token::Identifier("numberOfEmployees"))));
        assert!(matches!(tokens.get(3), Some(Token::Operator(OperatorVariation::GreaterThan))));
        assert!(matches!(tokens.get(4), Some(Token::Literal(LiteralVariation::Integer(50)))));
        Ok(())
    }

    #[test]
    fn lex_invariant_with_contextual_instance() -> Result<(), ParsingError<'static>> {
        let invariant_with_context_instance = "context Company inv:
self.numberOfEmployees > 50";

        let tokens = lex(invariant_with_context_instance)?;

        assert_eq!(tokens.len(), 9);
        assert!(matches!(tokens.get(0), Some(Token::Keyword(Context))));
        assert!(matches!(tokens.get(1), Some(Token::Identifier("Company"))));
        assert!(matches!(tokens.get(2), Some(Token::Keyword(Inv))));
        assert!(matches!(tokens.get(3), Some(Token::Operator(Colon))));
        assert!(matches!(tokens.get(4), Some(Token::Keyword(SelfReference))));
        assert!(matches!(tokens.get(5), Some(Token::Operator(OperatorVariation::Point))));
        assert!(matches!(tokens.get(6), Some(Token::Identifier("numberOfEmployees"))));
        assert!(matches!(tokens.get(7), Some(Token::Operator(OperatorVariation::GreaterThan))));
        assert!(matches!(tokens.get(8), Some(Token::Literal(LiteralVariation::Integer(50)))));
        Ok(())
    }

    #[test]
    fn lex_invariant_with_named_contextual_instance() -> Result<(), ParsingError<'static>> {
        let invariant_with_named_contextual_instance = "context c : Company inv:
c.numberOfEmployees > 50";

        let tokens = lex(invariant_with_named_contextual_instance)?;

        assert_eq!(tokens.len(), 11);
        assert!(matches!(tokens.get(0), Some(Token::Keyword(Context))));
        assert!(matches!(tokens.get(1), Some(Token::Identifier("c"))));
        assert!(matches!(tokens.get(2), Some(Token::Operator(Colon))));
        assert!(matches!(tokens.get(3), Some(Token::Identifier("Company"))));
        assert!(matches!(tokens.get(4), Some(Token::Keyword(Inv))));
        assert!(matches!(tokens.get(5), Some(Token::Operator(Colon))));
        assert!(matches!(tokens.get(6), Some(Token::Identifier("c"))));
        assert!(matches!(tokens.get(7), Some(Token::Operator(OperatorVariation::Point))));
        assert!(matches!(tokens.get(8), Some(Token::Identifier("numberOfEmployees"))));
        assert!(matches!(tokens.get(9), Some(Token::Operator(OperatorVariation::GreaterThan))));
        assert!(matches!(tokens.get(10), Some(Token::Literal(LiteralVariation::Integer(50)))));
        Ok(())
    }

    #[test]
    fn lex_named_invariant_with_named_contextual_instance() -> Result<(), ParsingError<'static>> {
        let named_invariant = "context c : Company inv enoughEmployees:
c.numberOfEmployees > 50";

        let tokens = lex(named_invariant)?;

        assert_eq!(tokens.len(), 12);
        assert!(matches!(tokens.get(0), Some(Token::Keyword(Context))));
        assert!(matches!(tokens.get(1), Some(Token::Identifier("c"))));
        assert!(matches!(tokens.get(2), Some(Token::Operator(Colon))));
        assert!(matches!(tokens.get(3), Some(Token::Identifier("Company"))));
        assert!(matches!(tokens.get(4), Some(Token::Keyword(Inv))));
        assert!(matches!(tokens.get(5), Some(Token::Identifier("enoughEmployees"))));
        assert!(matches!(tokens.get(6), Some(Token::Operator(Colon))));
        assert!(matches!(tokens.get(7), Some(Token::Identifier("c"))));
        assert!(matches!(tokens.get(8), Some(Token::Operator(OperatorVariation::Point))));
        assert!(matches!(tokens.get(9), Some(Token::Identifier("numberOfEmployees"))));
        assert!(matches!(tokens.get(10), Some(Token::Operator(OperatorVariation::GreaterThan))));
        assert!(matches!(tokens.get(11), Some(Token::Literal(LiteralVariation::Integer(50)))));
        Ok(())
    }
}