use js::object::{ObjectData, Property};
use js::value::{Value, VInteger, VObject, ResultValue, to_value};
use ast::Expr;
use collections::treemap::TreeMap;
use std::gc::Gc;
use exec::Executor;
use std::cell::RefCell;
pub type NativeFunctionData = fn(Value, Value, Vec<Value>) -> ResultValue;
#[deriving(Clone)]
/// A Javascript function
pub enum Function {
	/// A native javascript function
	NativeFunc(NativeFunction),
	/// A regular javascript function
	RegularFunc(RegularFunction)
}
impl Function {
	/// Call a function with some arguments
	pub fn call(&self, exe:&mut Executor, this:Value, callee:Value, args:Vec<Value>) -> ResultValue {
		match *self {
			NativeFunc(ref ntv) => {
				let func = ntv.data;
				func(this, callee, args)
			}, RegularFunc(ref data) => {
				let scope = exe.make_scope();
				scope.borrow().borrow_mut().insert(~"this", Property::new(this));
				for i in range(0, data.args.len()) {
					let name = data.args.get(i);
					let expr = args.get(i);
					scope.borrow().borrow_mut().insert(name.clone(), Property::new(*expr));
				}
				let result = exe.run(&data.expr);
				exe.destroy_scope();
				result
			}
		}
	}
}
#[deriving(Clone)]
/// Represents a regular javascript function in memory
pub struct RegularFunction {
	/// The fields associated with the function
	pub object : ObjectData,
	/// This function's expression
	pub expr : Expr,
	/// The arguments
	pub args : Vec<~str>
}
impl RegularFunction {
	/// Make a new regular function
	pub fn new(expr : Expr, args: Vec<~str>) -> RegularFunction {
		let mut obj = TreeMap::new();
		obj.insert(~"arguments", Property::new(Gc::new(VInteger(args.len() as i32))));
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
		let mut obj = TreeMap::new();
		NativeFunction {object: obj, data: data}
	}
}
/// Create a new 'Function' object
pub fn _create() -> Value {
	let mut function : ObjectData = TreeMap::new();
	to_value(function)
}