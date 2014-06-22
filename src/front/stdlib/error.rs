use stdlib::object::PROTOTYPE;
use stdlib::value::{Value, ResultValue, to_value};
use stdlib::function::Function;

/// Create a new error
pub fn make_error(args:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    if args.len() >= 1 {
        this.set_field("message", to_value(args.get(0).to_str().into_string()));
    }
    Ok(Value::undefined())
}
/// Get the string representation of the error
pub fn to_string(_:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    let name = this.get_field("name");
    let message = this.get_field("message");
    Ok(to_value(format!("{}: {}", name, message).into_string()))
}
/// Create a new `Error` object
pub fn _create(global: Value) -> Value {
    let prototype = js!(global, {
        "message": "",
        "name": "Error",
        "toString": Function::make(to_string, [])
    });
    let error = Function::make(make_error, ["message"]);
    error.set_field(PROTOTYPE, prototype);
    error
}
/// Initialise the global object with the `Error` object
pub fn init(global:Value) {
    js_extend!(global, {
        "Error": _create(global)
    });
}