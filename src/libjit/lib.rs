/* Copyright (c) 2014, Peter Nelson
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without 
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, 
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#![crate_id = "jit#0.1.2"]
#![comment = "LibJIT Bindings"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![allow(raw_pointer_deriving, dead_code, non_camel_case_types)]
#![feature(globs, macro_rules)]
#![stable]
//! This crate wraps LibJIT

extern crate libc;
extern crate native;
extern crate syntax;
use std::ptr;
use std::mem::transmute;
use libc::{c_int, c_void, c_uint};
use bindings::*;
pub use bindings::{jit_nint, jit_nuint};

macro_rules! native_ref(
	($name:ident, $field:ident, $pointer_ty:ty) => (
		pub struct $name {
			$field: *mut c_void
		}
		impl NativeRef for $name {
			#[inline]
			/// Convert to a native pointer
			unsafe fn as_ptr(&self) -> $pointer_ty {
				self.$field
			}
			#[inline]
			/// Convert from a native pointer
			unsafe fn from_ptr(ptr:$pointer_ty) -> $name {
				$name {
					$field: ptr
				}
			}
		}
	)
)

/// A native reference
pub trait NativeRef {
	/// Returns an unsafe mutable pointer to the object
	unsafe fn as_ptr(&self) -> *mut c_void;
	/// Make an unsafe pointer to the object
	unsafe fn from_ptr(ptr:*mut c_void) -> Self;
}
/// A platform's application binary interface
pub enum ABI {
	/// The C application binary interface
	CDECL = 0
}
/// Call flags to a function
pub enum CallFlags {
	/// When the function won't throw a value
	JitCallNothrow = 1,
	/// When the function won't return a value
	JitCallNoReturn = 2,
	/// When the function is tail-recursive
	JitCallTail = 4,
}
mod bindings;
/// Holds all of the functions you have built and compiled. There can be multiple, but normally there is only one.
native_ref!(Context, _context, jit_context_t)
impl Context {
	/// Create a new JIT Context
	pub fn new() -> Context {
		unsafe {
			NativeRef::from_ptr(jit_context_create())
		}
	}
	/// Run a closure that can generate IR
	pub fn build<R>(&self, cb: || -> R) -> R {
		unsafe {
			jit_context_build_start(self.as_ptr());
		}
		let rv = cb();
		unsafe {
			jit_context_build_end(self.as_ptr());
		}
		rv
	}
}

impl Drop for Context {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			jit_context_destroy(self.as_ptr());
		}
	}
}

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
			let signature = jit_type_create_signature(abi as jit_abi_t, return_type.as_ptr(), transmute(params.as_mut_ptr()), params.len() as c_uint, 1);
			NativeRef::from_ptr(signature)
		}
	}

	fn create_complex(fields: &mut [&Type], union: bool) -> Type {
		unsafe {
			let f = if union { jit_type_create_union } else { jit_type_create_struct };
			let ty = f(transmute(fields.as_mut_ptr()), fields.len() as c_uint, 1);
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
}
#[deriving(Clone)]
/// A function to JIT compile
native_ref!(Function, _function, jit_function_t)
impl Drop for Function {
	fn drop(&mut self) {
		unsafe {
			jit_function_abandon(self.as_ptr());
		}
	}
}
impl Function {
	/// Create a function in the context with the type signature given
	pub fn new(context:&Context, signature: &Type) -> Function {
		unsafe {
			NativeRef::from_ptr(jit_function_create(context.as_ptr(), signature.as_ptr()))
		}
	}
	fn insn_binop(&self, v1: &Value, v2: &Value, f: unsafe extern "C" fn(function: jit_function_t, v1: jit_value_t, v2: jit_value_t) -> jit_value_t) -> Value {
		unsafe {
			let value = f(self.as_ptr(), v1.as_ptr(), v2.as_ptr());
			NativeRef::from_ptr(value)
		}
	}

	fn insn_unop(&self, value: &Value, f: unsafe extern "C" fn(function: jit_function_t, value: jit_value_t) -> jit_value_t) -> Value {
		unsafe {
			let value = f(self.as_ptr(), value.as_ptr());
			NativeRef::from_ptr(value)
		}
	}
	/// Get the context this function was made i
	pub fn get_context(&self) -> Context {
		unsafe {
			let context = jit_function_get_context(self.as_ptr());
			NativeRef::from_ptr(context)
		}
	}
	/// Set the optimization level of the function, where the bigger the level, the more effort should be spent optimising
	pub fn set_optimization_level(&self, level: c_uint) {
		unsafe {
			jit_function_set_optimization_level(self.as_ptr(), level);
		}
	}
	/// Make this funcition a candidate for recompilation
	pub fn set_recompilable(&self) {
		unsafe {
			jit_function_set_recompilable(self.as_ptr());
		}
	}
	/// Compile the function
	pub fn compile(&self) {
		unsafe {
			jit_function_compile(self.as_ptr());
		}
	}
	/// Get a parameter of the function as a JIT Value
	pub fn get_param(&self, param: uint) -> Value {
		unsafe {
			let value = jit_value_get_param(self.as_ptr(), param as c_uint);
			NativeRef::from_ptr(value)
		}
	}
	/// Make an instructional representation of a Rust value
	pub fn insn_of<T:Compilable>(&self, val:&T) -> Value {
		val.compile(self)
	}
	/// Notify libjit that this function has a catch block in it so it can prepare
	pub fn insn_uses_catcher(&self) {
		unsafe {
			jit_insn_uses_catcher(self.as_ptr());
		}
	}
	/// Throw an exception from the function with the value given
	pub fn insn_throw(&self, retval: &Value) {
		unsafe {
			jit_insn_throw(self.as_ptr(), retval.as_ptr());
		}
	}
	/// Return from the function with the value given
	pub fn insn_return(&self, retval: &Value) {
		unsafe {
			jit_insn_return(self.as_ptr(), retval.as_ptr());
		}
	}
	/// Return from the function
	pub fn insn_default_return(&self) {
		unsafe {
			jit_insn_default_return(self.as_ptr());
		}
	}
	/// Make an instruction that multiplies the values
	pub fn insn_mul(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_mul)
	}
	/// Make an instruction that adds the values
	pub fn insn_add(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_add)
	}
	/// Make an instruction that subtracts the second value from the first
	pub fn insn_sub(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_sub)
	}
	/// Make an instruction that divides the first number by the second
	pub fn insn_div(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_div)
	}
	/// Make an instruction that finds the remainder when the first number is divided by the second
	pub fn insn_rem(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_rem)
	}
	/// Make an instruction that checks if the first value is lower than or equal to the second
	pub fn insn_leq(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_le)
	}
	/// Make an instruction that checks if the first value is greater than or equal to the second
	pub fn insn_geq(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_ge)
	}
	/// Make an instruction that checks if the first value is lower than the second
	pub fn insn_lt(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_lt)
	}
	/// Make an instruction that checks if the first value is greater than the second
	pub fn insn_gt(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_gt)
	}
	/// Make an instruction that checks if the values are equal
	pub fn insn_eq(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_eq)
	}
	/// Make an instruction that checks if the values are not equal
	pub fn insn_neq(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_ne)
	}
	/// Make an instruction that performs a bitwise and on the two values
	pub fn insn_and(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_and)
	}
	/// Make an instruction that performs a bitwise or on the two values
	pub fn insn_or(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_or)
	}
	/// Make an instruction that performs a bitwise xor on the two values
	pub fn insn_xor(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_xor)
	}
	/// Make an instruction that performs a bitwise not on the two values
	pub fn insn_not(&self, value: &Value) -> Value {
		self.insn_unop(value, jit_insn_not)
	}
	/// Make an instruction that performs a left bitwise shift on the first value by the second value
	pub fn insn_shl(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_shl)
	}
	/// Make an instruction that performs a right bitwise shift on the first value by the second value
	pub fn insn_shr(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_shr)
	}
	/// Make an instruction that performs a right bitwise shift on the first value by the second value
	pub fn insn_ushr(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_ushr)
	}
	/// Make an instruction that performs a bitwise negate on the value
	pub fn insn_neg(&self, value: &Value) -> Value {
		self.insn_unop(value, jit_insn_neg)
	}
	/// Make an instruction that duplicates the value given
	pub fn insn_dup(&self, value: &Value) -> Value {
		unsafe {
			let dup_value = jit_insn_load(self.as_ptr(), value.as_ptr());
			NativeRef::from_ptr(dup_value)
		}
	}
	/// Make an instruction that loads a value from a src value
	pub fn insn_load(&self, src: &Value) -> Value {
		self.insn_unop(src, jit_insn_load)
	}
	/// Make an instruction that stores a value at a destination value
	pub fn insn_store(&self, dest: &Value, src: &Value) {
		unsafe {
			jit_insn_store(self.as_ptr(), dest.as_ptr(), src.as_ptr());
		}
	}
	/// Make an instruction that stores a value a certain offset away from a destination value
	pub fn insn_store_relative(&self, dest: &Value, offset: int, src: &Value) {
		unsafe {
			jit_insn_store_relative(self.as_ptr(), dest.as_ptr(), offset as jit_nint, src.as_ptr());
		}
	}
	/// Make an instruction that sets a label
	pub fn insn_set_label(&self, label: &mut Label) {
		unsafe {
			jit_insn_label(self.as_ptr(), &mut label._label);
		}
	}
	/// Make an instruction that branches to a certain label
	pub fn insn_branch(&self, label: &mut Label) {
		unsafe {
			jit_insn_branch(self.as_ptr(), &mut label._label);
		}
	}
	/// Make an instruction that branches to a certain label if the value is true
	pub fn insn_branch_if(&self, value: &Value, label: &mut Label) {
		unsafe {
			jit_insn_branch_if(self.as_ptr(), value.as_ptr(), &mut label._label);
		}
	}
	/// Make an instruction that branches to a certain label if the value is false
	pub fn insn_branch_if_not(&self, value: &Value, label: &mut Label) {
		unsafe {
			jit_insn_branch_if_not(self.as_ptr(), value.as_ptr(), &mut label._label);
		}
	}
	/// Make an instruction that branches to a label in the table
	pub fn insn_jump_table(&self, value: &Value, labels: &mut [Label]) {
		unsafe {
			let labels_ptr: *mut jit_label_t = transmute(labels.as_mut_ptr());
			jit_insn_jump_table(self.as_ptr(), value.as_ptr(), labels_ptr, labels.len() as u32);
		}
	}
	/// Make an instruction that calls a function that has the signature given with some arguments
	pub fn insn_call_indirect(&self, func:&Function, signature: &Type, args: &mut [&Value]) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_insn_call_indirect(self.as_ptr(), func.as_ptr(), signature.as_ptr(), transmute(args.as_mut_ptr()), args.len() as c_uint, JitCallNothrow as c_int))
		}
	}
	/// Make an instruction that calls a native function that has the signature given with some arguments
	fn insn_call_native(&self, name: &'static str, native_func: *mut c_void,
						signature: &Type, args: &mut [&Value]) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_insn_call_native(self.as_ptr(), name.to_c_str().unwrap(), native_func,
											signature.as_ptr(), transmute(args.as_mut_ptr()), args.len() as c_uint,
											JitCallNothrow as c_int))
		}
	}
	/// Make an instruction that calls a Rust function that has the signature given with no arguments and expects a return value
	pub fn insn_call_native0<R>(&self, name: &'static str,
								native_func: fn() -> R,
								signature: &Type, args: &mut [&Value]) -> Value {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with a single argument and expects a return value
	pub fn insn_call_native1<A,R>(&self, name: &'static str,
								  native_func: fn(A) -> R,
								  signature: &Type, args: &mut [&Value]) -> Value {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with two arguments and expects a return value
	pub fn insn_call_native2<A,B,R>(&self, name: &'static str,
								  native_func: fn(A, B) -> R,
								  signature: &Type, args: &mut [&Value]) -> Value {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with three arguments and expects a return value
	pub fn insn_call_native3<A,B,C,R>(&self, name: &'static str,
								  native_func: fn(A, B, C) -> R,
								  signature: &Type, args: &mut [&Value]) -> Value {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with four arguments and expects a return value
	pub fn insn_call_native4<A,B,C,D,R>(&self, name: &'static str,
								  native_func: fn(A, B, C, D) -> R,
								  signature: &Type, args: &mut [&Value]) -> Value {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that allocates some space
	pub fn insn_alloca(&self, size: &Value) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_insn_alloca(self.as_ptr(), size.as_ptr()))
		}
	}
	/// Apply a function to some arguments and set the retval to the return value
	pub fn apply<T>(&self, args: &mut [*mut c_void], retval: &mut T) {
		unsafe {
			jit_function_apply(self.as_ptr(), args.as_mut_ptr(), transmute(retval));
		}
	}
	/// Execute a function and with some arguments
	pub fn execute(&self, args: &mut [*mut c_void]) {
		unsafe {
			jit_function_apply(self.as_ptr(), args.as_mut_ptr(), ptr::mut_null());
		}
	}
	/// Turn this function into a closure
	pub fn closure<T>(&self) -> T {
		unsafe {
			transmute(jit_function_to_closure(self.as_ptr()))
		}
	}
	/// Create a new value with the given type
	pub fn create_value(&self, value_type: &Type) -> Value {
		unsafe {
			let value = jit_value_create(self.as_ptr(), value_type.as_ptr());
			NativeRef::from_ptr(value)
		}
	}
	pub fn insn_convert(&self, v: &Value, t:&Type, overflow_check:bool) -> Value {
		unsafe {
			NativeRef::from_ptr(jit_insn_convert(self.as_ptr(), v.as_ptr(), t.as_ptr(), overflow_check as c_int))
		}
	}
	/// Make an instruction that gets the inverse cosine of the number given
	pub fn insn_acos(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_acos)
	}
	/// Make an instruction that gets the inverse sine of the number given
	pub fn insn_asin(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_asin)
	}
	/// Make an instruction that gets the inverse tangent of the number given
	pub fn insn_atan(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_atan)
	}
	/// Make an instruction that gets the inverse tangent of the numbers given
	pub fn insn_atan2(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_atan2)
	}
	/// Make an instruction that finds the nearest integer above a number
	pub fn insn_ceil(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_ceil)
	}
	/// Make an instruction that gets the consine of the number given
	pub fn insn_cos(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_cos)
	}
	/// Make an instruction that gets the hyperbolic consine of the number given
	pub fn insn_cosh(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_cosh)
	}
	/// Make an instruction that gets the natural logarithm rased to the power of the number
	pub fn insn_exp(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_exp)
	}
	/// Make an instruction that finds the nearest integer below a number
	pub fn insn_floor(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_floor)
	}
	/// Make an instruction that gets the natural logarithm of the number
	pub fn insn_log(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_log)
	}
	/// Make an instruction that gets the base 10 logarithm of the number
	pub fn insn_log10(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_log10)
	}
	/// Make an instruction the gets the result of raising the first value to the power of the second value
	pub fn insn_pow(&self, v1: &Value, v2:&Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_pow)
	}
	/// Make an instruction the gets the result of rounding the value to the nearest integer
	pub fn insn_rint(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_rint)
	}
	/// Make an instruction the gets the result of rounding the value to the nearest integer
	pub fn insn_round(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_round)
	}
	/// Make an instruction the gets the sine of the number
	pub fn insn_sin(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_sin)
	}
	/// Make an instruction the gets the hyperbolic sine of the number
	pub fn insn_sinh(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_sinh)
	}
	/// Make an instruction the gets the square root of a number
	pub fn insn_sqrt(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_sqrt)
	}
	/// Make an instruction the gets the tangent of a number
	pub fn insn_tan(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_tan)
	}
	/// Make an instruction the gets the hyperbolic tangent of a number
	pub fn insn_tanh(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_tanh)
	}
	/// Make an instruction that truncates the value
	pub fn insn_trunc(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_trunc)
	}
	/// Make an instruction that checks if the number is NaN
	pub fn insn_is_nan(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_is_nan)
	}
	/// Make an instruction that checks if the number is finite
	pub fn insn_is_finite(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_is_finite)
	}
	/// Make an instruction that checks if the number is  infinite
	pub fn insn_is_inf(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_is_inf)
	}
	/// Make an instruction that gets the absolute value of a number
	pub fn insn_abs(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_abs)
	}
	/// Make an instruction that gets the smallest of two numbers
	pub fn insn_min(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_min)
	}
	/// Make an instruction that gets the biggest of two numbers
	pub fn insn_max(&self, v1: &Value, v2: &Value) -> Value {
		self.insn_binop(v1, v2, jit_insn_max)
	}
	/// Make an instruction that gets the sign of a number
	pub fn insn_sign(&self, v: &Value) -> Value {
		self.insn_unop(v, jit_insn_sign)
	}
}
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