use ast::{Token, TokenData, Expr};
use ast::{BlockExpr, ThrowExpr, ReturnExpr, CallExpr, ConstructExpr, IfExpr, WhileLoopExpr, SwitchExpr, TypeOfExpr, FunctionDeclExpr, ArrowFunctionDeclExpr, LocalExpr, ArrayDeclExpr, ObjectDeclExpr, GetConstFieldExpr, GetFieldExpr, BinOpExpr, UnaryOpExpr, ConstExpr, AssignExpr};
use ast::{CBool, CNull, CUndefined, CString, CNum};
use ast::{TIdent, TNumber, TString, TSemicolon, TColon, TComment, TDot, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TUnaryOp, TEqual, TArrow, TAssignOp, TBinOp};
use ast::{OpSub, UnaryMinus, UnaryNot};
use ast::{BinNum, GetPrecedence};
use collections::treemap::TreeMap;
use std::fmt;
use std::vec::Vec;
macro_rules! mk (
	($def:expr) => (
		Expr::new($def, try!(self.get_token(self.pos - 1)).pos, try!(self.get_token(self.pos - 1)).pos)
	);
	($def:expr, $first:expr) => (
		Expr::new($def, $first.pos, try!(self.get_token(self.pos - 1)).pos)
	);
)
#[deriving(Clone)]
#[deriving(Eq)]
/// An error encountered during parsing an expression
pub enum ParseError {
	/// When it expected a certain kind of token, but got another as part of something
	Expected(Vec<TokenData>, Token, &'static str),
	/// When it expected a certain expression, but got another
	ExpectedExpr(&'static str, Expr),
	/// When there is an abrupt end to the parsing
	AbruptEnd
}
impl fmt::Show for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match *self {
			Expected(ref wanted, ref got, ref routine) if wanted.len() == 0 => write!(f, "{}:{}: Expected expression for {}, got {}", got.pos.line_number, got.pos.column_number, routine, got.data),
			Expected(ref wanted, ref got, ref routine) => {
				try!(write!(f, "{}:{}: ", got.pos.line_number, got.pos.column_number));
				try!(write!(f, "Expected "));
				let last = wanted.last().unwrap();
				for wanted_token in wanted.iter() {
					try!(write!(f, "'{}'{}", wanted_token, if wanted_token == last {""} else {", "}));
				}
				try!(write!(f, " for {}", routine));
				write!(f, " but got {}", got.data)
			},
			ExpectedExpr(ref wanted, ref got) => {
				write!(f, "Expected {}, but got {}", wanted, got)
			},
			AbruptEnd => {
				write!(f, "Abrupt end")
			}
		}
	}
}
pub type ParseResult = Result<Expr, ParseError>;
pub type ParseStructResult = Result<Option<Expr>, ParseError>;
/// A Javascript parser
pub struct Parser {
	/// The tokens being input
	tokens: Vec<Token>,
	/// The current position within the tokens
	pos: uint
}
impl Parser {
	/// Creates a new parser, using `tokens` as input
	pub fn new(tokens: Vec<Token>) -> Parser {
		Parser {tokens: tokens, pos: 0}
	}
	/// Parse all expressions in the token array
	pub fn parse_all(&mut self) -> ParseResult {
		let mut exprs = Vec::new();
		while self.pos < self.tokens.len() {
			let result = try!(self.parse());
			exprs.push(result);
		}
		Ok(mk!(BlockExpr(exprs)))
	}
	fn parse_struct(&mut self, name:&str) -> ParseStructResult {
		match name {
			"true" => Ok(Some(mk!(ConstExpr(CBool(true))))),
			"false" => Ok(Some(mk!(ConstExpr(CBool(false))))),
			"null" => Ok(Some(mk!(ConstExpr(CNull)))),
			"undefined" => Ok(Some(mk!(ConstExpr(CUndefined)))),
			"throw" => {
				let thrown = try!(self.parse());
				Ok(Some(mk!(ThrowExpr(box thrown))))
			},
			"return" => Ok(Some(mk!(ReturnExpr(Some(box try!(self.parse()).clone()))))),
			"new" => {
				let call = try!(self.parse());
				match call.def {
					CallExpr(ref func, ref args) => Ok(Some(mk!(ConstructExpr(func.clone(), args.clone())))),
					_ => Err(ExpectedExpr("constructor", call))
				}
			},
			"typeof" => Ok(Some(mk!(TypeOfExpr(box try!(self.parse()))))),
			"if" => {
				try!(self.expect(TOpenParen, "if block"));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen, "if block"));
				let expr = try!(self.parse());
				let next = try!(self.get_token(self.pos + 1));
				Ok(Some(mk!(IfExpr(box cond, box expr, if next.data == TIdent("else".to_strbuf()) {
					self.pos += 2;
					Some(box try!(self.parse()))
				} else {
					None
				}))))
			},
			"while" => {
				try!(self.expect(TOpenParen, "while condition"));
				let cond = try!(self.parse());
				try!(self.expect(TCloseParen, "while condition"));
				let expr = try!(self.parse());
				Ok(Some(mk!(WhileLoopExpr(box cond, box expr))))
			},
			"switch" => {
				try!(self.expect(TOpenParen, "switch value"));
				let value = self.parse();
				try!(self.expect(TCloseParen, "switch value"));
				try!(self.expect(TOpenBlock, "switch block"));
				let mut cases = Vec::new();
				let mut default = None;
				while self.pos + 1 < self.tokens.len() {
					let tok = try!(self.get_token(self.pos));
					self.pos += 1;
					match tok.data {
						TIdent(ref id) if id.as_slice() == "case" => {
							let cond = self.parse();
							let mut block = Vec::new();
							try!(self.expect(TColon, "switch case"));
							loop {
								match try!(self.get_token(self.pos)).data.clone() {
									TIdent(ref id) if id.as_slice() == "case" || id.as_slice() == "default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							cases.push((cond.unwrap(), block));
						},
						TIdent(ref id) if id.as_slice() == "default" => {
							let mut block = Vec::new();
							try!(self.expect(TColon, "default switch case"));
							loop {
								match try!(self.get_token(self.pos)).data.clone() {
									TIdent(ref id) if id.as_slice() == "case" || id.as_slice() == "default" => break,
									TCloseBlock => break,
									_ => block.push(try!(self.parse()))
								}
							}
							default = Some(mk!(BlockExpr(block)));
						},
						TCloseBlock => break,
						_ => return Err(Expected(vec!(TIdent("case".to_strbuf()), TIdent("default".to_strbuf()), TCloseBlock), tok, "switch block"))
					}
				}
				try!(self.expect(TCloseBlock, "switch block"));
				Ok(Some(mk!(SwitchExpr(box value.unwrap(), cases, match default {
					Some(v) => Some(box v),
					None => None
				}))))
			},
			"function" => {
				let tk = try!(self.get_token(self.pos));
				let name = match tk.data {
					TIdent(ref name) => {
						self.pos += 1;
						Some(name.clone())
					},
					TOpenParen => None,
					_ => return Err(Expected(vec!(TIdent("identifier".to_strbuf())), tk.clone(), "function name"))
				};
				try!(self.expect(TOpenParen, "function"));
				let mut args:Vec<StrBuf> = Vec::new();
				let mut tk = try!(self.get_token(self.pos));
				while tk.data != TCloseParen {
					match tk.data {
						TIdent(ref id) => args.push(id.clone()),
						_ => return Err(Expected(vec!(TIdent("identifier".to_strbuf())), tk.clone(), "function arguments"))
					}
					self.pos += 1;
					if try!(self.get_token(self.pos)).data == TComma {
						self.pos += 1;
					}
					tk = try!(self.get_token(self.pos));
				}
				self.pos += 1;
				let block = try!(self.parse());
				Ok(Some(mk!(FunctionDeclExpr(name, args, box block))))
			},
			_ => Ok(None)
		}
	}
	/// Parse a single expression
	pub fn parse(&mut self) -> ParseResult {
		if self.pos > self.tokens.len() {
			return Err(AbruptEnd);
		}
		let token = try!(self.get_token(self.pos));
		self.pos += 1;
		let expr : Expr = match token.data {
			TSemicolon | TComment(_) if self.pos < self.tokens.len() => try!(self.parse()),
			TSemicolon | TComment(_) => mk!(ConstExpr(CUndefined)),
			TIdent(ref s) => {
				let structure = try!(self.parse_struct(s.as_slice()));
				match structure {
					Some(v) => v,
					None => mk!(LocalExpr(s.clone()))
				}
			},
			TString(ref s) => mk!(ConstExpr(CString(s.clone()))),
			TOpenParen => {
				match try!(self.get_token(self.pos)).data.clone() {
					TCloseParen if try!(self.get_token(self.pos + 1)).data == TArrow => {
						self.pos += 2;
						let expr = try!(self.parse());
						mk!(ArrowFunctionDeclExpr(Vec::new(), box expr), token)
					},
					_ => {
						let next = try!(self.parse());
						let next_tok = try!(self.get_token(self.pos));
						self.pos += 1;
						match next_tok.data.clone() {
							TCloseParen => next,
							TComma => { // at this point it's probably gonna be an arrow function
								let mut args = vec!(match next.def {
									LocalExpr(name) => name,
									_ => "".to_strbuf()
								}, match try!(self.get_token(self.pos)).data {
									TIdent(ref id) => id.clone(),
									_ => "".to_strbuf()
								});
								let mut expect_ident = true;
								loop {
									self.pos += 1;
									let curr_tk = try!(self.get_token(self.pos));
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
										_ if expect_ident => return Err(Expected(vec!(TIdent("identifier".to_strbuf())), curr_tk, "arrow function")),
										_ => return Err(Expected(vec!(TComma, TCloseParen), curr_tk, "arrow function"))
									}
								}
								try!(self.expect(TArrow, "arrow function"));
								let expr = try!(self.parse());
								mk!(ArrowFunctionDeclExpr(args, box expr), token)
							}
							_ => return Err(Expected(vec!(TCloseParen), next_tok, "brackets"))
						}
					}
				}
			},
			TOpenArray => {
				let mut array : Vec<Expr> = Vec::new();
				let mut expect_comma_or_end = try!(self.get_token(self.pos)).data == TCloseArray;
				loop {
					let token = try!(self.get_token(self.pos));
					if token.data == TCloseArray && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token.data == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if token.data == TComma && !expect_comma_or_end {
						array.push(mk!(ConstExpr(CNull)));
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseArray), token.clone(), "array declaration"));
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
			TOpenBlock if try!(self.get_token(self.pos)).data == TCloseBlock => {
				self.pos += 1;
				mk!(ObjectDeclExpr(box TreeMap::new()), token)
			},
			TOpenBlock if try!(self.get_token(self.pos + 1)).data == TColon => {
				let mut map = box TreeMap::new();
				while try!(self.get_token(self.pos - 1)).data == TComma || map.len() == 0 {
					let tk = try!(self.get_token(self.pos));
					let name = match tk.data {
						TIdent(ref id) => id.clone(),
						TString(ref str) => str.clone(),
						_ => return Err(Expected(vec!(TIdent("identifier".to_strbuf()), TString("string".to_strbuf())), tk, "object declaration"))
					};
					self.pos += 1;
					try!(self.expect(TColon, "object declaration"));
					let value = try!(self.parse());
					map.insert(name, value);
					self.pos += 1;
				}
				mk!(ObjectDeclExpr(map), token)
			},
			TOpenBlock => {
				let mut exprs = Vec::new();
				loop {
					if try!(self.get_token(self.pos)).data == TCloseBlock {
						break;
					} else {
						exprs.push(try!(self.parse()));
					}
				}
				self.pos += 1;
				mk!(BlockExpr(exprs), token)
			},
			TNumber(num) =>
				mk!(ConstExpr(CNum(num))),
			TBinOp(BinNum(OpSub)) =>
				mk!(UnaryOpExpr(UnaryMinus, box try!(self.parse()))),
			TUnaryOp(UnaryNot) =>
				mk!(UnaryOpExpr(UnaryNot, box try!(self.parse()))),
			_ => return Err(Expected(Vec::new(), token.clone(), "script"))
		};
		if self.pos >= self.tokens.len() {
			Ok(expr)
		} else {
			self.parse_next(expr)
		}
	}
	fn get_token(&self, pos:uint) -> Result<Token, ParseError> {
		if pos < self.tokens.len() {
			Ok(self.tokens.get(pos).clone())
		} else {
			Err(AbruptEnd)
		}
	}
	fn parse_next(&mut self, expr:Expr) -> ParseResult {
		let next = try!(self.get_token(self.pos));
		let mut carry_on = true;
		let mut result = expr.clone();
		match next.data {
			TDot => {
				self.pos += 1;
				let tk = try!(self.get_token(self.pos));
				match tk.data {
					TIdent(ref s) => result = mk!(GetConstFieldExpr(box expr, s.to_strbuf())),
					_ => return Err(Expected(vec!(TIdent("identifier".to_strbuf())), tk, "field access"))
				}
				self.pos += 1;
			},
			TOpenParen => {
				let mut args = Vec::new();
				let mut expect_comma_or_end = try!(self.get_token(self.pos + 1)).data == TCloseParen;
				loop {
					self.pos += 1;
					let token = try!(self.get_token(self.pos));
					if token.data == TCloseParen && expect_comma_or_end {
						self.pos += 1;
						break;
					} else if token.data == TComma && expect_comma_or_end {
						expect_comma_or_end = false;
					} else if expect_comma_or_end {
						return Err(Expected(vec!(TComma, TCloseParen), token, "function call arguments"));
					} else {
						let parsed = try!(self.parse());
						self.pos -= 1;
						args.push(parsed);
						expect_comma_or_end = true;
					}
				}
				result = mk!(CallExpr(box expr, args));
			},
			TQuestion => {
				self.pos += 1;
				let if_e = try!(self.parse());
				try!(self.expect(TColon, "if expression"));
				let else_e = try!(self.parse());
				result = mk!(IfExpr(box expr, box if_e, Some(box else_e)));
			},
			TOpenArray => {
				self.pos += 1;
				let index = try!(self.parse());
				try!(self.expect(TCloseArray, "array declaration"));
				result = mk!(GetFieldExpr(box expr, box index));
			},
			TSemicolon | TComment(_) => {
				self.pos += 1;
			},
			TBinOp(op) => {
				let precedence = op.get_precedence();
				self.pos += 1;
				let next = try!(self.parse());
				result = match (next.clone()).def {
					BinOpExpr(ref op2, ref a, ref b) if precedence <= op2.get_precedence() =>
						mk!(BinOpExpr(*op2, b.clone(), box mk!(BinOpExpr(op.clone(), box expr, a.clone())))),
					_ => mk!(BinOpExpr(op, box expr, box next))
				}
			},
			TAssignOp(op) => {
				self.pos += 1;
				let next = try!(self.parse());
				let boxed = box expr;
				let op_result = mk!(BinOpExpr(op, boxed.clone(), box next));
				result = mk!(AssignExpr(boxed, box op_result))
			},
			TEqual => {
				self.pos += 1;
				let next = try!(self.parse());
				result = mk!(AssignExpr(box expr, box next));
			},
			TArrow => {
				self.pos += 1;
				let mut args = Vec::with_capacity(1);
				match result.def {
					LocalExpr(name) => args.push(name),
					_ => return Err(ExpectedExpr("identifier", result))
				}
				let next = try!(self.parse());
				result = mk!(ArrowFunctionDeclExpr(args, box next));
			},
			_ => carry_on = false
		};
		if carry_on && self.pos < self.tokens.len() {
			self.parse_next(result)
		} else {
			Ok(result)
		}
	}
	/// Returns an error if the next symbol is not `tk`
	fn expect(&mut self, tk:TokenData, routine:&'static str) -> Result<(), ParseError> {
		self.pos += 1;
		let curr_tk = try!(self.get_token(self.pos - 1));
		if curr_tk.data != tk {
			Err(Expected(vec!(tk), curr_tk, routine))
		} else {
			Ok(())
		}
	}
}