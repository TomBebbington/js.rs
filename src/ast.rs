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
			CString(ref st) => write!(f, "\"{}\"", st),
			CRegExp(ref reg, _, _) => write!(f, "~/{}/", reg),
			CNum(num) => write!(f, "{}", num),
			CInt(num) => write!(f, "{}", num),
			CBool(v) => write!(f, "{}", v),
			CNull => write!(f, "null"),
			CUndefined => write!(f, "undefined")
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
	/// Get the modulus of a number and another
	OpMod
}
impl fmt::Show for NumOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			OpAdd => "+",
			OpSub => "-",
			OpDiv => "/",
			OpMul => "*",
			OpMod => "%"
		})
	}
}
#[deriving(Clone, Eq)]
/// An operation on a single value
pub enum UnaryOp {
	/// Unary increment (++) with the bool being true if it is before the variable
	UnaryIncrement(bool),
	/// Unary decrement (--) with the bool being true if it is before the variable
	UnaryDecrement(bool),
	/// Unary minus operator on a number or variable
	UnaryMinus
}
impl fmt::Show for UnaryOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			UnaryIncrement(_) => "++",
			UnaryDecrement(_) => "--",
			UnaryMinus => "-"
		})
	}
}
#[deriving(Clone, Eq)]
/// A bitwise operation
pub enum BitOp {
	/// Bitwise and
	BitAnd,
	/// Bitwise or
	BitOr,
	/// Bitwise xor
	BitXor,
	/// Bitwise shift left
	BitShl,
	/// Bitwise shift right
	BitShr
}
impl fmt::Show for BitOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			BitAnd => "&",
			BitOr => "|",
			BitXor => "^",
			BitShl => "<<",
			BitShr => ">>"
		})
	}
}
#[deriving(Clone, Eq)]
/// A comparison operation between two values
pub enum CompOp {
	/// If they represent the same value or similar values
	CompEqual,
	/// If they represent distinct values
	CompNotEqual,
	/// If they represent the same value
	CompStrictEqual,
	/// If they represent very distinct values
	CompStrictNotEqual,
	/// If the first is greater than the second
	CompGreaterThan,
	/// If the first is greater than or equal to the second
	CompGreaterThanOrEqual,
	/// If the first is less than the second
	CompLessThan,
	/// If the first is less than or equal to the second
	CompLessThanOrEqual,
}
impl fmt::Show for CompOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			CompEqual => "==",
			CompNotEqual => "!=",
			CompStrictEqual => "===",
			CompStrictNotEqual => "!==",
			CompGreaterThan => ">",
			CompGreaterThanOrEqual => ">=",
			CompLessThan => "<",
			CompLessThanOrEqual => "<="
		})
	}
}
#[deriving(Clone, Eq)]
/// A logical operation between two booleans
pub enum LogOp {
	/// Logical and
	LogAnd,
	/// Logical or
	LogOr
}
impl fmt::Show for LogOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			LogAnd => "&&",
			LogOr => "||"
		})
	}
}
#[deriving(Clone, Eq)]
/// A Javascript Expression, including position data
pub struct Expr {
	/// The expression definition
	pub def : ExprDef,
	/// The starting position
	pub start : Position,
	/// The ending position
	pub end : Position
}
impl Expr {
	/// Create a new expression with a position
	pub fn new(def: ExprDef, start:Position, end:Position) -> Expr {
		Expr{def: def, start: start, end: end}
	}
}
impl fmt::Show for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}: {}", self.start.line_number, self.start.column_number, self.def)
	}
}
#[deriving(Clone, Eq)]
/// A position in Javascript source code
pub struct Position {
	/// The column number
	pub column_number : uint,
	/// The line number
	pub line_number : uint
}
impl Position {
	/// Create a new position
	pub fn new(line_number: uint, column_number: uint) -> Position {
		Position {
			line_number: line_number,
			column_number: column_number
		}
	}
}
#[deriving(Clone, Eq)]
/// A Javascript Expression
pub enum ExprDef {
	/// Run a numeric operation on two numeric expressions
	NumOpExpr(NumOp, Box<Expr>, Box<Expr>),
	/// Run an operation on an expression
	UnaryOpExpr(UnaryOp, Box<Expr>),
	/// Run a bitwise operation on two integer expressions
	BitOpExpr(BitOp, Box<Expr>, Box<Expr>),
	/// Run a logical operation on two boolean expressions
	LogOpExpr(LogOp, Box<Expr>, Box<Expr>),
	/// Run a comparison operation on two expressions
	CompOpExpr(CompOp, Box<Expr>, Box<Expr>),
	/// Make a simple value
	ConstExpr(Const),
	/// Run several expressions
	BlockExpr(Vec<Expr>),
	/// Load a reference to a value
	LocalExpr(~str),
	/// Gets the cosntant field of the expression
	GetConstFieldExpr(Box<Expr>, ~str),
	/// Gets the field of an expression
	GetFieldExpr(Box<Expr>, Box<Expr>),
	/// Call a function with some arguments
	CallExpr(Box<Expr>, Vec<Expr>),
	/// Repeatedly run an expression while the conditional expression resolves to true
	WhileLoopExpr(Box<Expr>, Box<Expr>),
	/// Check if a conditional expression is true and run an expression if it is and another expression if it isn't
	IfExpr(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
	/// Run blocks whose cases match the expression
	SwitchExpr(Box<Expr>, Vec<(Expr, Vec<Expr>)>, Option<Box<Expr>>),
	/// Create an object
	ObjectDeclExpr(Box<TreeMap<~str, Expr>>),
	/// Create an array with items inside
	ArrayDeclExpr(Vec<Expr>),
	/// Create a function with the given name, arguments, and expression
	FunctionDeclExpr(Option<~str>, Vec<~str>, Box<Expr>),
	/// Create an arrow function with the fiven arguments and expression
	ArrowFunctionDeclExpr(Vec<~str>, Box<Expr>),
	/// Construct an object from the function and arguments given
	ConstructExpr(Box<Expr>, Vec<Expr>),
	/// Return the expression from a function
	ReturnExpr(Option<Box<Expr>>),
	/// Throw an expression
	ThrowExpr(Box<Expr>),
	/// Assign an expression to another expression
	AssignExpr(Box<Expr>, Box<Expr>),
	/// Return a string representing the type of the given expression
	TypeOfExpr(Box<Expr>)
}
impl fmt::Show for ExprDef {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			ConstExpr(ref c) => write!(f, "{}", c),
			BlockExpr(ref block) => {
				try!(write!(f, "{}", "{"));
				for expr in block.iter() {
					try!(write!(f, "{};", expr));
				}
				write!(f, "{}", "}")
			},
			LocalExpr(ref s) => write!(f, "{}", s),
			GetConstFieldExpr(ref ex, ref field) => write!(f, "{}.{}", ex, field),
			GetFieldExpr(ref ex, ref field) => write!(f, "{}[{}]", ex, field),
			CallExpr(ref ex, ref args) => {
				try!(write!(f, "{}", ex));
				try!(write!(f, "{}", "("));
				let last = args.iter().last();
				match last {
					Some(last_arg) => {
						for arg in args.iter() {
							try!(write!(f, "{}", arg));
							if arg != last_arg {
								try!(write!(f, "{}", ", "));
							}
						}
					},
					None => ()
				}
				write!(f, "{}", ")")
			},
			ConstructExpr(ref func, ref args) => write!(f, "new {}({})", func, args),
			WhileLoopExpr(ref cond, ref expr) => write!(f, "while({}) {}", cond, expr),
			IfExpr(ref cond, ref expr, None) => write!(f, "if({}) {}", cond, expr),
			IfExpr(ref cond, ref expr, Some(ref else_e)) => write!(f, "if({}) {} else {}", cond, expr, else_e),
			SwitchExpr(ref val, ref vals, None) => write!(f, "switch({}){}", val, vals),
			SwitchExpr(ref val, ref vals, Some(ref def)) => write!(f, "switch({}){}default:{}", val, vals, def),
			ObjectDeclExpr(ref map) => write!(f, "{}", map),
			ArrayDeclExpr(ref arr) => write!(f, "{}", arr),
			FunctionDeclExpr(ref name, ref args, ref expr) => write!(f, "function {}({}){}", name, args.connect(", "), expr),
			ArrowFunctionDeclExpr(ref args, ref expr) => write!(f, "({}) => {}", args.connect(", "), expr),
			NumOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
			UnaryOpExpr(ref op, ref a) => write!(f, "{}{}", op, a),
			BitOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
			LogOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
			CompOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
			ReturnExpr(Some(ref ex)) => write!(f, "return {}", ex),
			ReturnExpr(None) => write!(f, "{}", "return"),
			ThrowExpr(ref ex) => write!(f, "throw {}", ex),
			AssignExpr(ref ref_e, ref val) => write!(f, "{} = {}", ref_e, val),
			TypeOfExpr(ref e) => write!(f, "typeof {}", e)
		}
	}
}

