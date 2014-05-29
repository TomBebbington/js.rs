use stdlib::value::{Value, ResultValue, VObject};
use syntax::ast::expr::Expr;
use run::jit::JITCompiler;
/// An execution engine
pub trait Executor<T> {
	/// Make a new execution engine
	fn new() -> Self;
	/// Set a global variable called `name` with the value `val`
	fn set_global(&mut self, name:String, val:Value) -> Value;
	/// Resolve the global variable `name`
	fn get_global(&self, name:String) -> Value;
	/// Compile the expression
	fn compile(&self, expr:&Expr) -> Box<T>;
	/// Run the compiled expression
	fn run(&mut self, comp:Box<T>) -> ResultValue;
}
/// Execute an expression
pub fn execute(expr:&Expr) -> ResultValue {
	let mut exec:JITCompiler = Executor::new();
	let compiled = exec.compile(expr);
	exec.run(compiled)
}
/// Execute an expression with an environment
pub fn execute_env(expr:&Expr, env:Value) -> ResultValue {
	let mut exec:JITCompiler = Executor::new();
	exec.global.set_field_slice("__proto__", env);
	let compiled = exec.compile(expr);
	exec.run(compiled)
}