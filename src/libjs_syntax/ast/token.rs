use std::fmt::{Formatter, Result, Show};
use ast::pos::Position;
use ast::op::{BinOp, UnaryOp};
#[deriving(Clone)]
#[deriving(Eq)]
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
#[deriving(Clone)]
#[deriving(Eq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum TokenData {
	/// A string literal
	TString(String),
	/// A semicolon (;)
	TSemicolon,
	/// A colon
	TColon,
	/// A dot / full stop
	TDot,
	/// An equal sign
	TEqual,
	/// A comma
	TComma,
	/// An identity literal
	TIdent(String),
	/// An opening bracket
	TOpenParen,
	/// A closing bracket
	TCloseParen,
	/// An opening curly bracket
	TOpenBlock,
	/// An closing curly bracket
	TCloseBlock,
	/// An opening square bracket
	TOpenArray,
	/// A closing square bracket
	TCloseArray,
	/// A 64-bit floating-point number
	TNumber(f64),
	/// A question mark
	TQuestion,
	/// An arrow
	TArrow,
	/// An operation between 2 values
	TBinOp(BinOp),
	/// A unary operation
	TUnaryOp(UnaryOp),
	/// An assign operation combined with something else
	TAssignOp(BinOp),
	/// A comment
	TComment(String)
}
impl Show for TokenData {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self.clone() {
			TString(ref s) => write!(f, "\"{}\"", s),
			TSemicolon => write!(f, "{}", ";"),
			TColon => write!(f, "{}", ":"),
			TDot => write!(f, "{}", "."),
			TEqual => write!(f, "{}", "="),
			TComma => write!(f, "{}", ","),
			TIdent(ref ident) => write!(f, "{}", *ident),
			TOpenParen => write!(f, "{}", "("),
			TCloseParen => write!(f, "{}", ")"),
			TOpenBlock => write!(f, "{}", "{"),
			TCloseBlock => write!(f, "{}", "}"),
			TOpenArray => write!(f, "{}", "["),
			TCloseArray => write!(f, "{}", "]"),
			TNumber(num) => write!(f, "{}", num),
			TQuestion => write!(f, "{}", "?"),
			TArrow => write!(f, "{}", "=>"),
			TBinOp(op) => write!(f, "{}", op),
			TUnaryOp(op) => write!(f, "{}", op),
			TAssignOp(op) => write!(f, "{}=", op),
			TComment(ref com) => write!(f, "// {}", com)
		}
	}
}