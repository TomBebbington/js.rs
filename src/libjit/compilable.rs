use bindings::*;
use function::Function;
use types::Types;
use value::Value;
use util::NativeRef;
/// A type that can be compiled into a LibJIT representation
pub trait Compilable {
	/// Get a JIT representation of this value
	fn compile(&self, func:&Function) -> Value;
}
impl Compilable for () {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_void_ptr, 0))
		}
	}
}
impl Compilable for f64 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_float64_constant(func.as_ptr(), jit_type_float64, *self) )
		}
	}
}
impl Compilable for f32 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_float32_constant(func.as_ptr(), jit_type_float32, *self) )
		}
	}
}
impl Compilable for int {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_long_constant(func.as_ptr(), jit_type_nint, *self as i64) )
		}
	}
}
impl Compilable for uint {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_nuint, *self as jit_nint) )
		}
	}
}
impl Compilable for i32 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_int, *self as jit_nint) )
		}
	}
}
impl Compilable for u32 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_uint, *self as jit_nint) )
		}
	}
}
impl Compilable for i16 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_short, *self as jit_nint) )
		}
	}
}
impl Compilable for u16 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_ushort, *self as jit_nint) )
		}
	}
}
impl Compilable for i8 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_sbyte, *self as jit_nint) )
		}
	}
}
impl Compilable for u8 {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_ubyte, *self as jit_nint) )
		}
	}
}
impl Compilable for bool {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_sys_bool, *self as jit_nint) )
		}
	}
}
impl Compilable for char {
	fn compile(&self, func:&Function) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_value_create_nint_constant(func.as_ptr(), jit_type_ubyte, *self as jit_nint) )
		}
	}
}
impl<'t> Compilable for &'t str {
	fn compile(&self, func:&Function) -> Value {
		let cstring_t = Types::get_cstring();
		let strlen_i = (self.len() as i32).compile(func);
		let bufptr = func.create_value(&cstring_t);
		func.insn_store(&bufptr, &func.insn_alloca(&strlen_i));
		for i in range(0, self.len()) {
			let char_i = self.char_at(i).compile(func);
			func.insn_store_relative(&bufptr, i as int, &char_i);
		}
		let null_term = '\0'.compile(func);
		func.insn_store_relative(&bufptr, self.len() as int, &null_term);
		bufptr
	}
}
impl Compilable for String {
	fn compile(&self, func:&Function) -> Value {
		self.as_slice().compile(func)
	}
}