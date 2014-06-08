use bindings::{
	jit_value_t,
	jit_value_get_type
};
use types::Type;
use util::NativeRef;
#[deriving(Clone)]
/// A Value that is being JIT compiled
native_ref!(Value, _value, jit_value_t)
impl Value {
	/// Get the type of the value
	pub fn get_type(&self) -> Type {
		unsafe {
			let ty = jit_value_get_type(self.as_ptr());
			NativeRef::from_ptr(ty)
		}
	}
}
