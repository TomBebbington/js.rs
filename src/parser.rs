use ast::{Token, TokenData, Expr};
use ast::{BlockExpr, ThrowExpr, ReturnExpr, CallExpr, ConstructExpr, IfExpr, WhileLoopExpr, SwitchExpr, TypeOfExpr, FunctionDeclExpr, ArrowFunctionDeclExpr, LocalExpr, ArrayDeclExpr, ObjectDeclExpr, GetConstFieldExpr, GetFieldExpr, NumOpExpr, BitOpExpr, CompOpExpr, LogOpExpr, ConstExpr, AssignExpr};
use ast::{CBool, CNull, CUndefined, CString, CNum};
use ast::{TIdent, TNumber, TString, TSemicolon, TColon, TComment, TDot, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TNumOp, TBitOp, TCompOp, TLogOp, TEqual, TArrow};
use collections::treemap::TreeMap;
use std::fmt;
use std::vec::Vec;
macro_rules! mk (
	($def:expr) => (
		Expr::new($def, self.tokens.get(self.pos).pos, self.tokens.get(self.pos).pos)
	);
	($def:expr, $first:expr) => (
		Expr::new($def, $first.pos, self.tokens.get(self.pos).pos)
	);
)
#[deriving(Clone)]
#[deriving(Eq)]
/// An error encountered during parsing an expression
pub enum ParseError {
	/// When it expected a certain kind of token, but got another as part of something
	Expected(Vec<TokenData>, Token, ~str),
	/// When it expected a certain expression, but got another
	ExpectedExpr(~str, Expr)
}
impl fmt::Show for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			Expected(ref wanted, ref got, ref routine) if wanted.len() == 0 => write!(f.buf, "{}:{}: Expected expression for {}, got {}", got.pos.line_number, got.pos.column_number, routine, got.data),
			Expected(ref wanted, ref got, ref routine) => {
				try!(write!(f.buf, "{}:{}: ", got.pos.line_number, got.pos.column_number));
				try!(write!(f.buf, "Expected "));
				let last = wanted.last().unwrap();
				for wanted_token in wanted.iter() {
					try!(write!(f.buf, "'{}'{}", wanted_token, if wanted_token == last {""} else {", "}));
				}
				try!(write!(f.buf, " for {}", routine));
				write!(f.buf, " but got {}", got.data)
			},
			ExpectedExpr(ref wanted, ref got) => {
				write!(f.buf, "Expected {}, but got {}", wanted, got)
			}
		}
	}
}
/// A result which gives a pretty-printed error if it fails
pub trait VerboseResult<T> {
	/// Unwrap the value if it's Ok and if not pretty-print the error
	fn v_unwrap(&self) -> T;
}
impl<T : Clone, E : fmt::Show> VerboseResult<T> for Result<T, E> {
	fn v_unwrap(&self) -> T {
		match *self {
			Ok(ref val) => val.clone(),
			Err(ref err) => fail!("{}", err)
		}
	}
}
pub type ParseResult = Result<~Expr, ParseError>;
pub type ParseStructResult = Result<Option<~Expr>, ParseError>;
/// A Javascript parser
pub struct Parser {
	/// The tokens being input
	tokens: Vec<Token>,
	/// The current position within the tokens
	pos: uint
}
impl Parser {
	/// Creates a new parser, using [tokens] as input
	pub fn new(tokens: Vec<Token>) -> ~Parser {
		return ~Parser {tokens: tokens, pos: 0};
	}
	/// Parse all expressions in the token array
	pub fn parse_all(&mut self) -> ParseResult {
		let mut exprs = Vec::new();
		while self.pos < self.tokens.len() {
			let result = try!(self.parse());
			exprs.push(result);
		}
		return Ok(mk!(BlockExpr(exprs)));
	}
	fn parse_struct(&mut self, name:&str) -> ParseStructResult {
		match name {
			"true" => Ok(Some(mk!(ConstExpr(CBool(true))))),
			"false" => Ok(Some(mk!(ConstExpr(CBool(false))))),
			"null" => Ok(Some(mk!(ConstExpr(CNull)))),
			"undefined" => Ok(Some(mk!(ConstExpr(CUndefined)))),
			"throw" => {
				let thrown = try!(self.parse());
				Ok(Some(mk!(ThrowExpr(thrown))))
			},
			"return" => Ok(Some(mk!(ReturnExpr(Some(try!(self.parse()).clone()))))),
			"new" => {
				let call = try!(self.parse());
				match call.def {
					CallExpr(ref func, ref args) => Ok(Some(mk!(ConstructExpr(func.clone(), args.clone())))),
					_ => Err(ExpectedExpr(~"constructor", *call))
				}
			},
			"typeof" => Ok(Some(mk!(TypeOfExpr(try!(self.parse()))))),
			"if" => {
				try!(self.expect(TOpenParen, ~"if block"));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen, ~"if block"));
				let expr = try!(self.parse());
				let next = self.tokens.get(self.pos + 1).clone();
				Ok(Some(mk!(IfExpr(cond, expr, if next.data == TIdent(~"else") {
					self.pos += 2;
					Some(try!(self.parse()))
				} else {
					None
				}))))
			},
			"while" => {
				try!(self.expect(TOpenParen, ~"while condition"));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen, ~"while condition"));
				let expr = try!(self.parse());
				Ok(Some(mk!(WhileLoopExpr(cond, expr))))
			},
			"switch" => {
				try!(self.expect(TOpenParen, ~"switch value"));
				let value = self.parse();
				try!(self.expect(TCloseParen, ~"switch value"));
				try!(self.expect(TOpenBlock, ~"switch block"));
				let mut cases = Vec::new();
				let mut default = None;
				while self.pos + 1 < self.tokens.len() {
					let tok = self.tokens.get(self.pos).clone();
					self.pos += 1;
					match tok.data {
						TIdent(ref id) if *id == ~"case" => {
							let cond = self.parse();
							let mut block = Vec::new();
							try!(self.expect(TColon, ~"switch case"));
							loop {
								match self.tokens.get(self.pos).data.clone() {
									TIdent(ref id) if *id == ~"case" || *id == ~"default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							cases.push((cond.unwrap(), block));
						},
						TIdent(ref id) if *id == ~"default" => {
							let mut block = Vec::new();
							try!(self.expect(TColon, ~"default switch case"));
							loop {
								match self.tokens.get(self.pos).data.clone() {
									TIdent(ref id) if *id == ~"case" || *id == ~"default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							default = Some(mk!(BlockExpr(block)));
						},
						TCloseBlock => break,
						_ => return Err(Expected(vec!(TIdent(~"case"), TIdent(~"default"), TCloseBlock), tok, ~"switch block"))
					}
				}
				try!(self.expect(TCloseBlock, ~"switch block"));
				Ok(Some(mk!(SwitchExpr(value.unwrap(), cases, default))))
			},
			"function" => {
				let tk = self.tokens.get(self.pos).clone();
				let name : Option<~str> = match tk.data {
					TIdent(ref name) => {
						self.pos += 1;
						Some(name.clone())
					},
					TOpenParen => None,
					_ => return Err(Expected(vec!(TIdent(~"identifier")), tk.clone(), ~"function name"))
				};
				try!(self.expect(TOpenParen, ~"function"));
				let mut args:Vec<~str> = Vec::new();
				let mut tk = self.tokens.get(self.pos).clone();
				while tk.data != TCloseParen {
					match tk.data {
						TIdent(ref id) => args.push(id.to_owned()),
						_ => return Err(Expected(vec!(TIdent(~"identifier")), tk.clone(), ~"function arguments"))
					}
					self.pos += 1;
					if self.tokens.get(self.pos).data == TComma {
						self.pos += 1;
					}
					tk = self.tokens.get(self.pos).clone();
				}
				self.pos += 1;
				let block = try!(self.parse());
				Ok(Some(mk!(FunctionDeclExpr(name, args, block))))
			},
			_ => Ok(None)
		}
	}
	/// Parse a single expression
	pub fn parse(&mut self) -> ParseResult {
		let token = self.tokens.get(self.pos).clone();
		self.pos += 1;
		let expr : ~Expr = match token.data {
			TSemicolon | TComment(_) if self.pos < self.tokens.len() => try!(self.parse()),
			TSemicolon | TComment(_) => mk!(ConstExpr(CUndefined)),
			TIdent(ref s) => {
				let structure = try!(self.parse_struct(s.clone()));
				match structure {
					Some(v) => v,
					None => mk!(LocalExpr(s.to_owned()))
				}
			},
			TString(ref s) => mk!(ConstExpr(CString(s.to_owned()))),
			TOpenParen => {
				match self.tokens.get(self.pos).data.clone() {
					TCloseParen if self.tokens.get(self.pos + 1).data == TArrow => {
						self.pos += 2;
						let expr = try!(self.parse());
						mk!(ArrowFunctionDeclExpr(Vec::new(), expr), token)
					},
					_ => {
						let next = try!(self.parse());
						let next_tok = self.tokens.get(self.pos).clone();
						self.pos += 1;
						match next_tok.data.clone() {
							TCloseParen => next,
							TComma => { // at this point it's probably gonna be an arrow function
								let mut args = vec!(match next.def {
									LocalExpr(name) => name,
									_ => ~""
								}, match self.tokens.get(self.pos).data {
									TIdent(ref id) => id.clone(),
									_ => ~""
								});
								let mut expect_ident = true;
								loop {
									self.pos += 1;
									let curr_tk = self.tokens.get(self.pos).clone();
									match curr_tk.data {
										TIdent(ref id) if expect_ident => {
											args.push(id.clone());
											expect_ident = false;
										},
										TComma => {
											expect_ident = true;
										},
										TCloseParen => {
											self.pos += 1;
											break;
										},
										_ if expect_ident => return Err(Expected(vec!(TIdent(~"identifier")), curr_tk, ~"arrow function")),
										_ => return Err(Expected(vec!(TComma, TCloseParen), curr_tk, ~"arrow function"))
									}
								}
								try!(self.expect(TArrow, ~"arrow function"));
								let expr = try!(self.parse());
								mk!(ArrowFunctionDeclExpr(args, expr), token)
							}
							_ => return Err(Expected(vec!(TCloseParen), next_tok, ~"brackets"))
						}
					}
				}
			},
			TOpenArray => {
				let mut array : Vec<~Expr> = Vec::new();
				let mut expect_comma_or_end = self.tokens.get(self.pos).data == TCloseArray;
				loop {
					let token = self.tokens.get(self.pos).clone();
					if token.data == TCloseArray && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token.data == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if token.data == TComma && !expect_comma_or_end {
						array.push(mk!(ConstExpr(CNull)));
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseArray), token.clone(), ~"array declaration"));
					} else {
						let parsed = try!(self.parse());
						self.pos -= 1;
						array.push(parsed);
						expect_comma_or_end = true;
					}
					self.pos += 1;
				}
				mk!(ArrayDeclExpr(array), token)
			},
			TOpenBlock if self.tokens.get(self.pos).data == TCloseBlock => {
				self.pos += 1;
				mk!(ObjectDeclExpr(~TreeMap::new()), token)
			},
			TOpenBlock if self.tokens.get(self.pos + 1).data == TColon => {
				let mut map = ~TreeMap::new();
				while self.tokens.get(self.pos - 1).data == TComma || map.len() == 0 {
					let tk = self.tokens.get(self.pos).clone();
					let name = match tk.data {
						TIdent(ref id) => id,
						TString(ref str) => str,
						_ => return Err(Expected(vec!(TIdent(~"identifier"), TString(~"string")), tk, ~"object declaration"))
					};
					self.pos += 1;
					try!(self.expect(TColon, ~"object declaration"));
					let value = try!(self.parse());
					map.insert(name.to_owned(), value);
					self.pos += 1;
				}
				mk!(ObjectDeclExpr(map), token)
			},
			TOpenBlock => {
				let mut exprs = Vec::new();
				loop {
					if self.tokens.get(self.pos).data == TCloseBlock {
						break;
					} else {
						exprs.push(try!(self.parse()));
					}
				}
				self.pos += 1;
				mk!(BlockExpr(exprs), token)
			},
			TNumber(num) => mk!(ConstExpr(CNum(num)), token),
			_ => return Err(Expected(Vec::new(), token.clone(), ~"script"))
		};
		return if self.pos >= self.tokens.len() { Ok(expr) } else {self.parse_next(expr)};
	}
	fn parse_next(&mut self, expr:~Expr) -> ParseResult {
		let next = self.tokens.get(self.pos).clone();
		let mut carry_on = true;
		let mut result = expr.clone();
		match next.data {
			TDot => {
				self.pos += 1;
				let tk = self.tokens.get(self.pos).clone();
				match tk.data {
					TIdent(ref s) => result = mk!(GetConstFieldExpr(expr, s.to_owned())),
					_ => return Err(Expected(vec!(TIdent(~"identifier")), tk, ~"field access"))
				}
				self.pos += 1;
			},
			TOpenParen => {
				let mut args = Vec::new();
				let mut expect_comma_or_end = self.tokens.get(self.pos + 1).data == TCloseParen;
				loop {
					self.pos += 1;
					let token = self.tokens.get(self.pos).clone();
					if token.data == TCloseParen && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token.data == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseParen), token, ~"function call arguments"));
					} else {
						let parsed = try!(self.parse());
						self.pos -= 1;
						args.push(parsed);
						expect_comma_or_end = true;
					}
				}
				result = mk!(CallExpr(expr, args));
			},
			TQuestion => {
				self.pos += 1;
				let if_e = try!(self.parse());
				try!(self.expect(TColon, ~"if expression"));
				let else_e = try!(self.parse());
				result = mk!(IfExpr(expr, if_e, Some(else_e)));
			},
			TOpenArray => {
				self.pos += 1;
				let index = try!(self.parse());
				try!(self.expect(TCloseArray, ~"array declaration"));
				result = mk!(GetFieldExpr(expr, index));
			},
			TSemicolon | TComment(_) => {
				self.pos += 1;
			},
			TNumOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(NumOpExpr(op, expr, next));
			},
			TBitOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(BitOpExpr(op, expr, next));
			},
			TCompOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(CompOpExpr(op, expr, next));
			},
			TLogOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(LogOpExpr(op, expr, next));
			},
			TEqual => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(AssignExpr(expr, next));
			},
			TArrow => {
				self.pos += 1;
				let mut args = Vec::with_capacity(1);
				match result.def {
					LocalExpr(name) => args.push(name),
					_ => return Err(ExpectedExpr(~"identifier", *result))
				}
				let next = try!(self.parse());
				result = mk!(ArrowFunctionDeclExpr(args, next));
			},
			_ => carry_on = false
		};
		return if carry_on && self.pos < self.tokens.len() { self.parse_next(result) } else {Ok(result)};
	}
	/// Returns an error if the next symbol is not [tk]
	fn expect(&mut self, tk:TokenData, routine:~str) -> Result<(), ParseError> {
		self.pos += 1;
		let curr_tk = self.tokens.get(self.pos - 1).clone();
		return if curr_tk.data != tk {
			Err(Expected(vec!(tk), curr_tk, routine))
		} else {
			Ok(())
		};
	}
}