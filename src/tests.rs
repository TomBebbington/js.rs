#![feature(globs)]
#![crate_type = "bin"]
extern crate rust_js;
extern crate test;
extern crate collections;
use test::Bencher;
use rust_js::lexer::Lexer;
use rust_js::parser::{Parser, VerboseResult};
use rust_js::exec::{Executor, Interpreter};
use rust_js::js::value::{Value, ValueData, VNull, VNumber, VString, VBoolean, VFunction, VUndefined, from_value, to_value};
use std::gc::Gc;
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
fn test_value_conversion() {
	let val:Vec<i32> = from_value(run("[0, 2, 34, 12]")).unwrap();
	if *val.get(0) != 0 || *val.get(1) != 2 || *val.get(2) != 34 || *val.get(3) != 12 {
		fail!("Bad value {}", val);
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
	assert_eq(VNumber(42.0), run("JSON.parse(JSON.stringify({num: 42})).num"))
}
#[test]
fn test_typeof() {
	assert_eq(VString(~"number"), run("typeof Math.PI"));
	assert_eq(VString(~"boolean"), run("typeof true"));
	assert_eq(VString(~"undefined"), run("typeof undefined"));
	assert_eq(VString(~"string"), run("typeof ''"));
	assert_eq(VString(~"function"), run("typeof Object.prototype.hasOwnProperty"));
}
#[test]
fn test_define_prop() {
	assert_eq(VBoolean(true), run("{
		var obj = {};
		Object.defineProperty(obj, \"some_bool\", {
			value: true
		});
		obj.some_bool
	}"));
}
#[bench]
fn bench_fib(b: &mut Bencher) {
	let code = "function fib(n) {
		if(n == 0) {
			return 0;
		} else if (n == 1) {
			return 1;
		} else {
			return fib(n - 2) + fib(n - 1);
		}
	}";
	let mut lexer = Lexer::new();
	lexer.lex_str(code.to_owned()).v_unwrap();
	let mut parser = Parser::new(lexer.tokens);
	let result = parser.parse_all().v_unwrap();
	let engine : &mut Interpreter = Executor::new();
	let result : Result<Value, Value> = engine.run(result);
	let fib = result.unwrap();
	b.iter(|| {
		match *fib.borrow() {
			VFunction(ref func) => {
				assert_eq!(102334155i32, from_value(func.borrow().call(&mut engine.clone(), ValueData::new_obj(None), Gc::new(VUndefined), vec!(to_value(40i32))).unwrap()).unwrap());
			},
			_ => ()
		}
	});
}