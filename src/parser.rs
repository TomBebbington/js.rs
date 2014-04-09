use ast::{Token, Expr};
use ast::{BlockExpr, ConstExpr, ThrowExpr, ReturnExpr, CallExpr, ConstructExpr, IfExpr, WhileLoopExpr, SwitchExpr, FunctionDeclExpr, LocalExpr, ArrayDeclExpr, ObjectDeclExpr, GetConstFieldExpr, GetFieldExpr, NumOpExpr, ConstExpr};
use ast::{CBool, CNull, CUndefined, CString, CNum};
use ast::{TIdent, TNumber, TString, TSemicolon, TColon, TDot, TEqual, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TNumOp};
use collections::treemap::TreeMap;
use std::fmt;
use std::vec::Vec;
#[deriving(Clone)]
#[deriving(Eq)]
/// An error encountered during parsing an expression
pub enum ParseError {
	/// When it expected a certain kind of token, but got another
	Expected(Vec<Token>, Token),
	/// When it expected a certain expression, but got another
	ExpectedExpr(~str, Expr)
}
impl fmt::Show for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			Expected(ref wanted, ref got) if wanted.len() == 0 => write!(f.buf, "Expected expression, got {}", got),
			Expected(ref wanted, ref got) => {
				try!(write!(f.buf, "Expected "));
				let last = wanted.last().unwrap();
				for wanted_token in wanted.iter() {
					match write!(f.buf, "'{}'{}", wanted_token, if wanted_token == last {""} else {", "}) {
						Err(err) => return Err(err),
						Ok(_) => ()
					}
				}
				write!(f.buf, ", got {}", got)
			},
			ExpectedExpr(ref wanted, ref got) => {
				write!(f.buf, "Expected {}, got {}", wanted, got)
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
		return Ok(~BlockExpr(exprs));
	}
	fn parse_struct(&mut self, name:&str) -> ParseStructResult {
		match name {
			"true" => Ok(Some(~ConstExpr(CBool(true)))),
			"false" => Ok(Some(~ConstExpr(CBool(false)))),
			"null" => Ok(Some(~ConstExpr(CNull))),
			"undefined" => Ok(Some(~ConstExpr(CUndefined))),
			"throw" => {
				let thrown = try!(self.parse());
				Ok(Some(~ThrowExpr(thrown)))
			},
			"return" => Ok(Some(~ReturnExpr(Some(try!(self.parse()))))),
			"new" => {
				let call = try!(self.parse());
				print!("new {}", call);
				match *call {
					CallExpr(ref func, ref args) => Ok(Some(~ConstructExpr(func.clone(), args.clone()))),
					_ => Err(ExpectedExpr(~"constructor", *call))
				}
			}
			"if" => {
				try!(self.expect(TOpenParen));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen));
				let expr = try!(self.parse());
				let next = self.tokens.get(self.pos + 1).clone();
				Ok(Some(~IfExpr(cond, expr, if next == TIdent(~"else") {
					self.pos += 2;
					Some(try!(self.parse()))
				} else {
					None
				})))
			},
			"while" => {
				try!(self.expect(TOpenParen));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen));
				let expr = try!(self.parse());
				Ok(Some(~WhileLoopExpr(cond, expr)))
			},
			"switch" => {
				try!(self.expect(TOpenParen));
				let value = self.parse();
				try!(self.expect(TCloseParen));
				try!(self.expect(TOpenBlock));
				let mut cases = Vec::new();
				let mut default = None;
				while self.pos + 1 < self.tokens.len() {
					let tok = self.tokens.get(self.pos).clone();
					self.pos += 1;
					match tok {
						TIdent(ref id) if *id == ~"case" => {
							let cond = self.parse();
							let mut block = Vec::new();
							try!(self.expect(TColon));
							loop {
								match self.tokens.get(self.pos).clone() {
									TIdent(ref id) if *id == ~"case" || *id == ~"default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							cases.push((cond.unwrap(), block));
						},
						TIdent(ref id) if *id == ~"default" => {
							let mut block = Vec::new();
							try!(self.expect(TColon));
							loop {
								match self.tokens.get(self.pos).clone() {
									TIdent(ref id) if *id == ~"case" || *id == ~"default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							default = Some(~BlockExpr(block));
						},
						TCloseBlock => break,
						_ => fail!("Expected 'case', 'default' or '{}', not '{}'", '}', tok)
					}
				}
				try!(self.expect(TCloseBlock));
				Ok(Some(~SwitchExpr(value.unwrap(), cases, default)))
			},
			"function" => {
				let tk = self.tokens.get(self.pos).clone();
				let name : Option<~str> = match tk {
					TIdent(ref name) => {
						self.pos += 1;
						Some(name.clone())
					},
					TOpenParen => None,
					_ => return Err(Expected(vec!(TIdent(~"identifier")), tk.clone()))
				};
				try!(self.expect(TOpenParen));
				let mut args:Vec<~str> = Vec::new();
				let mut tk = self.tokens.get(self.pos).clone();
				while tk != TCloseParen {
					match tk {
						TIdent(ref id) => args.push(id.to_owned()),
						_ => return Err(Expected(vec!(TIdent(~"identifier")), tk))
					}
					self.pos += 1;
					if *self.tokens.get(self.pos) == TComma {
						self.pos += 1;
					}
					tk = self.tokens.get(self.pos).clone();
				}
				self.pos += 1;
				let block = try!(self.parse());
				Ok(Some(~FunctionDeclExpr(name, args, block)))
			},
			_ => Ok(None)
		}
	}
	/// Parse a single expression
	pub fn parse(&mut self) -> ParseResult {
		let token = self.tokens.get(self.pos).clone();
		self.pos += 1;
		let expr : ~Expr = match token {
			TSemicolon => try!(self.parse()),
			TIdent(ref s) => {
				let structure = try!(self.parse_struct(s.clone()));
				match structure {
					Some(v) => v,
					None => ~LocalExpr(s.to_owned())
				}
			},
			TString(ref s) => ~ConstExpr(CString(s.to_owned())),
			TOpenParen => {
				let nexte = try!(self.parse());
				try!(self.expect(TCloseParen));
				nexte
			},
			TOpenArray => {
				let mut array = Vec::new();
				let mut expect_comma_or_end = *self.tokens.get(self.pos) == TCloseArray;
				loop {
					let token = self.tokens.get(self.pos).clone();
					if token == TCloseArray && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if token == TComma && !expect_comma_or_end {
						array.push(~ConstExpr(CNull));
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseArray), token));
					} else {
						let parsed = try!(self.parse());
						self.pos -= 1;
						array.push(parsed);
						expect_comma_or_end = true;
					}
					self.pos += 1;
				}
				~ArrayDeclExpr(array)
			},
			TOpenBlock if self.tokens.get(self.pos) == &TCloseBlock => {
				self.pos += 1;
				~ObjectDeclExpr(~TreeMap::new())
			},
			TOpenBlock if self.tokens.get(self.pos + 1) == &TColon => {
				let mut map = ~TreeMap::new();
				while self.tokens.get(self.pos - 1) == &TComma || map.len() == 0 {
					let tk = self.tokens.get(self.pos).clone();
					let name = match tk {
						TIdent(ref id) => id,
						TString(ref str) => str,
						_ => return Err(Expected(vec!(TIdent(~"identifier"), TString(~"string")), tk))
					};
					self.pos += 1;
					try!(self.expect(TColon));
					let value = try!(self.parse());
					map.insert(name.to_owned(), value);
					self.pos += 1;
				}
				~ObjectDeclExpr(map)
			},
			TOpenBlock => {
				let mut exprs = Vec::new();
				loop {
					if *self.tokens.get(self.pos) == TCloseBlock {
						break;
					} else {
						exprs.push(try!(self.parse()));
					}
				}
				self.pos += 1;
				~BlockExpr(exprs)
			},
			TNumber(num) => ~ConstExpr(CNum(num)),
			_ => return Err(Expected(Vec::new(), token.clone()))
		};
		return if self.pos >= self.tokens.len() { Ok(expr) } else {self.parse_next(expr)};
	}
	fn parse_next(&mut self, expr:~Expr) -> ParseResult {
		let next = self.tokens.get(self.pos).clone();
		let mut carry_on = true;
		let mut result = expr.clone();
		match next {
			TDot => {
				self.pos += 1;
				let tk = self.tokens.get(self.pos).clone();
				match tk {
					TIdent(ref s) => result = ~GetConstFieldExpr(expr, s.to_owned()),
					_ => return Err(Expected(vec!(TIdent(~"identifier")), next))
				}
				self.pos += 1;
			},
			TOpenParen => {
				let mut args = Vec::new();
				let mut expect_comma_or_end = *self.tokens.get(self.pos + 1) == TCloseParen;
				loop {
					self.pos += 1;
					let token = self.tokens.get(self.pos).clone();
					if token == TCloseParen && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseParen), token));
					} else {
						let parsed = try!(self.parse());
						self.pos -= 1;
						args.push(parsed);
						expect_comma_or_end = true;
					}
				}
				result = ~CallExpr(expr, args);
			},
			TQuestion => {
				self.pos += 1;
				let if_e = try!(self.parse());
				try!(self.expect(TColon));
				let else_e = try!(self.parse());
				result = ~IfExpr(expr, if_e, Some(else_e));
			},
			TOpenArray => {
				self.pos += 1;
				let index = try!(self.parse());
				try!(self.expect(TCloseArray));
				result = ~GetFieldExpr(expr, index);
			},
			TSemicolon => {
				self.pos += 1;
			},
			TNumOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				result = ~NumOpExpr(op, expr, next);
			},
			_ => carry_on = false
		};
		return if carry_on && self.pos < self.tokens.len() { self.parse_next(result) } else {Ok(result)};
	}
	/// Returns an error if the next symbol is not [tk]
	fn expect(&mut self, tk:Token) -> Result<(), ParseError> {
		self.pos += 1;
		let curr_tk = self.tokens.get(self.pos - 1).clone();
		return if curr_tk != tk {
			Err(Expected(vec!(tk), curr_tk))
		} else {
			Ok(())
		};
	}
}