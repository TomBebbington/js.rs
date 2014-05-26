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
use collections::treemap::TreeMap;
use std::vec::Vec;
use std::gc::Gc;
use std::cell::RefCell;
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
	fn set_global(&mut self, name:StrBuf, val:Value) -> Value;
	/// Resolve the global variable `name`
	fn get_global(&self, name:StrBuf) -> Value;
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
		let global = ValueData::new_obj(None);
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
	fn set_global(&mut self, name:StrBuf, val:Value) -> Value {
		self.global.borrow().set_field(name, val)
	}
	#[inline(always)]
	fn get_global(&self, name:StrBuf) -> Value {
		self.global.borrow().get_field(name)
	}
	fn make_scope(&mut self, this:Value) -> Scope {
		let scope = Scope {
			this: this,
			vars: ValueData::new_obj(None)
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
			fn create_value_from_data(arg: ValueData) -> Value {
				Gc::new(arg)
			}
			let valuedata_t = Types::get_int();
			let valuedata_ptr_t = Type::create_pointer(&*valuedata_t);
			let value_t = Type::create_struct(&[&*valuedata_ptr_t]);
			let create_val_sig = Type::create_signature(CDECL, &*value_t, &[&*valuedata_t]);
			let default_sig_t = Type::create_signature(CDECL, Types::get_void(), &[&*valuedata_ptr_t]);
			fn compile_value(func:&Function, expr: &Expr) -> Box<jit::Value> {
				let valuedata_t = Types::get_int();
				let valuedata_ptr_t = Type::create_pointer(&*valuedata_t);
				let value_t = Type::create_struct(&[&*valuedata_ptr_t]);
				let value_ptr_t = Type::create_pointer(&*value_t);
				let create_val_sig = Type::create_signature(CDECL, &*value_t, &[&*valuedata_t]);
				match expr.def {
					ConstExpr(CNull) => {
						let vnull = func.insn_alloca(func.constant_int32(valuedata_t.get_size() as i32));
						func.insn_store_relative(&*vnull, 0, func.constant_int32(0));
						let t = func.create_value(&*valuedata_t);
						func.insn_store(t, vnull);
						func.insn_call_native1("create_value_from_data", create_value_from_data, &*create_val_sig, &[&*t])
					},
					ConstExpr(CUndefined) => {
						let vundef = func.insn_alloca(func.constant_int32(valuedata_t.get_size() as i32));
						func.insn_store_relative(&*vundef, 0, func.constant_int32(1));
						let t = func.create_value(&*valuedata_t);
						func.insn_store(t, vundef);
						func.insn_call_native1("create_value_from_data", create_value_from_data, &*create_val_sig, &[&*t])
					},
					ConstExpr(CBool(v)) => {
						let create_bool_value = to_value::<bool>;
						let val = func.constant_int32_as_type(v as i32, *Types::get_bool());
						let create_bool_sig = Type::create_signature(CDECL, &*value_t, &[&*Types::get_bool()]);
						func.insn_call_native1("create_bool_value", create_bool_value, &*create_bool_sig, &[&*val])
					},
					ConstExpr(CNum(n)) => {
						let create_number_value = to_value::<f64>;
						let val = func.constant_float64(n);
						let create_number_sig = Type::create_signature(CDECL, &*value_t, &[&*Types::get_float64()]);
						func.insn_call_native1("create_number_value", create_number_value, &* create_number_sig, &[&*val])
					},
					BinOpExpr(op, ref a, ref b) => {
						let i_a = compile_value(func, *a);
						let i_b = compile_value(func, *b);
						let binop_sig = Type::create_signature(CDECL, &*value_t, &[&*value_t, &*value_t]);
						
						let (name, op_func) = match op {
							BinNum(OpAdd) => {
								fn add_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() + *b.borrow())
								}
								("add", add_values)
							},
							BinNum(OpSub) => {
								fn sub_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() - *b.borrow())
								}
								("sub", sub_values)
							},
							BinNum(OpMul) => {
								fn mul_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() * *b.borrow())
								}
								("mul", mul_values)
							},
							BinNum(OpDiv) => {
								fn div_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() / *b.borrow())
								}
								("div", div_values)
							},
							BinNum(OpMod) => {
								fn mod_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() % *b.borrow())
								}
								("mod", mod_values)
							},
							BinBit(BitAnd) => {
								fn and_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() & *b.borrow())
								}
								("and", and_values)
							},
							BinBit(BitOr) => {
								fn or_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() | *b.borrow())
								}
								("or", or_values)
							},
							BinBit(BitXor) => {
								fn xor_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() ^ *b.borrow())
								}
								("xor", xor_values)
							},
							BinBit(BitShl) => {
								fn shl_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() << *b.borrow())
								}
								("shl", shl_values)
							},
							BinBit(BitShr) => {
								fn shr_values(a: Value, b:Value) -> Value {
									Gc::new(a.borrow() >> *b.borrow())
								}
								("shr", shr_values)
							},
							BinLog(LogOr) => {
								fn or_values(a: Value, b:Value) -> Value {
									to_value(a.borrow().is_true() || b.borrow().is_true())
								}
								("or", or_values)
							},
							BinLog(LogAnd) => {
								fn and_values(a: Value, b:Value) -> Value {
									to_value(a.borrow().is_true() && b.borrow().is_true())
								}
								("and", and_values)
							},
							BinComp(CompEqual) | BinComp(CompStrictEqual) => {
								fn eq_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() == b.borrow())
								}
								("eq", eq_values)
							},
							BinComp(CompNotEqual) | BinComp(CompStrictNotEqual) => {
								fn neq_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() != b.borrow())
								}
								("neq", neq_values)
							},
							BinComp(CompLessThan) => {
								fn lt_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() < b.borrow())
								}
								("lt", lt_values)
							},
							BinComp(CompLessThanOrEqual) => {
								fn lte_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() <= b.borrow())
								}
								("lte", lte_values)
							},
							BinComp(CompGreaterThan) => {
								fn gt_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() > b.borrow())
								}
								("gt", gt_values)
							},
							BinComp(CompGreaterThanOrEqual) => {
								fn gte_values(a: Value, b:Value) -> Value {
									to_value(a.borrow() >= b.borrow())
								}
								("gte", gte_values)
							}
						};
						func.insn_call_native2(name, op_func, binop_sig, &[&*i_a, &*i_b])
					},
					_ => fail!("Unimplemented {}", expr)
				}
			}
			let func = self.context.create_function(&*default_sig_t);
			let result = func.get_param(0);
			let value = compile_value(func, expr);
			func.insn_store_relative(&result, 0, &*value);
			func.compile();
			func
		})
	}
	fn run(&mut self, comp:Box<Function>) -> ResultValue {
		let func: fn(&mut Value) = comp.closure();
		let mut rv: Value = Gc::new(VUndefined);
		func(&mut rv);
		return Ok(rv);
	}
}