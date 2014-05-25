use stdlib::object::{ObjectData, Property};
use stdlib::value::{Value, VInteger, ResultValue, to_value};
use syntax::ast::Expr;
use collections::treemap::TreeMap;
use std::gc::Gc;
pub type NativeFunctionData = fn(Value, Value, Vec<Value>) -> ResultValue;
#[deriving(Clone)]
/// A Javascript function
pub enum Function {
	/// A native javascript function
	NativeFunc(NativeFunction),
	/// A regular javascript function
	RegularFunc(RegularFunction)
}
#[deriving(Clone)]
/// Represents a regular javascript function in memory
pub struct RegularFunction {
	/// The fields associated with the function
	pub object : ObjectData,
	/// This function's expression
	pub expr : Expr,
	/// The argument names of the function
	pub args : Vec<StrBuf>
}
impl RegularFunction {
	/// Make a new regular function
	pub fn new(expr : Expr, args: Vec<StrBuf>) -> RegularFunction {
		let mut obj = TreeMap::new();
		obj.insert("arguments".into_strbuf(), Property::new(Gc::new(VInteger(args.len() as i32))));
		RegularFunction {object: obj, expr: expr, args: args}
	}
}
#[deriving(Clone)]
/// Represents a native javascript function in memory
pub struct NativeFunction {
	/// The fields associated with the function
	pub object : ObjectData,
	/// The callable function data
	pub data: NativeFunctionData
}
impl NativeFunction {
	/// Make a new native function with the given function data
	pub fn new(data : NativeFunctionData) -> NativeFunction {
		let obj = TreeMap::new();
		NativeFunction {object: obj, data: data}
	}
}
/// Create a new `Function` object
pub fn _create(_ : Value) -> Value {
	let function : ObjectData = TreeMap::new();
	to_value(function)
}
/// Initialise the global object with the `Function` object
pub fn init(global:Value) {
	let global_ptr = global.borrow();
	global_ptr.set_field_slice("Function", _create(global));
}