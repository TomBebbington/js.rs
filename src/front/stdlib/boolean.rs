use stdlib::value::{Value, VUndefined, ResultValue, to_value};
use stdlib::function::Function;
use std::gc::GC;

/// Create a new boolean
pub fn make_boolean(_:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    Ok(Value::new(VUndefined))
}
/// Create a new `Boolean` object
pub fn _create(global: Value) -> Value {
    let boolean = Function::make(make_boolean, []);
    boolean
}
/// Initialise the global object with the `Boolean` object
pub fn init(global:Value) {
	js_extend!(global, {
		"Boolean": _create(global)
	});
}