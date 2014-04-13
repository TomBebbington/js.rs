use std::io::{BufReader, BufferedReader, Reader};
use std::strbuf::StrBuf;
use std::char::from_u32;
use std::num::FromStrRadix;
use ast::{TIdent, TNumber, TString, TSemicolon, TColon, TDot, TEqual, TOpenParen, TCloseParen, TComma, TOpenBlock, TCloseBlock, TOpenArray, TCloseArray, TQuestion, TNumOp, TBitOp};
use ast::{OpAdd, OpSub, OpMul, OpDiv, OpMod};
use ast::{BitAnd, BitOr, BitXor};
use ast::Token;
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
	escaped : bool
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
			escaped: false
		};
	}
	fn clear_buffer(&mut self) {
		if self.ident_buffer.len() > 0 {
			self.tokens.push(TIdent(self.ident_buffer.clone().into_owned()));
			self.ident_buffer.truncate(0);
		}
		if self.in_num {
			self.tokens.push(TNumber(from_str(self.num_buffer.as_slice()).unwrap()));
			self.num_buffer.truncate(0);
			self.in_num = false;
		}
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
								let as_num = match FromStrRadix::from_str_radix(nums, 16) {
									Some(v) => v,
									None => 0
								};
								match from_u32(as_num) {
									Some(v) => v,
									None => fail!("{} is not a valid unicode scalar value", as_num)
								}
							},
							'\'' if self.string_start == Some(SingleQuote) => '\'',
							'"' if self.string_start == Some(DoubleQuote) => '"',
							_ => fail!("Invalid escape `{}` after {}", ch, self.string_start)
						});
					}
				},
				'"' if self.string_start == Some(DoubleQuote) => {
					self.string_start = None;
					self.tokens.push(TString(self.string_buffer.clone().into_owned()));
					self.string_buffer.truncate(0);
				},
				'\'' if self.string_start == Some(SingleQuote) => {
					self.string_start = None;
					self.tokens.push(TString(self.string_buffer.clone().into_owned()));
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
					self.tokens.push(TSemicolon)
				},
				':' => {
					self.clear_buffer();
					self.tokens.push(TColon)
				},
				'.' => {
					self.clear_buffer();
					self.tokens.push(TDot)
				},
				'(' => {
					self.clear_buffer();
					self.tokens.push(TOpenParen)
				},
				')' => {
					self.clear_buffer();
					self.tokens.push(TCloseParen)
				},
				',' => {
					self.clear_buffer();
					self.tokens.push(TComma)
				},
				'{' => {
					self.clear_buffer();
					self.tokens.push(TOpenBlock)
				},
				'}' => {
					self.clear_buffer();
					self.tokens.push(TCloseBlock)
				},
				'[' => {
					self.clear_buffer();
					self.tokens.push(TOpenArray)
				},
				']' => {
					self.clear_buffer();
					self.tokens.push(TCloseArray)
				},
				'?' => {
					self.clear_buffer();
					self.tokens.push(TQuestion)
				},
				'/' => {
					self.clear_buffer();
					self.tokens.push(TNumOp(OpDiv))
				},
				'*' => {
					self.clear_buffer();
					self.tokens.push(TNumOp(OpMul))
				},
				'+' => {
					self.clear_buffer();
					self.tokens.push(TNumOp(OpAdd))
				},
				'-' => {
					self.clear_buffer();
					self.tokens.push(TNumOp(OpSub))
				},
				'%' => {
					self.clear_buffer();
					self.tokens.push(TNumOp(OpMod))
				},
				'|' => {
					self.clear_buffer();
					self.tokens.push(TBitOp(BitOr))
				},
				'&' => {
					self.clear_buffer();
					self.tokens.push(TBitOp(BitAnd))
				},
				'^' => {
					self.clear_buffer();
					self.tokens.push(TBitOp(BitXor))
				},
				'=' => {
					self.clear_buffer();
					self.tokens.push(TEqual)
				},
				_ if is_whitespace(ch) => self.clear_buffer(),
				_ => self.ident_buffer.push_char(ch)
			};
		}
	}
}