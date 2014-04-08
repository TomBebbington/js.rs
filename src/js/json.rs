use collections::treemap::TreeMap;
use js::function::Function;
use js::value::{Value, VObject, VUndefined, VInteger, ResultValue};
use std::gc::Gc;
use std::cell::RefCell;
pub type ObjectData = TreeMap<~str, Value>;

/// Create a new 'JSON' object
pub fn _create() -> Value {
	let mut obj = TreeMap::new();
	Gc::new(VObject(RefCell::new(obj)))
}