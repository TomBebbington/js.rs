#![crate_id = "jit_macro"]
#![comment = "LibJIT Macro"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![feature(quote, globs, macro_registrar, managed_boxes)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]
//! This crate provides a macro `jit_compile` which can compile a Rust type identifier
//! into its LibJIT counterpart.
//! 
//! For example:
//! ```rust
//! # #![feature(phase)]
//! # extern crate regex;
//! # #[phase(syntax)]
//! # extern crate regex_macros;
//! # fn main() {
//! let ty = jit_compile!(i64);
//! assert_eq(ty.get_size(), 8);
//! #
//! ```
extern crate syntax;
use syntax::ext::quote::rt::ToSource;
use syntax::ast::*;
use syntax::codemap::Span;
use syntax::ext::base::*;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::parse::token::*;
use std::iter::Peekable;
use std::slice::Items;
#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |Name, SyntaxExtension|) {
	let expander = box BasicMacroExpander { expander: jit_compile, span: None };
	register(token::intern("jit_compile"), NormalTT(expander, None))
}
fn error(e:Option<TokenTree>, tok:Token) -> Result<(), String> {
	match e {
		Some(TTTok(_, ref got)) => Err(format!("Bad syntax - got {} but expected {}", got, tok)),
		Some(TTNonterminal(_, ref got)) => Err(format!("Bad syntax - got {} but expected {}", got, tok)),
		Some(TTDelim(ref trees)) => error(Some(trees.deref().get(0).clone()), tok),
		Some(_) => Err("Bad syntax".into_string()),
		None => Err("Abrupt end".into_string())
	}
}
fn expect<'a>(_: &mut ExtCtxt, tts:&mut Peekable<&'a TokenTree, Items<'a, TokenTree>>, tok:Token) -> Result<(), String> {
	match tts.next().map(|v| v.clone()) {
		Some(TTTok(_, ref got)) if *got == tok => {
			Ok(())
		},
		Some(TTDelim(ref trees)) => {
			let curr = trees.deref().get(0).clone();
			match curr {
				TTTok(_, ref got) if *got == tok =>
					Ok(()),
				_ =>
					Err("Bad result".into_string())
			}
		},
		v => error(v, tok)
	}
}
fn jit_parse_type<'a>(cx: &mut ExtCtxt, tts:&mut Peekable<&'a TokenTree, Items<'a, TokenTree>>) -> Result<P<Expr>, String> {
	let val:Option<&TokenTree> = tts.peek().map(|v|*v);
	match val {
		Some(ref val) => {
			match **val {
				TTDelim(ref toks) => {
					match *toks.get(0) {
						TTTok(span, LPAREN) => {
							let toks = toks.slice(1, toks.len() - 1);
							if toks.len() == 0 {
								Ok(quote_expr!(cx, ::jit::Types::get_void()))
							} else {
								let mut arg_types : Vec<@Expr> = Vec::new();
								{
									let mut tts = toks.iter().peekable();
									loop {
										let parsed = try!(jit_parse_type(cx, &mut tts));
										// &*parsed
										let derefed = cx.expr_deref(span, parsed);
										arg_types.push((&mut *cx).expr_addr_of(span, derefed));
										match tts.next() {
											Some(v) =>
												match *v {
													TTTok(_, COMMA) => {
														continue;
													},
													_ => return Err("Unexpected token in function decl".into_string())
												},
											None => break
										}
									}
								}
								tts.next();
								try!(expect(cx, tts, RARROW));
								let ret = try!(jit_parse_type(cx, tts));
								let arg_types_ex = cx.expr(ret.span, ExprVec(arg_types));
								let func = quote_expr!(&mut*cx, ::jit::Type::create_signature);
								let cdecl = quote_expr!(&mut*cx, ::jit::CDECL);
								let arg_types_ex = cx.expr_mut_addr_of(span, arg_types_ex);
								let call = cx.expr_call(span, func, vec!(cdecl, ret, arg_types_ex));
								Ok(call)

							}
						},
						_ => Err("Expected bracker".into_string())
					}
				},
				TTTok(_, IDENT(ident, _)) => {
					let ident = ident.to_source();
					let res = match ident.as_slice() {
						"void" => Ok(quote_expr!(cx, ::jit::Types::get_void())),
						"int" | "i32" => Ok(quote_expr!(cx, ::jit::Types::get_int())),
						"uint" | "u32" => Ok(quote_expr!(cx, ::jit::Types::get_uint())),
						"i64" => Ok(quote_expr!(cx, ::jit::Types::get_long())),
						"u64" => Ok(quote_expr!(cx, ::jit::Types::get_ulong())),
						"char" => Ok(quote_expr!(cx, ::jit::Types::get_char())),
						"bool" => Ok(quote_expr!(cx, ::jit::Types::get_bool())),
						"f64" => Ok(quote_expr!(cx, ::jit::Types::get_float64())),
						"f32" => Ok(quote_expr!(cx, ::jit::Types::get_float32())),
						"Vec" => Ok(quote_expr!(cx, ::jit::Types::get_vec())),
						id => Err(format!("Unexpected identifier {}", id))
					};
					if res.is_ok() {
						tts.next();
					}
					res
				},
				TTTok(span, BINOP(STAR)) => {
					tts.next();
					let func = quote_expr!(&mut*cx, ::jit::Type::create_pointer);
					let parsed_type = try!(jit_parse_type(&mut*cx, tts));
					let wv = cx.expr_addr_of(span, parsed_type);
					Ok(cx.expr_call(span, func, vec!(wv)))
				},
				TTTok(_, DOLLAR) => {
					tts.next();
					match tts.next() {
						Some(ref tks) => {
							match **tks {
								TTTok(span, IDENT(ident, _)) => {
									Ok(cx.expr_ident(span, ident))
								}
								_ => Err("Expected next".into_string())
							}
						},
						None => Err("Abrupt end".into_string())
					}
				},
				ref token => {
					try!(error(Some(token.clone()), DOLLAR));
					Err("...".into_string())
				}
			}
		},
		None =>
			Err("Not a type".into_string())
	}
}
fn jit_compile(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
	let mut iter = tts.iter().peekable();
	MacExpr::new(match jit_parse_type(cx, &mut iter) {
		Ok(v) => v,
		Err(e) => {
			cx.span_err(sp, e.as_slice());
			quote_expr!(cx, fail!("Invalid"))
		}
	})
}