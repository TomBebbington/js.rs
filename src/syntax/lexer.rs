use ast::punc::*;
use ast::token::*;
use std::io::{BufReader, BufferedReader, Buffer, IoError, IoResult, EndOfFile};
use std::char::from_u32;
use std::num::from_str_radix;
use std::from_str::FromStr;
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
	peek_buffer: String
}
impl<B:Buffer> Lexer<B> {
	/// Creates a new lexer with empty buffers
	pub fn new(buffer: B) -> Lexer<B> {
		Lexer {
			tokens: Vec::new(),
			line_number: 1,
			column_number: 0,
			buffer: buffer,
			peek_buffer: String::new()
		}
	}
	#[inline(always)]
	fn push_token(&mut self, tk:TokenData) {
		self.tokens.push(Token::new(tk, self.line_number, self.column_number))
	}
	#[inline(always)]
	fn push_punc(&mut self, punc:Punctuator) {
		self.push_token(TPunctuator(punc));
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
					let mut buf = String::new();
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
											let mut nums = String::with_capacity(2);
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
											let mut nums = String::new();
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
					self.push_token(TStringLiteral(buf))
				},
				'0' if self.peek_for('x') => {
					let mut buf = String::new();
					loop {
						match try!(self.next()) {
							ch if ch.is_digit_radix(16) => buf.push_char(ch),
							ch => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(TNumericLiteral(from_str_radix(buf.as_slice(), 16).unwrap()));
				},
				'0' => {
					let mut buf = "0".into_string();
					let mut gone_decimal = false;
					loop {
						let ch = self.next();
						match ch {
							Ok(ch) if ch.is_digit_radix(8) =>
								buf.push_char(ch),
							Ok('8') | Ok('9') | Ok('.') => {
								gone_decimal = true;
								buf.push_char(ch.unwrap());
							},
							Ok(ch) => {
								self.peek_buffer.push_char(ch);
								break;
							},
							Err(IoError {kind: EndOfFile, ..}) =>
								break,
							Err(err) =>
								return Err(err)
						}
					}
					self.push_token(TNumericLiteral(if gone_decimal {
						from_str(buf.as_slice())
					} else {
						from_str_radix(buf.as_slice(), 8)
					}.unwrap()));
				},
				_ if ch.is_digit() => {
					let mut buf = String::new();
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
					self.push_token(TNumericLiteral(from_str(buf.as_slice()).unwrap()));
				},
				_ if ch.is_alphabetic() || ch == '$' || ch == '_' => {
					let mut buf = String::new();
					buf.push_char(ch);
					loop {
						let ch = try!(self.next());
						match ch {
							_ if ch.is_alphabetic() || ch.is_digit() || ch == '_' => buf.push_char(ch),
							_ => {
								self.peek_buffer.push_char(ch);
								break;
							}
						}
					}
					self.push_token(match buf.as_slice() {
						"true" => TBooleanLiteral(true),
						"false" => TBooleanLiteral(false),
						"null" => TNullLiteral,
						slice => match FromStr::from_str(slice) {
							Some(keyword) => TKeyword(keyword),
							None => TIdentifier(buf.clone())
						}
					});
				},
				';' => {
					self.push_punc(PSemicolon);
				},
				':' => {
					self.push_punc(PColon);
				},
				'.' => {
					self.push_punc(PDot);
				},
				'(' => {
					self.push_punc(POpenParen);
				},
				')' => {
					self.push_punc(PCloseParen);
				},
				',' => {
					self.push_punc(PComma);
				},
				'{' => {
					self.push_punc(POpenBlock);
				},
				'}' => {
					self.push_punc(PCloseBlock);
				},
				'[' => {
					self.push_punc(POpenBracket);
				},
				']' => {
					self.push_punc(PCloseBracket);
				},
				'?' => {
					self.push_punc(PQuestion);
				},
				'/' if self.peek_for('/') => {
					let mut buf = String::new();
					loop {
						match try!(self.next()) {
							'\n' => break,
							ch => buf.push_char(ch)
						}
					}
					self.push_token(TComment(buf));
				},
				'/' if self.peek_for('*') => {
					let mut buf = String::new();
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
					self.push_punc(PAssignDiv);
				},
				'/' => {
					self.push_punc(PDiv);
				},
				'*' if self.peek_for('=') => {
					self.push_punc(PAssignMul);
				},
				'*' => {
					self.push_punc(PMul);
				},
				'+' if self.peek_for('=') => {
					self.push_punc(PAssignAdd);
				},
				'+' if self.peek_for('+') => {
					self.push_punc(PInc);
				},
				'+' => {
					self.push_punc(PAdd);
				},
				'-' if self.peek_for('=') => {
					self.push_punc(PAssignSub);
				},
				'-' if self.peek_for('-') => {
					self.push_punc(PDec);
				},
				'-' => {
					self.push_punc(PSub);
				},
				'%' if self.peek_for('=') => {
					self.push_punc(PAssignMod);
				},
				'%' => {
					self.push_punc(PMod);
				},
				'|' if self.peek_for('|') => {
					self.push_punc(PBoolOr);
				},
				'|' if self.peek_for('=') => {
					self.push_punc(PAssignOr);
				},
				'|' => {
					self.push_punc(POr);
				},
				'&' if self.peek_for('&') => {
					self.push_punc(PBoolAnd);
				},
				'&' if self.peek_for('=') => {
					self.push_punc(PAssignAnd);
				},
				'&' => {
					self.push_punc(PAnd);
				},
				'^' if self.peek_for('=') => {
					self.push_punc(PAssignXor);
				},
				'^' => {
					self.push_punc(PXor);
				},
				'=' if self.peek_for('>') => {
					self.push_punc(PArrow);
				},
				'=' if self.peek_for('=') => {
					if self.peek_for('=') {
						self.push_punc(PStrictEq);
					} else {
						self.push_punc(PEq);
					}
				},
				'=' => {
					self.push_punc(PAssign);
				},
				'<' if self.peek_for('=') => {
					self.push_punc(PLessThanOrEq);
				},
				'<' if self.peek_for('<') => {
					if self.peek_for('=') {
						self.push_punc(PAssignLeftSh);
					} else {
						self.push_punc(PLeftSh);
					}
				},
				'<' => {
					self.push_punc(PLessThan);
				},
				'>' if self.peek_for('=') => {
					self.push_punc(PGreaterThanOrEq);
				},
				'>' if self.peek_for('>') => {
					if self.peek_for('=') {
						self.push_punc(PAssignRightSh);
					} else if self.peek_for('>') {
						if self.peek_for('=') {
							self.push_punc(PAssignURightSh);
						} else {
							self.push_punc(PURightSh);
						}
					} else {
						self.push_punc(PRightSh);
					}
				},
				'>' => {
						self.push_punc(PGreaterThan);
				},
				'!' if self.peek_for('=') => {
					if self.peek_for('=') {
						self.push_punc(PStrictNotEq);
					} else {
						self.push_punc(PNotEq);
					}
				},
				'!' => {
					self.push_punc(PNot);
				},
				'~' => {
					self.push_punc(PNeg);
				},
				'\n' | '\u2028'|'\u2029' => {
					self.line_number += 1;
					self.column_number = 0;
				},
				'\r' => {
					self.column_number = 0;
				},
				' ' => (),
				ch => fail!("{}:{}: Unexpected '{}'", self.line_number, self.column_number, ch)
			};
		};
		Ok(())
	}
}