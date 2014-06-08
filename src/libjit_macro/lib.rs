#![crate_id = "jit_macro"]
#![comment = "LibJIT Macro"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![feature(quote, globs, macro_registrar, managed_boxes)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]
//! This crate provides a macro `jit_type` which can compile a Rust type
//! into its LibJIT counterpart. It even supports functions *and* structs!
//! 
//! For example:
//! 
//! ```rust
//! #![feature(phase)]
//! extern crate jit;
//! #[phase(syntax)]
//! extern crate jit_macro;
//! fn main() {
//! 	let ty = jit_type!(i64);
//! 	assert_eq(ty.get_size(), 8);
//! 	let floor_sig = jit_type!((f64) -> i32);
//! 	let double_array_ty = jit_type!({
//! 		len: int,
//! 		ptr: **f64
//! 	});
//! }
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
	let expander = box BasicMacroExpander { expander: jit_type, span: None };
	register(token::intern("jit_type"), NormalTT(expander, None))
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
				TTTok(_, ref tok) =>
					Err(format!("Bad result {}", tok)),
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
								let mut arg_types : Vec<P<Expr>> = Vec::new();
								{
									let mut tts = toks.iter().peekable();
									loop {
										let parsed = try!(jit_parse_type(cx, &mut tts));
										arg_types.push(cx.expr_addr_of(parsed.span, parsed));
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
								let parsed_ret = try!(jit_parse_type(cx, tts));
								let ret = cx.expr_addr_of(parsed_ret.span, parsed_ret);
								let arg_types_ex = cx.expr(span, ExprVec(arg_types));
								let func = quote_expr!(&mut*cx, ::jit::Type::create_signature);
								let cdecl = quote_expr!(&mut*cx, ::jit::CDECL);
								let arg_types_ex = cx.expr_mut_addr_of(arg_types_ex.span, arg_types_ex);
								let arg_types_ex = cx.expr_method_call(span, arg_types_ex, cx.ident_of("as_mut_slice"), vec!());
								let call = cx.expr_call(span, func, vec!(cdecl, ret, arg_types_ex));
								Ok(call)

							}
						},
						TTTok(span, LBRACE) => {
							let toks = toks.slice(1, toks.len() - 1);
							let mut tts = toks.iter().peekable();
							let mut names:Vec<P<Expr>> = Vec::new();
							let mut types:Vec<P<Expr>> = Vec::new();
							loop {
								match tts.next() {
									Some(ref v) => {
										match **v {
											TTTok(_, COMMA) =>
												continue,
											TTTok(span, IDENT(field_name, _)) => {
												try!(expect(cx, &mut tts, COLON));
												let field_type = try!(jit_parse_type(cx, &mut tts));
												names.push(cx.expr_method_call(span, cx.expr_str(span, get_ident(field_name)), cx.ident_of("into_string"), vec!()));
												types.push(cx.expr_addr_of(field_type.span, field_type));
											},
											_ => return Err("Expected ident".into_string())
										}
									},
									None => break
								}
							}
							let func = quote_expr!(&mut*cx, ::jit::Type::create_struct);
							let types_vec = cx.expr(span, ExprVec(types));
							let types_vec_slices = cx.expr_method_call(span, types_vec, cx.ident_of("as_mut_slice"), vec!());
							let names_vec = cx.expr(span, ExprVec(names));
							let ident_type = cx.ident_of("ty");
							let final = cx.expr_block(cx.block(span, vec!(
								cx.stmt_let(span, false, ident_type, cx.expr_call(span, func, vec!(types_vec_slices))),
								cx.stmt_expr(cx.expr_method_call(span, cx.expr_ident(span, ident_type), cx.ident_of("set_names"), vec!(names_vec)))
							), Some(cx.expr_ident(span, ident_type))));
							println!("{}", final.to_source());
							Ok(final)
						},
						_ => Err("Expected bracket".into_string())
					}
				},
				TTTok(_, IDENT(ident, _)) => {
					let ident = ident.to_source();
					let type_id = match ident.as_slice() {
						"void" => "void",
						"int" | "i32" => "int",
						"uint" | "u32" => "uint",
						"i64" => "long",
						"u64" => "ulong",
						"char" => "char",
						"bool" => "bool",
						"f64" => "float64",
						"f32" => "float32",
						"String" => "cstring",
						"Vec" => "vec",
						id => return Err(format!("Unexpected identifier {}", id))
					};
					let full_type_id = cx.ident_of("get_".into_string().append(type_id).as_slice());
					tts.next();
					Ok(quote_expr!(cx, ::jit::Types::$full_type_id()))
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
fn jit_type(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult> {
	let mut iter = tts.iter().peekable();
	MacExpr::new(match jit_parse_type(cx, &mut iter) {
		Ok(v) => v,
		Err(e) => {
			cx.span_err(sp, e.as_slice());
			quote_expr!(cx, fail!("Invalid"))
		}
	})
}