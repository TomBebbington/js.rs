use stdlib::value::{Value, ResultValue, to_value, from_value};
use stdlib::function::Function;
use stdlib::object::{PROTOTYPE, Property};

/// Create new string
pub fn make_string(_:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    this.set_field("length", to_value(0i32));
    Ok(Value::undefined())
}
/// Get a string's length
pub fn get_string_length(_:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    let this_str: String = from_value(this).unwrap();
    Ok(to_value::<i32>(this_str.len() as i32))
}
/// Create a new `String` object
pub fn _create(global: Value) -> Value {
    let string = Function::make(make_string, ["string"]);
    let proto = Value::new_obj(Some(global));
    let prop = Property {
        configurable: false,
        enumerable: false,
        writable: false,
        value: Value::undefined(),
        get: Function::make(get_string_length, []),
        set: Value::undefined()
    };
    proto.set_prop("length", prop);
    string.set_field(PROTOTYPE, proto);
    string
}
/// Initialise the `String` object on the global object
pub fn init(global:Value) {
    global.set_field("String", _create(global));
}