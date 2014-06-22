use stdlib::value::{Value, ResultValue, to_value, from_value};
use stdlib::function::Function;
use std::iter::FromIterator;
use std::io::stdio::stderr;
use time::{now, strftime};
/// Print a javascript value to the standard output stream
pub fn log(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let args : Vec<String> = FromIterator::from_iter(args.iter().map(|x|from_value::<String>(*x).unwrap()));
    println!("{}: {}", strftime("%X", &now()), args.connect(" "));
    Ok(Value::undefined())
}
/// Print a javascript value to the standard error stream
pub fn error(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let args : Vec<String> = FromIterator::from_iter(args.iter().map(|x|from_value::<String>(*x).unwrap()));
    match writeln!(&mut stderr().unwrap(), "{}: {}", strftime("%X", &now()), args.connect(" ")) {
        Ok(_) => Ok(Value::undefined()),
        Err(io_error) => Err(to_value(io_error.to_str()))
    }
}
/// Create a new `console` object
pub fn _create(global : Value) -> Value {
    js!(global, {
        "log": Function::make(log, ["object"]),
        "error": Function::make(error, ["error"]),
        "exception": Function::make(error, ["error"])
    })
}
/// Initialise the global object with the `console` object
pub fn init(global:Value) {
    js_extend!(global, {
        "console": _create(global)
    });
}