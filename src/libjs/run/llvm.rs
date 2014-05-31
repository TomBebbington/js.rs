use syntax::ast::expr::*;
use syntax::ast::constant::*;
use syntax::ast::op::*;
use stdlib::value::*;
use stdlib::{object, console, math, array, function, json, number, error, string, uri};
use run::exec::Executor;
use std::gc::Gc;
use std::c_str::CString;
use std::mem::size_of;
use llvm::{Context, Compilable, Builder, Module, Type, Types, TypeKind, ExecutionEngine};
use llvm::{Integer};
use JValue = llvm::Value;

/// A Javascript JIT compiler
pub struct LLVMCompiler {
	/// The JIT Context
	context: Box<Context>,
	/// The JIT Module
	module: Box<Module>,
	/// An object representing the global object
	pub global: Value
}
impl LLVMCompiler {
	fn with_builder<R>(&self, cb: |b:&Builder| -> R) -> R {
		let builder = Builder::new(&*self.context);
		let rv = cb(builder);
		rv
	}
	fn compile_value(&self, bd: &Builder, expr: &Expr) -> Box<JValue> {
		println!("{}", expr);
		match expr.def {
			ConstExpr(CString(ref text)) => text.compile(self.context),
			ConstExpr(CNum(num)) => num.compile(self.context),
			ConstExpr(CInt(num)) => num.compile(self.context),
			ConstExpr(CBool(val)) => val.compile(self.context),
			BlockExpr(ref exprs) => self.compile(exprs.get(0)),
			BinOpExpr(op, ref a, ref b) => {
				let (va, vb) = (self.compile_value(bd, *a), self.compile_value(bd, *b));
				let (vat, vbt) = (va.get_type(), vb.get_type());
				match op {
					BinNum(OpAdd) => 
						match (vat.get_kind(), vbt.get_kind()) {
							(Integer, Integer) => bd.insn_add(va, vb),
							_ => bd.insn_fadd(va, vb),
						},
					BinNum(OpSub) => 
						match (vat.get_kind(), vbt.get_kind()) {
							(Integer, Integer) => bd.insn_sub(va, vb),
							_ => bd.insn_fsub(va, vb),
						},
					BinNum(OpMul) => 
						match (vat.get_kind(), vbt.get_kind()) {
							(Integer, Integer) => bd.insn_mul(va, vb),
							_ => bd.insn_fmul(va, vb),
						},
					BinNum(OpDiv) => 
						match (vat.get_kind(), vbt.get_kind()) {
							(Integer, Integer) => bd.insn_sdiv(va, vb),
							_ => bd.insn_fdiv(va, vb),
						},
					BinNum(OpMod) => 
						match (vat.get_kind(), vbt.get_kind()) {
							(Integer, Integer) => bd.insn_srem(va, vb),
							_ => bd.insn_frem(va, vb),
						},
					BinLog(LogAnd) =>
						bd.insn_and(va, vb),
					BinLog(LogOr) =>
						bd.insn_or(va, vb),
					_ => fail!("Undefined {}", expr)
				}
			},
			_ => fail!("Undefined {}", expr)
		}
	}
}
impl Executor<JValue> for LLVMCompiler {
	fn new() -> LLVMCompiler {
		let global = to_value(());
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
		let context = Context::new();
		let module = Module::new("js.rs", &*context);
		LLVMCompiler {
			context: context,
			module: module,
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
	fn compile(&self, expr: &Expr) -> Box<JValue> {
		let value = self.with_builder(|bd| {
			self.compile_value(bd, expr)
		});
		let sig = Type::create_signature(value.get_type(), &[], false);
		self.module.add_function("main", sig)
	}
	fn run(&mut self, comp:Box<JValue>) -> ResultValue {
		let engine = ExecutionEngine::new(self.module);
		return Ok(to_value(comp.to_str()))
	}
}