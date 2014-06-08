use bindings::*;
use function::ABI;
use libc::c_uint;
use std::kinds::marker::ContravariantLifetime;
use std::str::raw::from_c_str;
use util::NativeRef;
/// The types that a value can be
bitflags!(
	flags TypeKind: i32 {
		static Invalid		= -1,
		static Void			= 0,
		static SByte		= 1,
		static UByte		= 2,
		static Short		= 3,
		static UShort		= 4,
		static Int			= 5,
		static UInt 		= 6,
		static NInt 		= 7,
		static NUInt 		= 8,
		static Long 		= 9,
		static ULong 		= 10,
		static Float32 		= 11,
		static Float64 		= 12,
		static NFloat 		= 13,
		static MaxPrimitive = 13,
		static Struct 		= 14,
		static Union 		= 15,
		static Signature 	= 16,
		static Pointer 		= 17,
		static FirstTagged	= 32,
		static SysBool 		= 10009,
		static SysChar 		= 10010
	}
)
/// A type field iterator
pub struct Fields<'a> {
	_type: jit_type_t,
	index: c_uint,
	marker: ContravariantLifetime<'a>
}
impl<'a> Fields<'a> {
	fn new(ty:&'a Type) -> Fields<'a> {
		unsafe {
			Fields {
				_type: ty.as_ptr(),
				index: 0 as c_uint,
				marker: ContravariantLifetime::<'a>
			}
		}
	}
}
impl<'a> Iterator<(String, Type)> for Fields<'a> {
	fn next(&mut self) -> Option<(String, Type)> {
		unsafe {
			let index = self.index;
			self.index += 1;
			if index < jit_type_num_fields(self._type) {
				let name = from_c_str(jit_type_get_name(self._type, index));
				let native_field = jit_type_get_field(self._type, index);
				if name.len() == 0 || native_field.is_null() {
					None
				} else {
					let field:Type = NativeRef::from_ptr(native_field);
					Some((name, field))
				}
			} else {
				None
			}
		}
	}
}
/// A type of a value to JIT compile
native_ref!(Type, _type, jit_type_t)
impl Clone for Type {
	#[inline]
	fn clone(&self) -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_copy(self.as_ptr()))
		}
	}
}
impl Drop for Type {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			jit_type_free(self.as_ptr());
		}
	}
}
impl Type {
	/// Create a function signature, with the given ABI, return type and parameters
	pub fn create_signature(abi: ABI, return_type: &Type, params: &mut [&Type]) -> Type {
		unsafe {
			let mut native_params:Vec<jit_type_t> = params.iter().map(|param| param.as_ptr()).collect();
			let signature = jit_type_create_signature(abi as jit_abi_t, return_type.as_ptr(), native_params.as_mut_ptr(), params.len() as c_uint, 1);
			NativeRef::from_ptr(signature)
		}
	}

	fn create_complex(fields: &mut [&Type], union: bool) -> Type {
		unsafe {
			let mut native_fields:Vec<jit_type_t> = fields.iter().map(|field| field.as_ptr()).collect();
			let f = if union { jit_type_create_union } else { jit_type_create_struct };
			let ty:jit_type_t = f(native_fields.as_mut_ptr(), fields.len() as c_uint, 1);
			NativeRef::from_ptr(ty)
		}
	}
	/// Create a struct type with the given field types
	pub fn create_struct(fields: &mut [&Type]) -> Type {
		Type::create_complex(fields, false)
	}
	/// Create a union type with the given field types
	pub fn create_union(fields: &mut [&Type]) -> Type {
		let inner = Type::create_complex(fields, true);
		Type::create_struct(&mut [&Types::get_int(), &inner])
	}
	/// Create a pointer type with the given pointee type
	pub fn create_pointer(pointee: &Type) -> Type {
		unsafe {
			let ptr = jit_type_create_pointer(pointee.as_ptr(), 1);
			NativeRef::from_ptr(ptr)
		}
	}
	/// Work out the size of this type
	pub fn get_size(&self) -> jit_nuint {
		unsafe {
			jit_type_get_size(self.as_ptr())
		}
	}
	/// Get the kind of this type
	pub fn get_kind(&self) -> TypeKind {
		unsafe {
			TypeKind::from_bits(jit_type_get_kind(self.as_ptr())).unwrap()
		}
	}
	/// Get the reference this pointer points to
	pub fn get_ref(&self) -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_get_ref(self.as_ptr()))
		}
	}
	/// Set the field names of this type
	pub fn set_names(&self, names:&[String]) -> bool {
		unsafe {
			let native_names : Vec<*i8> = names.iter().map(|name| name.to_c_str().unwrap()).collect();
			jit_type_set_names(self.as_ptr(), native_names.as_ptr() as *mut *mut i8, names.len() as u32) != 0
		}
	}
	/// Iterator over the type's fields
	pub fn iter_fields<'a>(&'a self) -> Fields<'a> {
		Fields::new(self)
	}
}

/// Holds type constructors
pub struct Types;
impl Types {
	#[inline]
	/// Void type
	pub fn get_void() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_void)
		}
	}
	#[inline]
	/// Integer type
	pub fn get_int() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_int)
		}
	}
	#[inline]
	/// Unsigned integer type
	pub fn get_uint() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_uint)
		}
	}
	#[inline]
	/// Long integer type
	pub fn get_long() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_long)
		}
	}
	#[inline]
	/// Unsigned long integer type
	pub fn get_ulong() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_ulong)
		}
	}
	#[inline]
	/// 32-bit floating point type
	pub fn get_float32() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_float32)
		}
	}
	#[inline]
	/// 64-bit floating point type
	pub fn get_float64() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_float64)
		}
	}
	#[inline]
	/// Default floating point type
	pub fn get_float() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_nfloat)
		}
	}
	#[inline]
	/// A void pointer, which can represent any kind of pointer
	pub fn get_void_ptr() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_void_ptr)
		}
	}
	#[inline]
	/// Character type
	pub fn get_char() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_sys_char)
		}
	}
	#[inline]
	/// C String type
	pub fn get_cstring() -> Type {
		Type::create_pointer(&Types::get_char())
	}
	#[inline]
	/// Boolean type
	pub fn get_bool() -> Type {
		unsafe {
			NativeRef::from_ptr(jit_type_sys_bool)
		}
	}
}