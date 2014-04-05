use std::fmt;
use std::vec::Vec;
use collections::treemap::TreeMap;
#[deriving(Clone, Eq)]
/// A Javascript Constant
pub enum Const {
	/// A UTF-8 string
	CString(~str),
	/// A regular expression
	CRegExp(~str, bool, bool),
	/// A 64-bit floating-point number
	CNum(f64),
	/// A 32-bit integer
	CInt(i32),
	/// A boolean
	CBool(bool),
	/// Null
	CNull,
	/// The infamous value returned when you access a non-existent field
	CUndefined
}
impl fmt::Show for Const {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			CString(ref st) => write!(f.buf, "\"{}\"", st),
			CRegExp(ref reg, _, _) => write!(f.buf, "~/{}/", reg),
			CNum(num) => write!(f.buf, "{}", num),
			CInt(num) => write!(f.buf, "{}", num),
			CBool(v) => write!(f.buf, "{}", v),
			CNull => write!(f.buf, "null"),
			CUndefined => write!(f.buf, "undefined")
		}
	}
}
#[deriving(Clone, Eq)]
/// An operation between 2 values
pub enum NumOp {
	/// Add them togther
	OpAdd,
	/// Subtract the second from the first
	OpSub,
	/// Divide the first by the second
	OpDiv,
	/// Multiply them together
	OpMul,
	/// Bitwise and
	OpAnd,
	/// Bitwise or
	OpOr
}
impl fmt::Show for NumOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return f.buf.write_str(match *self {
			OpAdd => "+",
			OpSub => "-",
			OpDiv => "/",
			OpMul => "*",
			OpAnd => "&",
			OpOr => "|"
		});
	}
}
#[deriving(Clone, Eq)]
/// A Javscript Expression
pub enum Expr {
	/// Run a numeric operation on two numeric expressions
	NumOpExpr(NumOp, ~Expr, ~Expr),
	/// Make a simple value
	ConstExpr(Const),
	/// Run several expressions
	BlockExpr(Vec<~Expr>),
	/// Load a reference to a value
	LocalExpr(~str),
	/// Gets the cosntant field of the expression
	GetConstFieldExpr(~Expr, ~str),
	/// Gets the field of an expression
	GetFieldExpr(~Expr, ~Expr),
	/// Call a function with some arguments
	CallExpr(~Expr, Vec<~Expr>),
	/// Repeatedly run an expression while the conditional expression resolves to true
	WhileLoopExpr(~Expr, ~Expr),
	/// Check if a conditional expression is true and run an expression if it is and another expression if it isn't
	IfExpr(~Expr, ~Expr, Option<~Expr>),
	/// Run blocks whose cases match the expression
	SwitchExpr(~Expr, Vec<(~Expr, Vec<~Expr>)>, Option<~Expr>),
	/// Create an object
	ObjectDeclExpr(~TreeMap<~str, ~Expr>),
	/// Create an array with items inside
	ArrayDeclExpr(Vec<~Expr>),
	/// Create a function with the given name, arguments, and expression
	FunctionDeclExpr(Option<~str>, Vec<~str>, ~Expr),
	/// Construct an object from the function and arguments given
	ConstructExpr(~Expr, Vec<~Expr>),
	/// Return the expression from a function
	ReturnExpr(Option<~Expr>),
	/// Throw an expression
	ThrowExpr(~Expr),
	/// Assign an expression to another expression
	AssignExpr(~Expr, ~Expr)
}
impl fmt::Show for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			ConstExpr(ref c) => write!(f.buf, "{}", c),
			BlockExpr(ref block) => write!(f.buf, "{} {} {}", '{', block, '}'),
			LocalExpr(ref s) => write!(f.buf, "{}", s),
			GetConstFieldExpr(ref ex, ref field) => write!(f.buf, "{}.{}", ex, field),
			GetFieldExpr(ref ex, ref field) => write!(f.buf, "{}[{}]", ex, field),
			CallExpr(ref ex, ref args) => write!(f.buf, "{}({})", ex, args),
			ConstructExpr(ref func, ref args) => write!(f.buf, "new {}({})", func, args),
			WhileLoopExpr(ref cond, ref expr) => write!(f.buf, "while({}) {}", cond, expr),
			IfExpr(ref cond, ref expr, None) => write!(f.buf, "if({}) {}", cond, expr),
			IfExpr(ref cond, ref expr, Some(ref else_e)) => write!(f.buf, "if({}) {} else {}", cond, expr, else_e),
			SwitchExpr(ref val, ref vals, None) => write!(f.buf, "switch({}){}", val, vals),
			SwitchExpr(ref val, ref vals, Some(ref def)) => write!(f.buf, "switch({}){}default:{}", val, vals, def),
			ObjectDeclExpr(ref map) => write!(f.buf, "{}", map),
			ArrayDeclExpr(ref arr) => write!(f.buf, "{}", arr),
			FunctionDeclExpr(ref name, ref args, ref expr) => write!(f.buf, "function {}({}){}", name, args, expr),
			NumOpExpr(ref op, ref a, ref b) => write!(f.buf, "{} {} {}", a, op, b),
			ReturnExpr(Some(ref ex)) => write!(f.buf, "return {}", ex),
			ReturnExpr(None) => f.buf.write_str("return"),
			ThrowExpr(ref ex) => write!(f.buf, "throw {}", ex),
			AssignExpr(ref ref_e, ref val) => write!(f.buf, "{} = {}", ref_e, val)
		}
	}
}

impl fmt::Show for TreeMap<~str, ~Expr> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(f.buf.write_str("{ "));
		for (k, v) in self.iter() {
			try!(write!(f.buf, "{} = {}\n", k, v));
		}
		f.buf.write_str("}")
	}
}
#[deriving(Clone)]
#[deriving(Eq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum Token {
	/// A string literal
	TString(~str),
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
	TIdent(~str),
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
	/// A question
	TQuestion,
	/// A numeric operation
	TNumOp(NumOp)
}
impl fmt::Show for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			TString(ref s) => write!(f.buf, "\"{}\"", s),
			TSemicolon => f.buf.write_str(";"),
			TColon => f.buf.write_str(":"),
			TDot => f.buf.write_str("."),
			TEqual => f.buf.write_str("="),
			TComma => f.buf.write_str(","),
			TIdent(ref ident) => f.buf.write_str(*ident),
			TOpenParen => f.buf.write_str("("),
			TCloseParen => f.buf.write_str(")"),
			TOpenBlock => f.buf.write_str("{"),
			TCloseBlock => f.buf.write_str("}"),
			TOpenArray => f.buf.write_str("["),
			TCloseArray => f.buf.write_str("]"),
			TNumber(num) => write!(f.buf, "{}", num),
			TQuestion => f.buf.write_str("?"),
			TNumOp(op) => write!(f.buf, "{}", op)
		}
	}
}