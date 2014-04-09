#![feature(globs)]
#![crate_type = "bin"]
extern crate rust_js;
extern crate test;
extern crate collections;
use rust_js::lexer::Lexer;
use rust_js::parser::{Parser, VerboseResult};
use rust_js::exec::{Executor, Interpreter};
use rust_js::js::value::{Value, ValueData, VNull, VNumber, VString};

fn run(script:&str) -> Value {
	let mut lexer = Lexer::new();
	lexer.lex_str(script.to_owned()).v_unwrap();
	let mut parser = Parser::new(lexer.tokens);
	let result = parser.parse_all().v_unwrap();
	let mut engine : ~Interpreter = Executor::new();
	let result : Result<Value, Value> = engine.run(result);
	return result.unwrap();
}
fn assert_eq(a:ValueData, b:Value) -> () {
	if a != *b.borrow() {
		fail!("Expected {}, got {}", a, b.borrow());
	}
}
#[test]
fn test_array_comma() {
	assert_eq(VNull, run("[ , 'home', , 'school'][0]"));
}
#[test]
fn test_escape() {
	assert_eq(VString(~"Newline:'"), run("'Newline:\\\''"));
}
#[test]
fn test_to_string() {
	assert_eq(VString(~"117"), run("117+''"));
	assert_eq(VString(~"0.12"), run("0.12+''"));
	assert_eq(VString(~"null"), run("null+''"));
	assert_eq(VString(~"undefined"), run("unexisty+''"));
}
#[test]
fn test_num_op() {
	assert_eq(VNumber(10.0), run("((4 + 2) / 3) * 5"));
}
#[test]
fn test_hello_world() {
	assert_eq(VString(~"Hello, world!"), run("'Hello, world!'"));
}
#[test]
fn test_function() {
	assert_eq(VString(~"Function!"), run("(function(a){return a + '!'})('Function')"))
}
#[test]
fn test_constructor() {
	assert_eq(VString(~"Hello"), run("function Text(phrase) { this.phrase = phrase; }; text = new Text('Hello'); text.phrase"));
}
#[test]
fn test_json() {
	assert_eq(VNumber(~"42"), run("JSON.parse(JSON.stringify({num: 42})).num"))
}