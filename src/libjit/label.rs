use bindings::{
	jit_label_t,
	jit_function_reserve_label
};
use function::Function;
use util::NativeRef;
#[deriving(PartialEq)]
/// A label in the code that can be branched to in instructions
pub struct Label {
	_label: jit_label_t
}
impl Label {
	/// Create a new label
	pub fn new(func:&Function) -> Label {
		unsafe {
			Label {
				_label: jit_function_reserve_label(func.as_ptr())
			}
		}
	}
	/// Get the value of this label
	#[inline]
	pub fn get_value(&self) -> jit_label_t {
		self._label
	}
}