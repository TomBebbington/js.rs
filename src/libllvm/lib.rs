#![crate_id = "llvm"]
#![comment = "LLVM Bindings"]
#![crate_type = "dylib"]

#![doc(
	html_favicon_url = "http://tombebbington.github.io/favicon.png",
	html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![experimental]
#![feature(globs, macro_rules, struct_inherit, link_args)]
#![deny(non_uppercase_statics, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, unused_must_use)]
#![allow(non_camel_case_types)]
//! This crate provides LLVM Bindings
extern crate libc;
extern crate collections;
use llvm::*;
use llvm::llvm::*;
use libc::{c_ulonglong, c_char, c_uint, c_double, c_float};
use std::c_str::{CString, ToCStr};
use std::str::raw::from_c_str;
use std::ptr::RawPtr;
use std::fmt::{Show, Formatter};
use FmtResult = std::fmt::Result;
use std::iter::FromIterator;
pub use llvm::{TypeKind, Void, Float, Double, Integer, Function,};
mod llvm;
mod llvmdeps;
/// A dumpable value
pub trait Dumpable {
	/// Print a string representation of this value onto the standard output stream
	fn dump(&self) -> ();
}
/// A value that is inside a context
pub trait InContext {
	/// Get the context this value is inside
	fn get_context(&self) -> Box<Context>;
}
#[deriving(Clone, Eq)]
/// This is an important class for using LLVM in a threaded context. It (opaquely) owns and manages the core "global" data of LLVM's core infrastructure, including the type and constant uniquing tables.
pub struct Context {
	_ref: ContextRef
}
impl Context {
	/// Create a new context
	pub fn new() -> Box<Context> {
		unsafe {
			box Context {
				_ref: LLVMContextCreate()
			}
		}
	}
}
impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			LLVMContextDispose(self._ref);
		}
	}
}
/// The main container class for the LLVM Intermediate Representation.
/// A Module instance is used to store all the information related to an LLVM module.
/// Modules are the top level container of all other LLVM Intermediate Representation
/// (IR) objects. Each module directly contains a list of globals variables, a list
/// of functions, a list of libraries (or other modules) this module depends on, a
/// symbol table, and various data about the target's characteristics.
#[deriving(Eq)]
pub struct Module {
	_ref:ModuleRef
}
impl Module {
	/// Create a new module with the given ID in the given context
	pub fn new(id: &str, context: &Context) -> Box<Module> {
		unsafe {
			box Module {
				_ref: LLVMModuleCreateWithNameInContext(id.to_c_str().unwrap(), context._ref)
			}
		}
	}
	/// Add a function in the module with the name and signature given
	pub fn add_function(&self, name: &str, signature:&Type) -> Box<Value> {
		unsafe {
			box Value {
				_ref: LLVMAddFunction(self._ref, name.to_c_str().unwrap(), signature._ref)
			}
		}
	}
	/// Get a named function
	pub fn find_function(&self, name: &str) -> Option<Box<Value>> {
		unsafe {
			let _ref = LLVMGetNamedFunction(self._ref, name.to_c_str().unwrap());
			if _ref.is_null() {
				None
			} else {
				Some(box Value {
					_ref: _ref
				})
			}
		}
	}
	/// Get the data layout for the module's target platform
	pub fn get_data_layout(&self) -> String {
		unsafe {
			from_c_str(LLVMGetDataLayout(self._ref))
		}
	}
	/// Set the data layout for the module's target platform
	pub fn set_data_layout(&self, layout: String) {
		unsafe {
			LLVMSetDataLayout(self._ref, layout.to_c_str().unwrap())
		}
	}
	/// Get the target triple which is a string describing the target host. 
	pub fn get_target_triple(&self) -> String {
		unsafe {
			from_c_str(LLVMGetTarget(self._ref))
		}
	}
	/// Set the target triple which is a string describing the target host. 
	pub fn set_target_triple(&self, layout: String) {
		unsafe {
			LLVMSetTarget(self._ref, layout.to_c_str().unwrap())
		}
	}
}
impl InContext for Module {
	fn get_context(&self) -> Box<Context> {
		unsafe {
			box Context {
				_ref: LLVMGetModuleContext(self._ref)
			}
		}
	}
}
impl Dumpable for Module {
	fn dump(&self) {
		unsafe {
			LLVMDumpModule(self._ref);
		}
	}
}
impl Show for Module {
	fn fmt(&self, fmt:&mut Formatter) -> FmtResult {
		unsafe {
			let as_str = LLVMPrintModuleToString(self._ref);
			let result = write!(fmt, "{}", from_c_str(as_str).clone());
			LLVMDisposeMessage(as_str);
			result
		}
	}
}
impl Drop for Module {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeModule(self._ref)
		}
	}
}
/// The instances of the Type class are immutable: once they are
/// created, they are never changed. Also note that only one
/// instance of a particular type is ever created. Thus seeing
/// if two types are equal is a matter of doing a trivial
/// pointer comparison. To enforce that no two equal instances
/// are created, Type instances can only be created via static
/// factory methods in class Type and in derived classes. Once
/// allocated, Types are never free'd. 
pub struct Type {
	_ref: TypeRef
}
impl Type {
	/// Get the kind of this type
	pub fn get_kind(&self) -> TypeKind {
		unsafe {
			LLVMGetTypeKind(self._ref)
		}
	}
	/// Create a new pointer to the type given
	pub fn create_pointer(ty:&Type) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMPointerType(ty._ref, 0)
			}
		}
	}
	/// Create a new function signature
	pub fn create_signature(ret:&Type, args:&[&Type], is_var_arg:bool) -> Box<Type> {
		let nargs:Vec<*Type_opaque> = FromIterator::from_iter(args.iter().map(|arg|arg._ref));
		unsafe {
			box Type {
				_ref: LLVMFunctionType(ret._ref, nargs.as_ptr(), args.len() as u32, is_var_arg as Bool)
			}
		}
	}
	/// Get the int type width
	pub fn get_int_width(&self) -> c_uint {
		unsafe {
			LLVMGetIntTypeWidth(self._ref)
		}
	}
	/// Get the element type
	pub fn get_elem_type(&self) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMGetElementType(self._ref)
			}
		}
	}
}
impl Show for Type {
	fn fmt(&self, fmt:&mut Formatter) -> FmtResult {
		unsafe {
			let as_str = LLVMPrintTypeToString(self._ref);
			let result = write!(fmt, "{}", from_c_str(as_str).clone());
			LLVMDisposeMessage(as_str);
			result
		}
	}
}
impl InContext for Type {
	fn get_context(&self) -> Box<Context> {
		unsafe {
			box Context {
				_ref: LLVMGetTypeContext(self._ref)
			}
		}
	}
}
/// The base class of all values computed by a program that may be used as operands to other values
#[deriving(Eq)]
pub struct Value {
	_ref: ValueRef
}
impl Value {
	/// Return a constant reference to the value's name
	pub fn get_name(&self) -> String {
		unsafe {
			from_c_str(LLVMGetValueName(self._ref))
		}
	}
	/// Set the value's name
	pub fn set_name(&self, name: &str) {
		unsafe {
			LLVMSetValueName(self._ref, name.to_c_str().unwrap())
		}
	}
	/// Get the value type
	pub fn get_type(&self) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMTypeOf(self._ref)
			}
		}
	}
	/// Check if the value is a constant
	pub fn is_constant(&self) -> bool {
		unsafe {
			LLVMIsConstant(self._ref) == TRUE
		}
	}
}
impl Show for Value {
	fn fmt(&self, fmt:&mut Formatter) -> FmtResult {
		unsafe {
			let as_str = LLVMPrintValueToString(self._ref);
			let result = write!(fmt, "{}", from_c_str(as_str).clone());
			LLVMDisposeMessage(as_str);
			result
		}
	}
}
impl Dumpable for Value {
	fn dump(&self) {
		unsafe {
			LLVMDumpValue(self._ref);
		}
	}
}
/// An IR builder
pub struct Builder {
	_ref: BuilderRef
}
impl Builder {
	/// Create a new IR builder in the given context
	pub fn new(context: &Context) -> Box<Builder> {
		unsafe {
			box Builder {
				_ref: LLVMCreateBuilderInContext(context._ref)
			}
		}
	}
	/// Insert an instruction into the builder
	pub fn insert(&self, insn:&Value) -> () {
		unsafe {
			LLVMInsertIntoBuilder(self._ref, insn._ref)
		}
	}
	/// Create a new instruction that adds the left and right integer operands
	pub fn insn_add(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildAdd)
	}
	/// Create a new instruction that adds the left and right integer operands
	pub fn insn_nsw_add(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNSWAdd)
	}
	/// Create a new instruction that adds the left and right integer operands
	pub fn insn_nuw_add(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNUWAdd)
	}
	/// Create a new instruction that adds the left and right float operands
	pub fn insn_fadd(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildFAdd)
	}
	/// Create a new instruction that subtracts the right int operand from the left int operand
	pub fn insn_sub(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildSub)
	}
	/// Create a new instruction that subtracts the right int operand from the left int operand
	pub fn insn_nsw_sub(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNSWSub)
	}
	/// Create a new instruction that subtracts the right int operand from the left int operand
	pub fn insn_nuw_sub(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNUWSub)
	}
	/// Create a new instruction that subtracts the right float operand from the float int operand
	pub fn insn_fsub(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildFSub)
	}
	/// Create a new instruction that multiples the left and right integer operands
	pub fn insn_mul(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildMul)
	}
	/// Create a new instruction that multiples the left and right integer operands
	pub fn insn_nsw_mul(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNSWMul)
	}
	/// Create a new instruction that multiples the left and right integer operands
	pub fn insn_nuw_mul(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildNUWMul)
	}
	/// Create a new instruction that multiples the left and right float operands
	pub fn insn_fmul(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildFMul)
	}
	/// Create a new instruction that divides the first operand by the second operand as an unsigned integer
	pub fn insn_udiv(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildUDiv)
	}
	/// Create a new instruction that divides the first operand by the second operand as a signed integer
	pub fn insn_sdiv(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildSDiv)
	}
	/// Create a new instruction that divides the first operand by the second operand as a signed integer exactly
	pub fn insn_exact_sdiv(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildExactSDiv)
	}
	/// Create a new instruction that divides the first operand by the second operand as a float
	pub fn insn_fdiv(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildFDiv)
	}
	/// Create a new instruction that finds the remainder when the left operand is divided by the right operand as an unsigned int
	pub fn insn_urem(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildURem)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_srem(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildSRem)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_frem(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildFRem)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_shl(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildShl)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_lshr(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildLShr)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_ashr(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildAShr)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_and(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildAnd)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_or(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildOr)
	}
	/// Create a new instruction that adds the left and right operands
	pub fn insn_xor(&self, left:&Value, right:&Value) -> Box<Value> {
		self.binop(left, right, LLVMBuildXor)
	}
	/// Create a new instruction that returns a value
	pub fn insn_return(&self, val:&Value) -> Box<Value> {
		unsafe {
			box Value {
				_ref: LLVMBuildRet(self._ref, val._ref)
			}
		}
	}
	/// Create a new instruction that returns void
	pub fn insn_default_return(&self) -> Box<Value> {
		unsafe {
			box Value {
				_ref: LLVMBuildRetVoid(self._ref)
			}
		}
	}
	fn binop(&self, left:&Value, right:&Value, func:unsafe extern fn(BuilderRef, ValueRef, ValueRef, *c_char) -> ValueRef) -> Box<Value> {
		unsafe {
			box Value {
				_ref: func(self._ref, left._ref, right._ref, "result".to_c_str().unwrap())
			}
		}
	}
}
impl Drop for Builder {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeBuilder(self._ref)
		}
	}
}
/** Abstract interface for implementation execution of LLVM
modules, designed to support both interpreter and just-in-time
(JIT) compiler implementations */
pub struct ExecutionEngine {
	_ref: ExecutionEngineRef
}
impl ExecutionEngine {
	pub fn new(module:&Module) -> Box<ExecutionEngine> {
		unsafe {
			LLVMLinkInJIT();
			LLVMInitializeNativeTarget();
			let mut engine = RawPtr::null();
			let mut out_error = RawPtr::null();
			LLVMCreateJITCompilerForModule(&mut engine, module._ref, 0, &mut out_error);
			if out_error.is_null() {
				box ExecutionEngine {
					_ref: engine
				}
			} else {
				let out_error = from_c_str(out_error);
				fail!("LLVM error whilst creating execution engine: {}", out_error)
			}
		}
	}
}
impl Drop for ExecutionEngine {
	fn drop(&mut self) {
		unsafe {
			LLVMDisposeExecutionEngine(self._ref);
		}
	}
}
/// Holds type constructors
pub struct Types;
impl Types {
	/// Void type
	pub fn get_void(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMVoidTypeInContext(context._ref)
			}
		}
	}
	/// Character type
	pub fn get_char(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMInt8TypeInContext(context._ref)
			}
		}
	}
	/// Integer type
	pub fn get_int(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMInt32TypeInContext(context._ref)
			}
		}
	}
	/// Long integer type
	pub fn get_long(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMInt64TypeInContext(context._ref)
			}
		}
	}
	/// Float type
	pub fn get_float32(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMFloatTypeInContext(context._ref)
			}
		}
	}
	/// Double type
	pub fn get_float64(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMFloatTypeInContext(context._ref)
			}
		}
	}
	/// A void pointer, which can represent any kind of pointer
	pub fn get_void_ptr(context:&Context) -> Box<Type> {
		Type::create_pointer(&*Types::get_void(context))
	}
	/// C String type
	pub fn get_cstring(context:&Context) -> Box<Type> {
		Type::create_pointer(&*Types::get_char(context))
	}
	/// Boolean type
	pub fn get_bool(context:&Context) -> Box<Type> {
		unsafe {
			box Type {
				_ref: LLVMInt1TypeInContext(context._ref)
			}
		}
	}
}
/// A Rust value which can be compiled into an LLVM Value
pub trait Compilable {
	/// Compile this value into an LLVM Value
	fn compile(&self, context:&Context) -> Box<Value>;
}
impl Compilable for bool {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let bool_t = LLVMInt1TypeInContext(context._ref);
			box Value {
				_ref: LLVMConstInt(bool_t, *self as c_ulonglong, FALSE)
			}
		}
	}
}
impl Compilable for i16 {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let int_t = LLVMInt16TypeInContext(context._ref);
			box Value {
				_ref: LLVMConstInt(int_t, *self as c_ulonglong, FALSE)
			}
		}
	}
}
impl Compilable for i32 {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let int_t = LLVMInt32TypeInContext(context._ref);
			box Value {
				_ref: LLVMConstInt(int_t, *self as c_ulonglong, FALSE)
			}
		}
	}
}
impl Compilable for i64 {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let int_t = LLVMInt64TypeInContext(context._ref);
			box Value {
				_ref: LLVMConstInt(int_t, *self as c_ulonglong, FALSE)
			}
		}
	}
}
impl Compilable for int {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let int_t = LLVMInt32TypeInContext(context._ref);
			box Value {
				_ref: LLVMConstInt(int_t, *self as c_ulonglong, FALSE)
			}
		}
	}
}
impl Compilable for f32 {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let float_t = LLVMFloatTypeInContext(context._ref);
			box Value {
				_ref: LLVMConstReal(float_t, *self as c_double)
			}
		}
	}
}
impl Compilable for f64 {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let float_t = LLVMDoubleTypeInContext(context._ref);
			box Value {
				_ref: LLVMConstReal(float_t, *self)
			}
		}
	}
}
impl<'t> Compilable for &'t ToCStr {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let as_c_str = self.to_c_str();
			let len = as_c_str.len();
			box Value {
				_ref: LLVMConstStringInContext(context._ref, as_c_str.unwrap(), len as c_uint, FALSE)
			}
		}
	}
}
impl Compilable for String {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let as_c_str = self.to_c_str();
			let len = as_c_str.len();
			box Value {
				_ref: LLVMConstStringInContext(context._ref, as_c_str.unwrap(), len as c_uint, FALSE)
			}
		}
	}
}
impl<'t> Compilable for &'t str {
	fn compile(&self, context:&Context) -> Box<Value> {
		unsafe {
			let as_c_str = self.to_c_str();
			let len = as_c_str.len();
			box Value {
				_ref: LLVMConstStringInContext(context._ref, as_c_str.unwrap(), len as c_uint, FALSE)
			}
		}
	}
}