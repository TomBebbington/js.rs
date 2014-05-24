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
use std::char::{from_u32, is_whitespace};
use std::num::from_str_radix;
#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Show)]
/// The type of string used
pub enum StringType {
	/// `"` - Double-quoted
	DoubleQuote,
	/// `'` - Single-quoted
	SingleQuote
}
#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Show)]
/// The type of comment used
pub enum CommentType {
	/// `/*...*/` - A multi-line comment
	MultiLineComment,
	/// `//...` - A single-line comment
	SingleLineComment
}
#[deriving(Clone, Eq, Show)]
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
	/// The string buffer for identities
	ident_buffer : StrBuf,
	/// The string buffer for strings
	string_buffer : StrBuf,
	/// The string buffer for comments
	comment_buffer : StrBuf,
	/// The string buffer for numbers
	num_buffer : StrBuf,
	/// The kind of string or `None` if it isn't in a string
	string_start : Option<StringType>,
	/// The kind of comment or `None` if it isn't in a comment
	current_comment : Option<CommentType>,
	/// The kind of number or `None` if it isn't in a number
	current_number : Option<NumberType>,
	/// True if a backwards slash has just been read
	escaped : bool,
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
			ident_buffer: StrBuf::with_capacity(32),
			string_buffer: StrBuf::with_capacity(256),
			num_buffer: StrBuf::with_capacity(16),
			comment_buffer: StrBuf::with_capacity(32),
			string_start: None,
			current_comment: None,
			current_number: None,
			escaped: false,
			line_number: 1,
			column_number: 0,
			buffer: buffer,
			peek_buffer: StrBuf::new()
		};
	}
	fn clear_buffer(&mut self) {
		if self.ident_buffer.len() > 0 {
			let ident = self.ident_buffer.clone();
			self.push_token(TIdent(ident));
			self.ident_buffer.clear();
		}
		if self.current_number.is_some() {
			let radix = match self.current_number.unwrap() {
				HexadecimalNumber => 16,
				OctalNumber => 8,
				DecimalNumber => 10
			};
			let num = match from_str_radix(self.num_buffer.as_slice(), radix) {
				Some(v) => v,
				None => fail!("{}:{}: Could not parse '{}' as a base {} number", self.line_number, self.column_number, self.num_buffer, radix)
			};
			self.push_token(TNumber(num));
			self.num_buffer.clear();
			self.current_number = None;
		}
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
			Ok(self.peek_buffer.pop_char().unwrap())
		}
	}
	fn peek_for(&mut self, peek:char) -> bool {
		if self.peek_buffer.len() > 0 {
			self.peek_buffer.pop_char() == Some(peek)
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
				_ if self.escaped => {
					self.escaped = false;
					if ch != '\n' {
						let escaped_ch = match ch {
							'n' => '\n',
							'r' => '\r',
							't' => '\t',
							'b' => '\x08',
							'f' => '\x0c',
							'0' => '\0',
							'x' => {
								let mut nums = StrBuf::with_capacity(2);
								for _ in range(0, 2) {
									nums.push_char(try!(self.next()).clone());
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
							'\'' if self.string_start == Some(SingleQuote) => '\'',
							'"' if self.string_start == Some(DoubleQuote) => '"',
							_ => fail!("{}:{}: Invalid escape `{}`", self.line_number, self.column_number, ch)
						};
						self.string_buffer.push_char(escaped_ch);
					}
				},
				'\n' if self.current_comment == Some(SingleLineComment) => {
					let comment = self.comment_buffer.clone();
					self.push_token(TComment(comment));
					self.comment_buffer.clear();
					self.current_comment = None;
				},
				'*' if self.current_comment == Some(MultiLineComment) && self.peek_for('/') => {
					let comment = self.comment_buffer.clone();
					self.push_token(TComment(comment));
					self.comment_buffer.clear();
					self.current_comment = None;
				},
				_ if self.current_comment.is_some() => {
					self.comment_buffer.push_char(ch);
				},
				'"' if self.string_start == Some(DoubleQuote) => {
					self.string_start = None;
					let string = self.string_buffer.clone();
					self.push_token(TString(string));
					self.string_buffer.clear();
				},
				'\'' if self.string_start == Some(SingleQuote) => {
					self.string_start = None;
					let string = self.string_buffer.clone();
					self.push_token(TString(string));
					self.string_buffer.clear();
				},
				'\\' if self.string_start.is_some() => self.escaped = true,
				_ if self.string_start.is_some() => self.string_buffer.push_char(ch),
				'"' if self.string_start.is_none() => self.string_start = Some(DoubleQuote),
				'0' if self.peek_for('x') => {
					self.current_number = Some(HexadecimalNumber);
				},
				'0' if self.ident_buffer.len() == 0 && self.current_number.is_none() => {
					self.num_buffer.push_char(ch);
					self.current_number = Some(OctalNumber);
				},
				'0'..'7' if self.current_number == Some(OctalNumber) => {
					self.num_buffer.push_char(ch);
				},
				'8' | '9' if self.current_number == Some(OctalNumber) => {
					self.num_buffer.push_char(ch);
					self.current_number = Some(DecimalNumber);
				},
				'0'.. '9' | 'A'.. 'F' | 'a' .. 'f' if self.current_number == Some(HexadecimalNumber) => {
					self.num_buffer.push_char(ch)
				},
				'0'.. '9' if self.ident_buffer.len() == 0 => {
					self.num_buffer.push_char(ch);
					self.current_number = Some(DecimalNumber);
				},
				'\'' if self.string_start.is_none() => self.string_start = Some(SingleQuote),
				'.' if self.current_number.is_some() && !self.num_buffer.as_slice().contains(".") => {
					self.num_buffer.push_char(ch);
					self.current_number = Some(DecimalNumber);
				},
				';' => {
					self.clear_buffer();
					self.push_token(TSemicolon);
				},
				':' => {
					self.clear_buffer();
					self.push_token(TColon);
				},
				'.' => {
					self.clear_buffer();
					self.push_token(TDot);
				},
				'(' => {
					self.clear_buffer();
					self.push_token(TOpenParen);
				},
				')' => {
					self.clear_buffer();
					self.push_token(TCloseParen);
				},
				',' => {
					self.clear_buffer();
					self.push_token(TComma);
				},
				'{' => {
					self.clear_buffer();
					self.push_token(TOpenBlock);
				},
				'}' => {
					self.clear_buffer();
					self.push_token(TCloseBlock);
				},
				'[' => {
					self.clear_buffer();
					self.push_token(TOpenArray);
				},
				']' => {
					self.clear_buffer();
					self.push_token(TCloseArray);
				},
				'?' => {
					self.clear_buffer();
					self.push_token(TQuestion);
				},
				'/' if self.peek_for('/') => {
					self.current_comment = Some(SingleLineComment);
				},
				'/' if self.peek_for('*') => {
					self.current_comment = Some(MultiLineComment);
				},
				'/' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinNum(OpDiv)));
				},
				'/' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinNum(OpDiv)));
				},
				'*' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinNum(OpMul)));
				},
				'*' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinNum(OpMul)));
				},
				'+' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinNum(OpAdd)));
				},
				'+' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinNum(OpAdd)));
				},
				'-' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinNum(OpSub)));
				},
				'-' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinNum(OpSub)));
				},
				'%' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinNum(OpMod)));
				},
				'%' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinNum(OpMod)));
				},
				'|' if self.peek_for('|') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinLog(LogOr)));
					} else {
						self.push_token(TBinOp(BinLog(LogOr)));
					}
				},
				'|' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinBit(BitOr)));
				},
				'|' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinBit(BitOr)));
				},
				'&' if self.peek_for('&') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinLog(LogAnd)));
					} else {
						self.push_token(TBinOp(BinLog(LogAnd)));
					}
				},
				'&' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinBit(BitAnd)));
				},
				'&' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinBit(BitAnd)));
				},
				'^' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TAssignOp(BinBit(BitXor)));
				},
				'^' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinBit(BitXor)));
				},
				'=' if self.peek_for('>') => {
					self.clear_buffer();
					self.push_token(TArrow);
				},
				'=' if self.peek_for('=') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TBinOp(BinComp(CompStrictEqual)));
					} else {
						self.push_token(TBinOp(BinComp(CompEqual)));
					}
				},
				'=' => {
					self.clear_buffer();
					self.push_token(TEqual);
				},
				'<' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TBinOp(BinComp(CompLessThanOrEqual)));
				},
				'<' if self.peek_for('<') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinBit(BitShl)));
					} else {
						self.push_token(TBinOp(BinBit(BitShl)));
					}
				},
				'<' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinComp(CompLessThan)));
				},
				'>' if self.peek_for('=') => {
					self.clear_buffer();
					self.push_token(TBinOp(BinComp(CompGreaterThanOrEqual)));
				},
				'>' if self.peek_for('>') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TAssignOp(BinBit(BitShr)));
					} else {
						self.push_token(TBinOp(BinBit(BitShr)));
					}
				},
				'>' => {
					self.clear_buffer();
					self.push_token(TBinOp(BinComp(CompGreaterThan)));
				},
				'!' if self.peek_for('=') => {
					self.clear_buffer();
					if self.peek_for('=') {
						self.push_token(TBinOp(BinComp(CompStrictNotEqual)));
					} else {
						self.push_token(TBinOp(BinComp(CompNotEqual)));
					}
				},
				'!' => {
					self.clear_buffer();
					self.push_token(TUnaryOp(UnaryNot));
				},
				'\n' => {
					self.clear_buffer();
					self.line_number += 1;
					self.column_number = 0;
				},
				'\r' => {
					self.clear_buffer();
					self.column_number = 0;
				},
				_ if is_whitespace(ch) => self.clear_buffer(),
				_ => self.ident_buffer.push_char(ch)
			};
		}
		self.clear_buffer();
		Ok(())
	}
}