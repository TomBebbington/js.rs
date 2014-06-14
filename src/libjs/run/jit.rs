use syntax::ast::expr::*;
use syntax::ast::constant::*;
use syntax::ast::op::*;
use syntax::ast::types::*;
use JSType = syntax::ast::types::Type;
use stdlib::value::{Value, VNull, ResultValue, to_value};
use stdlib::*;
use run::exec::Executor;
use std::gc::Gc;
use std::c_str::CString;
use jit::{get_type, Context, Function, Type, Compilable};
use jit::{UByte, SysChar, SysBool, Int, UInt, Pointer, Float64};
use jit::{init, supports_threads, supports_virtual_memory, uses_interpreter};
use JitValue = jit::Value;

fn compile_type(js_type:&JSType) -> Type {
	match *js_type {
		UndefinedType | NullType | NativeObjectType | ObjectType | AnyType => get_type::<*int>(),
		FunctionType => get_type::<fn(*int) -> *int>(),
		StringType => get_type::<String>(),
		BooleanType => get_type::<bool>(),
		NumberType => get_type::<f64>(),
		IntegerType => get_type::<i32>(),
		AnyOfType(ref types) => {
			fail!("Unknown types: {}", types)
		}
	}
}
/// A Javascript JIT compiler
pub struct JITCompiler {
	/// The JIT Context
	context: Context,
	/// An object representing the global object
	pub global: Value
}
impl Executor<Function> for JITCompiler {
	fn new() -> JITCompiler {
		debug!("Initialising LibJIT...");
		init();
		debug!("JIT supports threads? {}", supports_threads());
		debug!("JIT supports virtual memory? {}", supports_virtual_memory());
		debug!("JIT using interpreter? {}", uses_interpreter());
		debug!("Initialising global object...");
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
			global: global
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
	fn compile(&self, expr: &Expr) -> Function {
		debug!("Compiling {} in builder", expr);
		self.context.build(|| {
			let default_sig_t = get_type::<fn(*int, *int, *int) -> *int>();
			let func = Function::new(&self.context, &default_sig_t);
			let value = compile_value(&func, expr);
			func.insn_return(&convert_to_value(&func, &value));
			func.set_optimization_level(5);
			func.set_recompilable();
			func.compile();
			func
		})
	}
	fn run(&mut self, comp:&Function) -> ResultValue {
		debug!("Running function");
		debug!("There are {} functions in context", self.context.iter_funcs().size_hint());
		let func: fn(Value, Value, Value) -> Value = unsafe { comp.closure3() };
		return Ok(func(self.global, self.global, self.global));
	}
}

fn convert_to_value(func:&Function, val:&JitValue) -> JitValue {
	let value_t = get_type::<*int>();
	let undef_value = Value::undefined;
	let val_type = val.get_type();
	let val_kind = val_type.get_kind();
	debug!("Converting JIT value of kind {} to Javascript value", val_kind.bits());
	if val_kind.contains(SysBool) || val_kind.contains(UByte) {
		let bool_value = to_value::<bool>;
		let sig = get_type::<fn(bool) -> *int>();
		func.insn_call_native1("bool_value", bool_value, &sig, &mut [val])
	} else if val_kind.contains(Pointer) {
		let ref_t = val_type.get_ref();
		if ref_t.get_kind().contains(SysChar) {
			fn string_value(val: *i8) -> Value {
				unsafe {
					let text = CString::new(val, false);
					to_value(text.as_str().unwrap().into_string())
				}
			}
			let sig = get_type::<fn(String) -> *int>();
			func.insn_call_native1("string_value", string_value, &sig, &mut [val])
		} else {
			fn ptr_value(ptr: *i8) -> Value {
				match ptr.to_uint() {
					0u => Value::undefined(),
					1u => Value {
						ptr: Gc::new(VNull)
					},
					ptr => fail!("Invalid pointer: {}", ptr)
				}
			}
			let sig = get_type::<fn(*i8) -> *int>();
			func.insn_call_native1("ptr_value", ptr_value, &sig, &mut [val])
		}
	} else if val_kind.contains(Int) || val_kind.contains(UInt) {
		let int_value = to_value::<i32>;
		let sig = get_type::<fn(i32) -> *int>();
		func.insn_call_native1("int_value", int_value, &sig, &mut [val])
	} else if val_kind.contains(Float64) {
		let float_value = to_value::<f64>;
		let sig = get_type::<fn(f64) -> *int>();
		func.insn_call_native1("float_value", float_value, &sig, &mut [val])
	} else {
		fail!("Invalid kind {}", val_kind.bits())
	}
}

fn compile_value(func:&Function, expr: &Expr) -> JitValue {
	let value_t = get_type::<*int>();
	let cstring_t = get_type::<*char>();
	let create_value_sig = get_type::<fn() -> *int>();
	let undefined = || {
		let ptr = JitValue::new(func, &get_type::<*int>());
		let val = 0u8.compile(func);
		func.insn_store(&ptr, &val);
		ptr
	};
	let global = func.get_param(0);
	let scope = func.get_param(1);
	let this = func.get_param(2);
	debug!("Compiling {} into a LibJIT value", expr);
	match expr.def {
		ConstExpr(CNull) => {
			let ptr = JitValue::new(func, &get_type::<*int>());
			let val = 1u8.compile(func);
			func.insn_store(&ptr, &val);
			ptr
		},
		ConstExpr(CUndefined) => {
			undefined()
		},
		ConstExpr(CBool(v)) => {
			v.compile(func)
		},
		ConstExpr(CNum(n)) => {
			n.compile(func)
		},
		ConstExpr(CString(ref s)) => {
			s.compile(func)
		},
		BinOpExpr(BinNum(OpAdd), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_add(&i_a, &i_b)
		},
		BinOpExpr(BinNum(OpSub), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_sub(&i_a, &i_b)
		},
		BinOpExpr(BinNum(OpMul), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_mul(&i_a, &i_b)
		},
		BinOpExpr(BinNum(OpDiv), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_div(&i_a, &i_b)
		},
		BinOpExpr(BinNum(OpMod), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_rem(&i_a, &i_b)
		},
		BinOpExpr(BinBit(BitAnd), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_and(&i_a, &i_b), &get_type::<i32>(), false)
		},
		BinOpExpr(BinBit(BitOr), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_or(&i_a, &i_b), &get_type::<i32>(), false)
		},
		BinOpExpr(BinBit(BitXor), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_xor(&i_a, &i_b), &get_type::<i32>(), false)
		},
		BinOpExpr(BinBit(BitShl), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_shl(&i_a, &i_b), &get_type::<i32>(), false)
		},
		BinOpExpr(BinBit(BitShr), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_shr(&i_a, &i_b), &get_type::<i32>(), false)
		},
		BinOpExpr(BinComp(CompEqual), ref a, ref b) | BinOpExpr(BinComp(CompStrictEqual), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_eq(&i_a, &i_b), &get_type::<bool>(), false)
		},
		BinOpExpr(BinComp(CompNotEqual), ref a, ref b) | BinOpExpr(BinComp(CompStrictNotEqual), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_neq(&i_a, &i_b), &get_type::<bool>(), false)
		},
		BinOpExpr(BinComp(CompLessThan), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_lt(&i_a, &i_b), &get_type::<bool>(), false)
		},
		BinOpExpr(BinComp(CompLessThanOrEqual), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_leq(&i_a, &i_b), &get_type::<bool>(), false)
		},
		BinOpExpr(BinComp(CompGreaterThan), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_gt(&i_a, &i_b), &get_type::<bool>(), false)
		},
		BinOpExpr(BinComp(CompGreaterThanOrEqual), ref a, ref b) => {
			let i_a = compile_value(func, *a);
			let i_b = compile_value(func, *b);
			func.insn_convert(&func.insn_geq(&i_a, &i_b), &get_type::<bool>(), false)
		},
		UnaryOpExpr(UnaryMinus, ref a) => {
			func.insn_neg(&compile_value(func, *a))
		},
		UnaryOpExpr(UnaryNot, ref a) => {
			let mut val = compile_value(func, *a);
			val = func.insn_convert(&val, &get_type::<bool>(), false);
			val = func.insn_neg(&val);
			func.insn_convert(&val, &get_type::<bool>(), false)
		},
		UnaryOpExpr(UnaryPlus, ref a) => {
			compile_value(func, *a)
		},
		BlockExpr(ref exprs) => {
			let mut result = undefined();
			let last = exprs.last();
			for expr in exprs.iter() {
				let res = compile_value(func, expr);
				if expr == last.unwrap() {
					result = res;
				}
			}
			result
		},
		ReturnExpr(Some(ref ex)) => {
			let ret = compile_value(func, *ex);
			func.insn_return(&convert_to_value(func, &ret));
			ret
		}
		_ => fail!("Unimplemented {}", expr)
	}
}