impl fmt::Show for TreeMap<~str, Expr> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(write!(f, "{}", "{ "));
		match self.iter().last() {
			Some((last_key, _)) => {
				for (k, v) in self.iter() {
					try!(write!(f, "{}: {}", k, v));
					if k == last_key {
						try!(write!(f, "{}", ","));
					}
					try!(write!(f, "{}", "\n"));
				}
			},
			None => ()
		}
		write!(f, "{}", "}")
	}
}
#[deriving(Clone)]
#[deriving(Eq)]
/// A single of token of Javascript code including the position
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
impl fmt::Show for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}: {}", self.pos.line_number, self.pos.column_number, self.data)
	}
}
#[deriving(Clone)]
#[deriving(Eq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum TokenData {
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
	/// A question mark
	TQuestion,
	/// An arrow
	TArrow,
	/// A numeric operation
	TNumOp(NumOp),
	/// A bitwise operation
	TBitOp(BitOp),
	/// A logical operation
	TLogOp(LogOp),
	/// A comparison operation
	TCompOp(CompOp),
	/// A comment
	TComment(~str)
}
impl fmt::Show for TokenData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
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
			TNumOp(op) => write!(f, "{}", op),
			TBitOp(op) => write!(f, "{}", op),
			TLogOp(op) => write!(f, "{}", op),
			TCompOp(op) => write!(f, "{}", op),
			TComment(ref com) => write!(f, "// {}", com)
		}
	}
}