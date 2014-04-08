use js::function::{NativeFunc, NativeFunction};
use js::value::{Value, VObject, VString, VFunction, ResultValue};
use collections::treemap::TreeMap;
use serialize::json::ToJson;
use std::gc::Gc;
use std::cell::RefCell;
/// Turn an object into a string
pub fn stringify(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	let json = (obj.borrow() as &ToJson).to_json();
	Ok(Gc::new(VString(json.to_pretty_str())))
}
/// Create a new 'JSON' object
pub fn _create() -> Value {
	let mut obj = TreeMap::new();
	obj.insert(~"stringify", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(stringify, 1))))));
	Gc::new(VObject(RefCell::new(obj)))
}