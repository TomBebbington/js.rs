use syntax::ast::expr::{Expr, ConstExpr, BlockExpr, TypeOfExpr, LocalExpr, VarDeclExpr, GetConstFieldExpr, GetFieldExpr, CallExpr, WhileLoopExpr, IfExpr, SwitchExpr, ObjectDeclExpr, ArrayDeclExpr, FunctionDeclExpr, ArrowFunctionDeclExpr, UnaryOpExpr, BinOpExpr, ConstructExpr, ReturnExpr, ThrowExpr, AssignExpr};
use syntax::ast::constant::{CNum, CInt, CString, CBool, CRegExp, CNull, CUndefined};
use syntax::ast::op::{OpSub, OpAdd, OpMul, OpDiv, OpMod};
use syntax::ast::op::{UnaryMinus, UnaryPlus, UnaryNot};
use syntax::ast::op::{BinNum, BinBit, BinLog, BinComp};
use syntax::ast::op::{BitAnd, BitOr, BitXor, BitShl, BitShr};
use syntax::ast::op::{LogAnd, LogOr};
use syntax::ast::op::{CompEqual, CompNotEqual, CompStrictEqual, CompStrictNotEqual, CompGreaterThan, CompGreaterThanOrEqual, CompLessThan, CompLessThanOrEqual};
use stdlib::value::{Value, ValueData, VNull, VUndefined, VString, VNumber, VInteger, VObject, VBoolean, VFunction, ResultValue, to_value, from_value, ToValue};
use stdlib::object::{INSTANCE_PROTOTYPE, PROTOTYPE};
use stdlib::function::{NativeFunc, RegularFunc, RegularFunction};
use stdlib::{console, math, object, array, function, json, number, error, uri, string};
use std::vec::Vec;
use std::gc::Gc;
use std::c_str::CString;
use std::mem::size_of;
use jit::{Context, Function, Type, Types, CDECL};
use jit;
/// A variable scope
pub struct Scope {
	/// The value of `this` in the scope
	pub this: Value,
	/// The variables declared in the scope
	pub vars: Value
}
/// An execution engine
pub trait Executor<T> {
	/// Make a new execution engine
	fn new() -> Self;
	/// Set a global variable called `name` with the value `val`
	fn set_global(&mut self, name:String, val:Value) -> Value;
	/// Resolve the global variable `name`
	fn get_global(&self, name:String) -> Value;
	/// Create a new scope and return it
	fn make_scope(&mut self, this:Value) -> Scope;
	/// Destroy the current scope
	fn destroy_scope(&mut self) -> Scope;
	/// Compile the expression
	fn compile(&self, expr:&Expr) -> Box<T>;
	/// Run an expression
	fn run(&mut self, comp:Box<T>) -> ResultValue;
}
/// A Javascript JIT compiler
pub struct JITCompiler {
	/// The JIT Context
	context: Box<Context>,
	/// An object representing the global object
	pub global: Value,
	/// The scopes
	pub scopes: Vec<Scope>
}
impl JITCompiler {
	#[inline(always)]
	/// Get the current scope
	pub fn scope(&self) -> Scope {
		*self.scopes.get(self.scopes.len() - 1)
	}
	fn with_builder<R>(&self, cb: || -> R) -> R {
		self.context.build_start();
		let rv = cb();
		self.context.build_end();
		rv
	}
}
impl Executor<Function> for JITCompiler {
	fn new() -> JITCompiler {
		let global = Value::new_obj(None);
		object::init(global);
		console::init(global);
		math::init(global);
		array::init(global);
		function::init(global);
		json::init(global);
		number::init(global);
		error::init(global);
		string::init(global);
		uri::init(global);
		JITCompiler {
			context: Context::new(),
			global: global,
			scopes: vec!(Scope {
				this: global,
				vars: global
			})
		}
	}
	#[inline(always)]
	fn set_global(&mut self, name:String, val:Value) -> Value {
		self.global.set_field(name, val)
	}
	#[inline(always)]
	fn get_global(&self, name:String) -> Value {
		self.global.get_field(name)
	}
	fn make_scope(&mut self, this:Value) -> Scope {
		let scope = Scope {
			this: this,
			vars: Value::new_obj(None)
		};
		self.scopes.push(scope);
		scope
	}
	#[inline(always)]
	fn destroy_scope(&mut self) -> Scope {
		self.scopes.pop().unwrap()
	}
	fn compile(&self, expr: &Expr) -> Box<Function> {
		self.with_builder(|| {
			fn compile_value(func:&Function, expr: &Expr) -> Box<jit::Value> {
				let create_undef_value = Value::undefined;
				let valuedata_t = Types::get_int();
				let valuedata_ptr_t = Type::create_pointer(&*valuedata_t);
				let value_t = Type::create_struct(&[&*valuedata_ptr_t]);
				let cstring_t = Type::create_pointer(&*Types::get_char());
				let create_value_sig = Type::create_signature(CDECL, &*value_t, &[]);
				let wrap_str = |text:&str| -> Box<jit::Value> {
					let strlen_i = func.constant_int32_as_type(text.len() as i32, &*Types::get_char());
					let bufptr = func.create_value(cstring_t);
					func.insn_store(bufptr, func.insn_alloca(&*strlen_i));
					for i in range(0, text.len()) {
						let char_i = func.constant_int32_as_type(text.char_at(i) as i32, &*Types::get_char());
						func.insn_store_relative(bufptr, i as i32, char_i);
					}
					let null_term = func.constant_int32_as_type(0i32, &*Types::get_char());
					func.insn_store_relative(bufptr, text.len() as i32, null_term);
					bufptr
				};
				let undefined = || func.insn_call_native0("undefined", create_undef_value, &*create_value_sig, &[]);
				let global = func.get_param(0);
				let scope = func.get_param(1);
				let this = func.get_param(2);
				match expr.def {
					ConstExpr(CNull) => {
						fn create_null_value() -> Value {
							Value {
								ptr: Gc::new(VNull)
							}
						}
						func.insn_call_native0("create_null_value", create_null_value, &*create_value_sig, &[])
					},
					ConstExpr(CUndefined) => {
						undefined()
					},
					ConstExpr(CBool(v)) => {
						let create_bool_value = to_value::<bool>;
						let val = func.constant_int32_as_type(v as i32, &*Types::get_bool());
						let create_bool_sig = Type::create_signature(CDECL, &*value_t, &[&*Types::get_bool()]);
						func.insn_call_native1("create_bool_value", create_bool_value, &*create_bool_sig, &[&*val])
					},
					ConstExpr(CNum(n)) => {
						let create_number_value = to_value::<f64>;
						let val = func.constant_float64(n);
						let create_number_sig = Type::create_signature(CDECL, &*value_t, &[&*Types::get_float64()]);
						func.insn_call_native1("create_number_value", create_number_value, &* create_number_sig, &[&*val])
					},
					ConstExpr(CString(ref s)) => {
						fn create_string_value(s: *i8) -> Value {
							unsafe {
								let cstr = CString::new(s, false);
								Value{
									ptr: Gc::new(VString(String::from_str(cstr.as_str().unwrap())))
								}
							}
						}
						let create_string_sig = Type::create_signature(CDECL, &*value_t, &[&*cstring_t]);
						let bufptr = wrap_str(s.as_slice());
						func.insn_call_native1("create_string_value", create_string_value, &* create_string_sig, &[&*bufptr])
					},
					GetConstFieldExpr(ref obj, ref field) => {
						fn find_field(obj:Value, s: *i8) -> Value {
							unsafe {
								let cstr = CString::new(s, false);
								obj.get_field_slice(cstr.as_str().unwrap())
							}
						}
						let find_field_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t, &*cstring_t]);
						let obj_i = compile_value(func, *obj);
						let bufptr = wrap_str(field.as_slice());
						func.insn_call_native2("find_field", find_field, &*find_field_sig, &[&*obj_i, &*bufptr])
					},
					GetFieldExpr(ref obj, ref field) => {
						fn find_field(obj:Value, field:Value) -> Value {
							obj.get_field(field.to_str())
						}
						let find_field_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t, &*value_t]);
						let obj_i = compile_value(func, *obj);
						let field_i = compile_value(func, *field);
						func.insn_call_native2("find_field", find_field, &*find_field_sig, &[&*obj_i, &*field_i])
					},
					ObjectDeclExpr(ref fields) => {
						fn create_object_with_fields(mut c_fields: **i8, mut vals: *Value, num_fields: i32) -> Value {
							let object = Value::new_obj(None);
							for _ in range(0, num_fields) {
								unsafe {
									let cstr = CString::new(*c_fields, false);
									let field = cstr.as_str().unwrap();
									object.set_field_slice(field, *vals);
									c_fields = ((c_fields as uint) + size_of::<*i8>()) as **i8;
									vals = ((vals as uint) + size_of::<Value>()) as *Value;
								}
							}
							object
						}
						let value_ptr_t = Type::create_pointer(&*value_t);
						let cstring_ptr_t = Type::create_pointer(&*cstring_t);
						let create_object_sig = Type::create_signature(CDECL, &*value_t, &[&*cstring_ptr_t, &*value_ptr_t, &*Types::get_int()]);
						let num_fields_i = func.constant_int32(fields.len() as i32);
						let fields_i = func.create_value(cstring_ptr_t);
						let fields_size = func.constant_int32((fields.len() as u32 * cstring_t.get_size()) as i32);
						func.insn_store(fields_i, func.insn_alloca(fields_size));
						let values_i = func.create_value(value_ptr_t);
						let values_size = func.constant_int32((fields.len() as u32 * value_t.get_size()) as i32);
						func.insn_store(values_i, func.insn_alloca(values_size));
						let mut i = 0i32;
						for (key, value) in fields.iter() {
							func.insn_store_relative(fields_i, i * cstring_t.get_size() as i32, wrap_str(key.as_slice()));
							func.insn_store_relative(values_i, i * value_t.get_size() as i32, compile_value(func, value));
							i += 1i32;
						}
						func.insn_call_native3("create_object_with_fields", create_object_with_fields, &*create_object_sig, &[&*fields_i, &*values_i, &*num_fields_i])
					},
					ArrayDeclExpr(ref values) => {
						fn create_array(mut vals: *Value, num_vals: i32) -> Value {
							let array = Value::new_obj(None);
							array.set_field_slice("length", to_value(num_vals));
							for i in range(0, num_vals) {
								unsafe {
									array.set_field(i.to_str(), *vals);
									vals = ((vals as uint) + size_of::<Value>()) as *Value;
								}
							}
							array
						}
						let value_ptr_t = Type::create_pointer(&*value_t);
						let create_array_sig = Type::create_signature(CDECL, &*value_t, &[&*value_ptr_t, &*Types::get_int()]);
						let values_i = func.create_value(value_ptr_t);
						let values_size = func.constant_int32((values.len() as u32 * value_t.get_size()) as i32);
						func.insn_store(values_i, func.insn_alloca(values_size));
						let mut i = 0i32;
						for val in values.iter() {
							func.insn_store_relative(values_i, i * value_t.get_size() as i32, compile_value(func, val));
							i += 1i32;
						}
						let num_values = func.constant_int32(values.len() as i32);
						func.insn_call_native2("create_array", create_array, &*create_array_sig, &[&*values_i, &*num_values])
					},
					/*
					FunctionDeclExpr(ref name, ref args, ref expr) => {
						let mut args_i = Vec::with_capacity(args.len());
						let mut arg_types = Vec::with_capacity(args.len());
						for _ in range(0, args.len()) {
							arg_types.push(&*value_t);
						}
						let sig_t = Type::create_signature(CDECL, &*value_t, arg_types.as_slice());
						let new_func = func.get_context().create_function(&*sig_t);
						let value = compile_value(new_func, &**expr);
						new_func.insn_return(&*value);
						let make_func_sig_t = Type::create_signature(CDECL, &*value_t, &[&*sig_t])
					},
					*/
					BlockExpr(ref block) => {
						let last = block.last();
						for expr in block.iter() {
							let comp = compile_value(func, expr);
							if last.unwrap() == expr {
								return comp
							}
						}
						undefined()
					},
					ReturnExpr(None) => {
						func.insn_default_return();
						undefined()
					},
					ReturnExpr(Some(ref ret)) => {
						let i_ret = compile_value(func, *ret);
						func.insn_return(i_ret);
						i_ret
					},
					ThrowExpr(ref val) => {
						let i_val = compile_value(func, *val);
						func.insn_throw(i_val);
						i_val
					},
					TypeOfExpr(ref ex) => {
						fn get_val_type(v:Value) -> Value {
							to_value(v.get_type())
						}
						let get_val_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t]);
						let i_ex = compile_value(func, *ex);
						func.insn_call_native1("get_value_type", get_val_type, &*get_val_sig, &[&*i_ex])
					},
					UnaryOpExpr(op, ref a) => {
						let i_a = compile_value(func, *a);
						let unop_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t]);
						let (name, op_func) = match op {
							UnaryPlus => return i_a,
							UnaryMinus => {
								fn neg_value(a:Value) -> Value {
									-a
								}
								("neg", neg_value)
							}
							UnaryNot => {
								fn not_value(a: Value) -> Value {
									!a
								}
								("not", not_value)
							},
							_ => fail!("Unimplemented {}", op)
						};
						func.insn_call_native1(name, op_func, unop_sig, &[&*i_a])
					},
					BinOpExpr(op, ref a, ref b) => {
						let i_a = compile_value(func, *a);
						let i_b = compile_value(func, *b);
						let binop_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t, &*value_t]);
						
						let (name, op_func) = match op {
							BinNum(OpAdd) => {
								fn add_values(a: Value, b:Value) -> Value {
									a + b
								}
								("add", add_values)
							},
							BinNum(OpSub) => {
								fn sub_values(a: Value, b:Value) -> Value {
									a - b
								}
								("sub", sub_values)
							},
							BinNum(OpMul) => {
								fn mul_values(a: Value, b:Value) -> Value {
									a * b
								}
								("mul", mul_values)
							},
							BinNum(OpDiv) => {
								fn div_values(a: Value, b:Value) -> Value {
									a / b
								}
								("div", div_values)
							},
							BinNum(OpMod) => {
								fn mod_values(a: Value, b:Value) -> Value {
									a % b
								}
								("mod", mod_values)
							},
							BinBit(BitAnd) => {
								fn and_values(a: Value, b:Value) -> Value {
									a & b
								}
								("and", and_values)
							},
							BinBit(BitOr) => {
								fn or_values(a: Value, b:Value) -> Value {
									a | b
								}
								("or", or_values)
							},
							BinBit(BitXor) => {
								fn xor_values(a: Value, b:Value) -> Value {
									a ^ b
								}
								("xor", xor_values)
							},
							BinBit(BitShl) => {
								fn shl_values(a: Value, b:Value) -> Value {
									a << b
								}
								("shl", shl_values)
							},
							BinBit(BitShr) => {
								fn shr_values(a: Value, b:Value) -> Value {
									a >> b
								}
								("shr", shr_values)
							},
							BinLog(LogOr) => {
								fn or_values(a: Value, b:Value) -> Value {
									to_value(a.is_true() || b.is_true())
								}
								("or", or_values)
							},
							BinLog(LogAnd) => {
								fn and_values(a: Value, b:Value) -> Value {
									to_value(a.is_true() && b.is_true())
								}
								("and", and_values)
							},
							BinComp(CompEqual) | BinComp(CompStrictEqual) => {
								fn eq_values(a: Value, b:Value) -> Value {
									to_value(a == b)
								}
								("eq", eq_values)
							},
							BinComp(CompNotEqual) | BinComp(CompStrictNotEqual) => {
								fn neq_values(a: Value, b:Value) -> Value {
									to_value(a != b)
								}
								("neq", neq_values)
							},
							BinComp(CompLessThan) => {
								fn lt_values(a: Value, b:Value) -> Value {
									to_value(a < b)
								}
								("lt", lt_values)
							},
							BinComp(CompLessThanOrEqual) => {
								fn lte_values(a: Value, b:Value) -> Value {
									to_value(a <= b)
								}
								("lte", lte_values)
							},
							BinComp(CompGreaterThan) => {
								fn gt_values(a: Value, b:Value) -> Value {
									to_value(a > b)
								}
								("gt", gt_values)
							},
							BinComp(CompGreaterThanOrEqual) => {
								fn gte_values(a: Value, b:Value) -> Value {
									to_value(a >= b)
								}
								("gte", gte_values)
							}
						};
						func.insn_call_native2(name, op_func, binop_sig, &[&*i_a, &*i_b])
					},
					IfExpr(ref cond, ref expr, None) => {
						fn from_bool_value(v:Value) -> bool {
							v.is_true()
						}
						let from_bool_sig = Type::create_signature(CDECL, &*Types::get_bool(), &[&*value_t]);
						let i_cond = compile_value(func, *cond);
						let i_cond_bool = func.insn_call_native1("to_bool", from_bool_value, from_bool_sig, &[&*i_cond]);
						let value = func.insn_call_native0("create_undef_value", create_undef_value, &*create_value_sig, &[]);
						let mut done_label = jit::Label::new();
						func.insn_branch_if_not(i_cond_bool, done_label);
						func.insn_store(value, compile_value(func, *expr));
						func.insn_set_label(done_label);
						value
					},
					_ => fail!("Unimplemented {}", expr)
				}
			}
			let valuedata_t = Types::get_int();
			let valuedata_ptr_t = Type::create_pointer(&*valuedata_t);
			let value_t = Type::create_struct(&[&*valuedata_ptr_t]);
			let default_sig_t = Type::create_signature(CDECL, &*value_t, &[&*value_t, &*value_t, &*value_t]);
			let func = self.context.create_function(&*default_sig_t);
			let value = compile_value(func, expr);
			func.insn_return(&*value);
			func.dump("js.rs");
			func.set_optimization_level(5);
			func.set_recompilable();
			func.compile();
			func
		})
	}
	fn run(&mut self, comp:Box<Function>) -> ResultValue {
		// global fn(global, scope, this)
		let func: fn(Value, Value, Value) -> Value = comp.closure();
		return Ok(func(self.global, self.global, self.global));
	}
}