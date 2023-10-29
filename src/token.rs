pub enum Token<'a> {
    Keyword(KeywordVariation),
    Identifier(&'a str),
    Operator(OperatorVariation),
    Literal(LiteralVariation<'a>),
}

/**
* As specified in the OCL specification chapter 7.4
*/
pub enum LiteralVariation<'a> {
    String(&'a str),
    Integer(i64),
    Real(f64),
}

/**
* As specified in the OCL specification chapter 7.4.9 and 7.4.10
*/
pub enum OperatorVariation {
    Point,
    DoublePoint,
    Arrow,
    Star,
    Plus,
    Minus,
    Slash,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equals,
    Unequals,
    Colon,
    DoubleColon,
    Comma,
    OpeningRoundBracket,
    ClosingRoundBracket,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    OpeningSquareBracket,
    ClosingSquareBracket,
    Pipe,
    AtPre,
}


/**
 * As specified in the OCL specification chapter 7.4.11
 */
pub enum KeywordVariation {
    And,
    Body,
    Context,
    Def,
    Derive,
    Else,
    Endif,
    Endpackage,
    False,
    If,
    Implies,
    In,
    Init,
    Inv,
    Invalid,
    Let,
    Not,
    Null,
    Or,
    Package,
    Post,
    Pre,
    SelfReference,
    Static,
    Then,
    True,
    Xor
}