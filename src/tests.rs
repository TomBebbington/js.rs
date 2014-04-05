#![feature(globs)]
#![crate_type = "bin"]
extern crate rust_js;
extern crate test;
extern crate collections;
use rust_js::lexer::Lexer;
use rust_js::parser::{Parser, VerboseResult};
use rust_js::exec::{Executor, Interpreter};
use rust_js::js::value::*;
use collections::TreeMap;
use std::fmt::Show;

fn run(script:&str) -> Value {
	let mut lexer = Lexer::new();
	lexer.lex_str(script.to_owned()).v_unwrap();
	let mut parser = Parser::new(lexer.tokens);
	let result = parser.parse_all().v_unwrap();
	let mut engine : ~Interpreter = Executor::new();
	let result = engine.run(result);
	println!("Result: {}", result);
	return result.unwrap();
}
fn assert_eq<T:Show + Eq>(a:T, b:T) -> () {
	if a != b {
		fail!("Expected {}, got {}", a, b);
	}
}
#[test]
fn test_array_comma() {
	let mut map = ~TreeMap::new();
	map.insert(~"0", VNull);
	map.insert(~"1", VString(~"home"));
	map.insert(~"2", VNull);
	map.insert(~"3", VString(~"school"));
	assert_eq(VObject(map), run("[ , 'home', , 'school']"));
}
#[test]
fn test_escape() {
	assert_eq(VString(~"Newline:'"), run("'Newline:\\\''"));
}
#[test]
fn test_to_string() {
	assert_eq(VString(~"{a: 3}"), run("({a : 3})+''"));
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
	assert_eq(VString(~"Hello"), run("function Text(phrase) { this.phrase = phrase; } new Text('Hello').phrase"));
}