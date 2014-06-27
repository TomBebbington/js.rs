use std::fmt::{Formatter, Result, Show};
use syntax::ast::pos::Position;
use syntax::ast::punc::Punctuator;
use syntax::ast::keyword::Keyword;
#[deriving(Clone, PartialEq)]
/// A single of token of Javascript code including its position
pub struct Token {
    /// The token
    pub data : TokenData,
    /// The token's position
    pub pos : Position
}
impl Token {
    /// Create a new detailed token from the token data, line number and column number
    pub fn new(data: TokenData, line_number: uint, column_number: uint) -> Token {
        Token {
            data: data,
            pos: Position::new(line_number, column_number)
        }
    }
}
impl Show for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.data)
    }
}
#[deriving(Clone, PartialEq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum TokenData {
    /// A boolean literal, which is either `true` or `false`
    TBooleanLiteral(bool),
    /// The end of the file
    TEOF,
    /// An identifier
    TIdentifier(String),
    /// A keyword
    TKeyword(Keyword),
    /// A `null` literal
    TNullLiteral,
    /// A numeric literal
    TNumericLiteral(f64),
    /// A piece of punctuation
    TPunctuator(Punctuator),
    /// A string literal
    TStringLiteral(String),
    /// A regular expression
    TRegularExpression(String),
    /// A comment
    TComment(String)
}
impl Show for TokenData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.clone() {
            TBooleanLiteral(val) => write!(f, "{}", val),
            TEOF => write!(f, "end of file"),
            TIdentifier(ident) => write!(f, "{}", ident),
            TKeyword(word) => write!(f, "{}", word),
            TNullLiteral => write!(f, "null"),
            TNumericLiteral(num) => write!(f, "{}", num),
            TPunctuator(punc) => write!(f, "{}", punc),
            TStringLiteral(lit) => write!(f, "{}", lit),
            TRegularExpression(reg) => write!(f, "{}", reg),
            TComment(comm) => write!(f, "/*{}*/", comm)
        }
    }
}