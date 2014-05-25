use std::fmt;
use std::vec::Vec;
use collections::treemap::TreeMap;
/// Represents an operator
pub trait Operator {
	/// Get the precedence as an unsignes integer, where the lower it is, the more precedence/priority it has
	fn get_precedence(&self) -> uint;
}
#[deriving(Clone, Eq)]
/// A Javascript constant
pub enum Const {
	/// A UTF-8 string, such as `"Hello, world"`
	CString(StrBuf),
	/// A regular expression, such as `/where('s| is) [wW]ally/`
	CRegExp(StrBuf, bool, bool),
	/// A 64-bit floating-point number, such as `3.1415`
	CNum(f64),
	/// A 32-bit integer, such as `42`
	CInt(i32),
	/// A boolean, which is either `true` or `false` and is used to check if criteria are met
	CBool(bool),
	/// The `null` value, which represents a non-existant value
	CNull,
	/// The `undefined` value, which represents a field or index that doesn't exist
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
/// A numeric operation between 2 values
pub enum NumOp {
	/// `a + b` - Addition
	OpAdd,
	/// `a - b` - Subtraction
	OpSub,
	/// `a / b` - Division
	OpDiv,
	/// `a * b` - Multiplication
	OpMul,
	/// `a % b` - Modulus
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
/// A unary operation on a single value
pub enum UnaryOp {
	/// `a++` - increment the value
	UnaryIncrement(bool),
	/// `a--` - decrement the value
	UnaryDecrement(bool),
	/// `-a` - negate the value
	UnaryMinus,
	/// `!a` - get the opposite of the boolean value
	UnaryNot
}
impl fmt::Show for UnaryOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			UnaryIncrement(_) => "++",
			UnaryDecrement(_) => "--",
			UnaryMinus => "-",
			UnaryNot => "!"
		})
	}
}
#[deriving(Clone, Eq)]
/// A bitwise operation between 2 values
pub enum BitOp {
	/// `a & b` - Bitwise and
	BitAnd,
	/// `a | b` - Bitwise or
	BitOr,
	/// `a ^ b` - Bitwise xor
	BitXor,
	/// `a << b` - Bit-shift leftwards
	BitShl,
	/// `a >> b` - Bit-shift rightrights
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
/// A comparitive operation between 2 values
pub enum CompOp {
	/// `a == b` - Equality
	CompEqual,
	/// `a != b` - Unequality
	CompNotEqual,
	/// `a === b` - Strict equality
	CompStrictEqual,
	/// `a !== b` - Strict unequality
	CompStrictNotEqual,
	/// `a > b` - If `a` is greater than `b`
	CompGreaterThan,
	/// `a >= b` - If `a` is greater than or equal to `b`
	CompGreaterThanOrEqual,
	/// `a < b` - If `a` is less than `b`
	CompLessThan,
	/// `a <= b` - If `a` is less than or equal to `b`
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
/// A logical operation between 2 boolean values
pub enum LogOp {
	/// `a && b` - Logical and
	LogAnd,
	/// `a || b` - Logical or
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
/// A binary operation between 2 values
pub enum BinOp {
	/// Numeric operation
	BinNum(NumOp),
	/// Bitwise operation
	BinBit(BitOp),
	/// Comparitive operation
	BinComp(CompOp),
	/// Logical operation
	BinLog(LogOp)
}
impl Operator for BinOp {
	fn get_precedence(&self) -> uint {
		match *self {
			BinNum(OpMul) | BinNum(OpDiv) | BinNum(OpMod) => 5,
			BinNum(OpAdd) | BinNum(OpSub) => 6,
			BinBit(BitShl) | BinBit(BitShr) => 7,
			BinComp(CompLessThan) | BinComp(CompLessThanOrEqual) | BinComp(CompGreaterThan) | BinComp(CompGreaterThanOrEqual) => 8,
			BinComp(CompEqual) | BinComp(CompNotEqual) | BinComp(CompStrictEqual) | BinComp(CompStrictNotEqual) => 9,
			BinBit(BitAnd) => 10,
			BinBit(BitXor) => 11,
			BinBit(BitOr) => 12,
			BinLog(LogAnd) => 13,
			BinLog(LogOr) => 14,
			
		}
	}
}
impl fmt::Show for BinOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			BinNum(op) => op.to_str(),
			BinBit(op) => op.to_str(),
			BinComp(op) => op.to_str(),
			BinLog(op) => op.to_str()
		})
	}
}
#[deriving(Clone, Eq)]
/// A Javascript expression, including its position
pub struct Expr {
	/// The expression definition
	pub def : ExprDef,
	/// The starting position
	pub start : Position,
	/// The ending position
	pub end : Position
}
impl Expr {
	/// Create a new expression with a starting and ending position
	pub fn new(def: ExprDef, start:Position, end:Position) -> Expr {
		Expr{def: def, start: start, end: end}
	}
}
impl fmt::Show for Expr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.def)
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
/// A Javascript expression
pub enum ExprDef {
	/// Run a operation between 2 expressions
	BinOpExpr(BinOp, Box<Expr>, Box<Expr>),
	/// Run an operation on a value
	UnaryOpExpr(UnaryOp, Box<Expr>),
	/// Make a constant value
	ConstExpr(Const),
	/// Run several expressions from top-to-bottom
	BlockExpr(Vec<Expr>),
	/// Load a reference to a value
	LocalExpr(StrBuf),
	/// Gets the constant field of a value
	GetConstFieldExpr(Box<Expr>, StrBuf),
	/// Gets the field of a value
	GetFieldExpr(Box<Expr>, Box<Expr>),
	/// Call a function with some values
	CallExpr(Box<Expr>, Vec<Expr>),
	/// Repeatedly run an expression while the conditional expression resolves to true
	WhileLoopExpr(Box<Expr>, Box<Expr>),
	/// Check if a conditional expression is true and run an expression if it is and another expression if it isn't
	IfExpr(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
	/// Run blocks whose cases match the expression
	SwitchExpr(Box<Expr>, Vec<(Expr, Vec<Expr>)>, Option<Box<Expr>>),
	/// Create an object out of the binary tree given
	ObjectDeclExpr(Box<TreeMap<StrBuf, Expr>>),
	/// Create an array with items inside
	ArrayDeclExpr(Vec<Expr>),
	/// Create a function with the given name, arguments, and expression
	FunctionDeclExpr(Option<StrBuf>, Vec<StrBuf>, Box<Expr>),
	/// Create an arrow function with the given arguments and expression
	ArrowFunctionDeclExpr(Vec<StrBuf>, Box<Expr>),
	/// Construct an object from the function and arguments given
	ConstructExpr(Box<Expr>, Vec<Expr>),
	/// Return the expression from a function
	ReturnExpr(Option<Box<Expr>>),
	/// Throw a value
	ThrowExpr(Box<Expr>),
	/// Assign an expression to a value
	AssignExpr(Box<Expr>, Box<Expr>),
	/// Return a string representing the type of the given expression
	TypeOfExpr(Box<Expr>)
}
impl Operator for ExprDef {
	fn get_precedence(&self) -> uint {
		match *self {
			GetFieldExpr(_, _) | GetConstFieldExpr(_, _) => 1,
			CallExpr(_, _) | ConstructExpr(_, _) => 2,
			UnaryOpExpr(UnaryIncrement(_), _) | UnaryOpExpr(UnaryDecrement(_), _) => 3,
			UnaryOpExpr(UnaryNot, _) | UnaryOpExpr(UnaryMinus, _) | TypeOfExpr(_) => 4,
			BinOpExpr(op, _, _) => op.get_precedence(),
			IfExpr(_, _, _) => 15,
			// 16 should be yield
			AssignExpr(_, _) => 17,
			_ => 19
		}
	}
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
			BinOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
			UnaryOpExpr(ref op, ref a) => write!(f, "{}{}", op, a),
			ReturnExpr(Some(ref ex)) => write!(f, "return {}", ex),
			ReturnExpr(None) => write!(f, "{}", "return"),
			ThrowExpr(ref ex) => write!(f, "throw {}", ex),
			AssignExpr(ref ref_e, ref val) => write!(f, "{} = {}", ref_e, val),
			TypeOfExpr(ref e) => write!(f, "typeof {}", e)
		}
	}
}

impl fmt::Show for TreeMap<StrBuf, Expr> {
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
impl fmt::Show for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}
#[deriving(Clone)]
#[deriving(Eq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum TokenData {
	/// A string literal
	TString(StrBuf),
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
	TIdent(StrBuf),
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
	TComment(StrBuf)
}
impl fmt::Show for TokenData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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