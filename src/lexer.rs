use std::io::{BufReader, BufferedReader, Reader};
use std::strbuf::StrBuf;
use std::char::from_u32;
use std::num::FromStrRadix;
use ast::{TIdent, TNumber, TString, TSemicolon, TColon, TDot, TEqual, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TNumOp, TBitOp};
use ast::{OpAdd, OpSub, OpMul, OpDiv, OpMod};
use ast::{BitAnd, BitOr, BitXor};
use ast::{Token, TokenData};
use std::io::{IoResult, EndOfFile};
use std::char::is_whitespace;
#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Show)]
/// The type of string used
pub enum StringType {
	/// Double-quoted
	DoubleQuote,
	/// Single-quoted
	SingleQuote
}
/// The Javascript Lexer
pub struct Lexer {
	/// The list of tokens generated so far
	pub tokens : Vec<Token>,
	/// The string buffer for identities
	ident_buffer : StrBuf,
	/// The string buffer for strings
	string_buffer : StrBuf,
	/// The string buffer for numbers
	num_buffer : StrBuf,
	/// The kind of string - i.e. double quote or single quote or none if it isn't in a string
	string_start : Option<StringType>,
	/// If the lexer is currently inside a number
	in_num : bool,
	/// If a backwards slash has just been read
	escaped : bool,
	/// The current line number
	line_number : uint,
	/// The current column number
	column_number : uint
}
impl Lexer {
	/// Creates a new lexer with empty buffers
	pub fn new() -> ~Lexer {
		return ~Lexer {
			tokens: Vec::new(),
			ident_buffer: StrBuf::with_capacity(32),
			string_buffer: StrBuf::with_capacity(256),
			num_buffer: StrBuf::with_capacity(16),
			string_start: None,
			in_num: false,
			escaped: false,
			line_number: 1,
			column_number: 0
		};
	}
	fn clear_buffer(&mut self) {
		if self.ident_buffer.len() > 0 {
			let ident = self.ident_buffer.clone().into_owned();
			self.push_token(TIdent(ident));
			self.ident_buffer.truncate(0);
		}
		if self.in_num {
			let num = from_str(self.num_buffer.clone().as_slice());
			self.push_token(TNumber(num.unwrap()));
			self.num_buffer.truncate(0);
			self.in_num = false;
		}
	}
	fn push_token(&mut self, tk:TokenData) {
		self.tokens.push(Token::new(tk, self.line_number, self.column_number))
	}
	/// Processes an input stream from a string into an array of tokens
	pub fn lex_str(&mut self, script:~str) -> IoResult<()> {
		let script_bytes:&[u8] = script.as_bytes();
		let reader = BufReader::new(script_bytes);
		let buf_reader = BufferedReader::new(reader);
		self.lex(buf_reader)
	}
	/// Processes an input stream from a BufferedReader into an array of tokens
	pub fn lex<R : Reader>(&mut self, mut reader : BufferedReader<R>) -> IoResult<()> {
		loop {
			let ch = match reader.read_char() {
				Ok(c) => c,
				Err(ref e) if e.kind == EndOfFile => {
					self.clear_buffer();
					return Ok(())
				},
				Err(ref e) =>
					return Err(e.clone())
			};
			self.column_number += 1;
			match ch {
				_ if self.escaped => {
					self.escaped = false;
					if ch != '\n' {
						self.string_buffer.push_char(match ch {
							'n' => '\n',
							'r' => '\r',
							't' => '\t',
							'b' => '\x08',
							'f' => '\x0c',
							'0' => '\0',
							'u' => {
								let mut nums = ~"";
								for _ in range(0, 4) {
									nums = format!("{}{}", nums, try!(reader.read_char()));
								}
								self.column_number += 4;
								let as_num = match FromStrRadix::from_str_radix(nums, 16) {
									Some(v) => v,
									None => 0
								};
								match from_u32(as_num) {
									Some(v) => v,
									None => fail!("Line {}, Column {}: {} is not a valid unicode scalar value", self.line_number, self.column_number, as_num)
								}
							},
							'\'' if self.string_start == Some(SingleQuote) => '\'',
							'"' if self.string_start == Some(DoubleQuote) => '"',
							_ => fail!("Line {}, Column {}: Invalid escape `{}`", self.line_number, self.column_number, ch)
						});
					}
				},
				'"' if self.string_start == Some(DoubleQuote) => {
					self.string_start = None;
					let string = self.string_buffer.clone().into_owned();
					self.push_token(TString(string));
					self.string_buffer.truncate(0);
				},
				'\'' if self.string_start == Some(SingleQuote) => {
					self.string_start = None;
					let string = self.string_buffer.clone().into_owned();
					self.push_token(TString(string));
					self.string_buffer.truncate(0);
				},
				'\\' if self.string_start.is_some() => self.escaped = true,
				_ if self.string_start.is_some() => self.string_buffer.push_char(ch),
				'"' if self.string_start.is_none() => self.string_start = Some(DoubleQuote),
				'0'.. '9' if self.string_start.is_none() && self.ident_buffer.len() == 0 => {
					self.num_buffer.push_char(ch);
					self.in_num = true;
				},
				'\'' if self.string_start.is_none() => self.string_start = Some(SingleQuote),
				'.' if self.in_num && !self.num_buffer.as_slice().contains(".") => {
					self.num_buffer.push_char(ch);
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
				'/' => {
					self.clear_buffer();
					self.push_token(TNumOp(OpDiv));
				},
				'*' => {
					self.clear_buffer();
					self.push_token(TNumOp(OpMul));
				},
				'+' => {
					self.clear_buffer();
					self.push_token(TNumOp(OpAdd));
				},
				'-' => {
					self.clear_buffer();
					self.push_token(TNumOp(OpSub));
				},
				'%' => {
					self.clear_buffer();
					self.push_token(TNumOp(OpMod));
				},
				'|' => {
					self.clear_buffer();
					self.push_token(TBitOp(BitOr));
				},
				'&' => {
					self.clear_buffer();
					self.push_token(TBitOp(BitAnd));
				},
				'^' => {
					self.clear_buffer();
					self.push_token(TBitOp(BitXor));
				},
				'=' => {
					self.clear_buffer();
					self.push_token(TEqual);
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
	}
}