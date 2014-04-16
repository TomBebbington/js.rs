use js::value::{Value, ValueData, ResultValue, VUndefined, to_value, from_value};
use std::gc::Gc;
use std::iter::FromIterator;
use std::io::stdio::stderr;
use std::str::MaybeOwned;
use time::{now, strftime};
/// Print a javascript value to the standard output stream
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<MaybeOwned> = FromIterator::from_iter(args.iter().map(|x|from_value::<MaybeOwned>(*x).unwrap()));
	println!("{}: {}", strftime("%X", &now()), args.connect(" "));
	return Ok(Gc::new(VUndefined));
}
/// Print a javascript value to the standard error stream
pub fn error(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<MaybeOwned> = FromIterator::from_iter(args.iter().map(|x|from_value::<MaybeOwned>(*x).unwrap()));
	match writeln!(&mut stderr().unwrap(), "{}: {}", strftime("%X", &now()), args.connect(" ")) {
		Ok(_) => Ok(Gc::new(VUndefined)),
		Err(io_error) => Err(to_value(io_error.to_str().into_maybe_owned()))
	}
}
/// Create a new `console` object
pub fn _create(global : Value) -> Value {
	let console = ValueData::new_obj(Some(global));
	console.borrow().set_field("log".into_maybe_owned(), to_value(log));
	console.borrow().set_field("error".into_maybe_owned(), to_value(error));
	console.borrow().set_field("exception".into_maybe_owned(), to_value(error));
	console
}
/// Initialise the global object with the `console` object
pub fn init(global:Value) {
	global.borrow().set_field("console".into_maybe_owned(), _create(global));
}