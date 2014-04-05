use collections::treemap::TreeMap;
use js::function::Function;
use js::value::{Value, VObject, VUndefined, VInteger, ResultValue};
use std::gc::Gc;
pub type ObjectData = TreeMap<~str, Value>;

/// Create a new 'JSON' object
pub fn _create() -> Value {
	let mut obj = TreeMap::new();
	VObject(Gc::new(obj))
}