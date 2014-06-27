use front::stdlib::object::{ObjectData, Property};
use front::stdlib::value::{Value, VFunction, ResultValue, to_value};
use collections::treemap::TreeMap;
use std::iter::FromIterator;
use std::cell::RefCell;
pub type FunctionData = fn(Vec<Value>, Value, Value, Value) -> ResultValue;
#[deriving(Clone)]
/// A Javascript function
pub struct Function {
    /// The fields associated with the function
    pub object : ObjectData,
    /// This function's JIT representation
    pub repr : FunctionData,
    /// The argument names of the function
    pub args : Vec<String>
}
impl Function {
    /// Make a new function
    pub fn new(repr : FunctionData, args: Vec<String>) -> Function {
        let mut obj = TreeMap::new();
        obj.insert("arguments".into_string(), Property::new(to_value(args.len() as i32)));
        Function {object: obj, repr: repr, args: args}
    }
    /// Create a function from function data and arguments
    pub fn make(repr: FunctionData, args:&[&'static str]) -> Value {
        Value::new(VFunction(RefCell::new(Function::new(repr, FromIterator::from_iter(args.iter().map(|arg|arg.to_string()))))))
    }
    /// Call with some args
    pub fn call(&self, args: Vec<Value>, global:Value, scope:Value, this:Value) -> ResultValue {
        (self.repr)(args, global, scope, this)
    }
}
/// Create a new `Function` object
pub fn _create(_ : Value) -> Value {
    let function : ObjectData = TreeMap::new();
    to_value(function)
}
/// Initialise the global object with the `Function` object
pub fn init(global:Value) {
    js_extend!(global, {
        "Function": _create(global)
    });
}