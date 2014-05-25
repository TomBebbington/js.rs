use ast::{TIdent, TNumber, TString, TSemicolon, TComment, TColon, TDot, TEqual, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TBinOp, TAssignOp, TUnaryOp, TArrow};
use ast::{OpAdd, OpSub, OpMul, OpDiv, OpMod};
use ast::{BitAnd, BitOr, BitXor, BitShl, BitShr};
use ast::{CompEqual, CompStrictEqual, CompNotEqual, CompStrictNotEqual, CompLessThan, CompGreaterThan, CompLessThanOrEqual, CompGreaterThanOrEqual};
use ast::{LogAnd, LogOr};
use ast::{UnaryNot};
use ast::{BinNum, BinBit, BinComp, BinLog};
use ast::{Token, TokenData};
use std::io::{BufReader, BufferedReader, Buffer, IoResult, EndOfFile};
use std::strbuf::StrBuf;
use std::char::from_u32;
use std::num::from_str_radix;
#[deriving(Clone, Eq)]
/// The type of string used
pub enum StringType {
	/// `"` - Double-quoted
	DoubleQuote,
	/// `'` - Single-quoted
	SingleQuote
}
#[deriving(Clone, Eq)]
/// The type of comment used
pub enum CommentType {
	/// `/*...*/` - A multi-line comment
	MultiLineComment,
	/// `//...` - A single-line comment
	SingleLineComment
}
#[deriving(Clone, Eq)]
/// The type of number used
pub enum NumberType {
	/// A decimal number, such as `3.1415`
	DecimalNumber,
	/// A hexadecimal number, such as `0xFF00FF`
	HexadecimalNumber,
	/// An octal number, such as `011`
	OctalNumber
}
/// A Javascript lexer
pub struct Lexer<B> {
	/// The list of tokens generated so far
	pub tokens : Vec<Token>,
	/// The current line number in the script
	line_number : uint,
	/// The current column number in the script
	column_number : uint,
	/// The reader
	buffer: B,
	/// The peeked character buffer
	peek_buffer: StrBuf
}
impl<B:Buffer> Lexer<B> {
	/// Creates a new lexer with empty buffers
	pub fn new(buffer: B) -> Lexer<B> {
		return Lexer {
			tokens: Vec::new(),
			line_number: 1,
			column_number: 0,
			buffer: buffer,
			peek_buffer: StrBuf::new()
		};
	}
	fn push_token(&mut self, tk:TokenData) {
		self.tokens.push(Token::new(tk, self.line_number, self.column_number))
	}
	/// Processes an input stream from a string into an array of tokens
	pub fn lex_str(script:&str) -> Vec<Token> {
		let script_bytes:&[u8] = script.as_bytes();
		let reader = BufReader::new(script_bytes);
		let buf_reader = BufferedReader::new(reader);
		let mut lexer = Lexer::new(buf_reader);
		lexer.lex().unwrap();
		lexer.tokens
	}
	fn next(&mut self) -> IoResult<char> {
		if self.peek_buffer.len() == 0 {
			self.buffer.read_char()
		} else {
			Ok(match self.peek_buffer.pop_char() {
				Some(v) => v,
				None => unreachable!()
			})
		}
	}
	fn peek_for(&mut self, peek:char) -> bool {
		if self.peek_buffer.len() > 0 {
			let matched = self.peek_buffer.as_slice().char_at(self.peek_buffer.len() - 1) == peek;
			if matched {
				self.peek_buffer.pop_char();
			}
			matched
		} else {
			match self.buffer.read_char() {
				Ok(ch) if ch == peek => {
					self.column_number += 1;
					true
				},
				Ok(ch) if ch != peek => {
					self.peek_buffer.push_char(ch);
					false
				},
				_ => false
			}
		}
	}
	/// Processes an input stream from the `buffer` into a vector of tokens
	pub fn lex(&mut self) -> IoResult<()> {
		loop {
			let ch = match self.next() {
				Ok(ch) => ch,
				Err(ref err) if err.kind == EndOfFile => break,
				Err(err) => return Err(err)
			};
			self.column_number += 1;
			match ch {
				'"' | '\'' => {
					let mut buf = StrBuf::new();
					loop {
						match try!(self.next()) {
							'\'' if ch == '\'' => {
								break;
							},
							'"' if ch == '"' => {
								break;
							},
							'\\' => {
								let escape = try!(self.next());
								if escape != '\n' {
									let escaped_ch = match escape {
										'n' => '\n',
										'r' => '\r',
										't' => '\t',
										'b' => '\x08',
										'f' => '\x0c',
										'0' => '\0',
										'x' => {
											let mut nums = StrBuf::with_capacity(2);
											for _ in range(0, 2) {
												nums.push_char(try!(self.next()));
											}
											self.column_number += 2;
											let as_num = match from_str_radix(nums.as_slice(), 16) {
												Some(v) => v,
												None => 0
											};
											match from_u32(as_num) {
												Some(v) => v,
												None => fail!("{}:{}: {} is not a valid unicode scalar value", self.line_number, self.column_number, as_num)
											}
										},
										'u' => {
											let mut nums = StrBuf::new();
											for _ in range(0, 4) {
												nums.push_char(try!(self.next()));
											}
											self.column_number += 4;
											let as_num = match from_str_radix(nums.as_slice(), 16) {
												Some(v) => v,
												None => 0
											};
											match from_u32(as_num) {
												Some(v) => v,
												None => fail!("{}:{}: {} is not a valid unicode scalar value", self.line_number, self.column_number, as_num)
											}
										},
										'\'' | '"' => escape,
										_ => fail!("{}:{}: Invalid escape `{}`", self.line_number, self.column_number, ch)
									};
									buf.push_char(escaped_ch);
								}
							},
							ch => buf.push_char(ch)
						}
					}
					self.push_token(TString(buf))
				},
				'0' if self.peek_for('x') => {
					let mut buf = StrBuf::new();
					loop {
						match try!(self.next()) {
							ch if ch.is_digit_radix(16) => buf.push_char(ch),
							ch => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(TNumber(from_str_radix(buf.as_slice(), 16).unwrap()));
				},
				'0' => {
					let mut buf = StrBuf::new();
					let mut gone_decimal = false;
					loop {
						let ch = try!(self.next());
						match ch {
							_ if ch.is_digit_radix(8) => buf.push_char(ch),
							'8' | '9' | '.' => {
								gone_decimal = true;
								buf.push_char(ch);
							},
							ch => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(TNumber(if gone_decimal {
						from_str(buf.as_slice())
					} else {
						from_str_radix(buf.as_slice(), 8)
					}.unwrap()));
				},
				_ if ch.is_digit() => {
					let mut buf = StrBuf::new();
					buf.push_char(ch);
					loop {
						let ch = try!(self.next());
						match ch {
							'.' => buf.push_char(ch),
							_ if ch.is_digit() => buf.push_char(ch),
							_ => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(TNumber(from_str(buf.as_slice()).unwrap()));
				},
				_ if ch.is_alphabetic() || ch == '$' || ch == '_' => {
					let mut buf = StrBuf::new();
					buf.push_char(ch);
					loop {
						let ch = try!(self.next());
						match ch {
							_ if ch.is_alphabetic() || ch.is_digit() => buf.push_char(ch),
							_ => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(TIdent(buf));
				},
				';' => {
					self.push_token(TSemicolon);
				},
				':' => {
					self.push_token(TColon);
				},
				'.' => {
					self.push_token(TDot);
				},
				'(' => {
					self.push_token(TOpenParen);
				},
				')' => {
					self.push_token(TCloseParen);
				},
				',' => {
					self.push_token(TComma);
				},
				'{' => {
					self.push_token(TOpenBlock);
				},
				'}' => {
					self.push_token(TCloseBlock);
				},
				'[' => {
					self.push_token(TOpenArray);
				},
				']' => {
					self.push_token(TCloseArray);
				},
				'?' => {
					self.push_token(TQuestion);
				},
				'/' if self.peek_for('/') => {
					let mut buf = StrBuf::new();
					loop {
						match try!(self.next()) {
							'\n' => break,
							ch => buf.push_char(ch)
						}
					}
					self.push_token(TComment(buf));
				},
				'/' if self.peek_for('*') => {
					let mut buf = StrBuf::new();
					loop {
						match try!(self.next()) {
							'\n' => break,
							'*' if self.peek_for('/') => break,
							ch => buf.push_char(ch)
						}
					}
					self.push_token(TComment(buf));
				},
				'/' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinNum(OpDiv)));
				},
				'/' => {
					self.push_token(TBinOp(BinNum(OpDiv)));
				},
				'*' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinNum(OpMul)));
				},
				'*' => {
					self.push_token(TBinOp(BinNum(OpMul)));
				},
				'+' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinNum(OpAdd)));
				},
				'+' => {
					self.push_token(TBinOp(BinNum(OpAdd)));
				},
				'-' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinNum(OpSub)));
				},
				'-' => {
					self.push_token(TBinOp(BinNum(OpSub)));
				},
				'%' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinNum(OpMod)));
				},
				'%' => {
					self.push_token(TBinOp(BinNum(OpMod)));
				},
				'|' if self.peek_for('|') => {
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinLog(LogOr)));
					} else {
						self.push_token(TBinOp(BinLog(LogOr)));
					}
				},
				'|' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinBit(BitOr)));
				},
				'|' => {
					self.push_token(TBinOp(BinBit(BitOr)));
				},
				'&' if self.peek_for('&') => {
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinLog(LogAnd)));
					} else {
						self.push_token(TBinOp(BinLog(LogAnd)));
					}
				},
				'&' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinBit(BitAnd)));
				},
				'&' => {
					self.push_token(TBinOp(BinBit(BitAnd)));
				},
				'^' if self.peek_for('=') => {
					self.push_token(TAssignOp(BinBit(BitXor)));
				},
				'^' => {
					self.push_token(TBinOp(BinBit(BitXor)));
				},
				'=' if self.peek_for('>') => {
					self.push_token(TArrow);
				},
				'=' if self.peek_for('=') => {
					if self.peek_for('=') {
						self.push_token(TBinOp(BinComp(CompStrictEqual)));
					} else {
						self.push_token(TBinOp(BinComp(CompEqual)));
					}
				},
				'=' => {
					self.push_token(TEqual);
				},
				'<' if self.peek_for('=') => {
					self.push_token(TBinOp(BinComp(CompLessThanOrEqual)));
				},
				'<' if self.peek_for('<') => {
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinBit(BitShl)));
					} else {
						self.push_token(TBinOp(BinBit(BitShl)));
					}
				},
				'<' => {
					self.push_token(TBinOp(BinComp(CompLessThan)));
				},
				'>' if self.peek_for('=') => {
					self.push_token(TBinOp(BinComp(CompGreaterThanOrEqual)));
				},
				'>' if self.peek_for('>') => {
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinBit(BitShr)));
					} else {
						self.push_token(TBinOp(BinBit(BitShr)));
					}
				},
				'>' => {
					self.push_token(TBinOp(BinComp(CompGreaterThan)));
				},
				'!' if self.peek_for('=') => {
					if self.peek_for('=') {
						self.push_token(TBinOp(BinComp(CompStrictNotEqual)));
					} else {
						self.push_token(TBinOp(BinComp(CompNotEqual)));
					}
				},
				'!' => {
					self.push_token(TUnaryOp(UnaryNot));
				},
				'\n' => {
					self.line_number += 1;
					self.column_number = 0;
				},
				'\r' => {
					self.column_number = 0;
				},
				' ' => (),
				ch => fail!("{}:{}: Unexpected '{}'", self.line_number, self.column_number, ch)
			};
		}
		Ok(())
	}
}