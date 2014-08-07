use collections::TreeMap;
use syntax::ast::expr::*;
use syntax::ast::op::*;
use syntax::ast::constant::Const;
/**
 * A compiler that transforms expressions into their compiled
 * form, typically through a library such as LibJIT or LLVM.
*/
pub trait Compiler<'a, Compiled> {
    /// Compile an expression
    fn compile(&'a self, expr:&Expr) -> Compiled {
        debug!("Compiling {}", expr);
        match expr.def.clone() {
            UnaryOpExpr(op, box ref ex) =>
                self.compile_unary_op(op, ex),
            BinOpExpr(op, box ref left, box ref right) =>
                self.compile_bin_op(op, left, right),
            ConstExpr(ref c) =>
                self.compile_const(c),
            LocalExpr(l) =>
                self.compile_local(l),
            BlockExpr(vals) =>
                self.compile_block(vals),
            GetConstFieldExpr(box ref obj, field) =>
                self.compile_get_const_field(obj, field),
            GetFieldExpr(box ref obj, box ref field) =>
                self.compile_get_field(obj, field),
            CallExpr(box ref func, args) =>
                self.compile_call(func, args),
            WhileLoopExpr(box ref cond, box ref expr) =>
                self.compile_while_loop(cond, expr),
            IfExpr(box ref cond, box ref if_expr, else_expr) =>
                self.compile_if(cond, if_expr, else_expr),
            SwitchExpr(box ref value, cases, default) =>
                self.compile_switch(value, cases, default),
            ObjectDeclExpr(box ref fields) =>
                self.compile_object_decl(fields),
            ArrayDeclExpr(values) =>
                self.compile_array_decl(values),
            FunctionDeclExpr(name, args, box ref ret) =>
                self.compile_function_decl(name, args, ret),
            ArrowFunctionDeclExpr(args, box ref ret) =>
                self.compile_arrow_function_decl(args, ret),
            ConstructExpr(box ref func, args) =>
                self.compile_construct(func, args),
            ReturnExpr(val) =>
                self.compile_return(val),
            ThrowExpr(box ref val) =>
                self.compile_throw(val),
            AssignExpr(box ref left, box ref right) =>
                self.compile_assign(left, right),
            VarDeclExpr(vars) =>
                self.compile_var_decl(vars),
            TypeOfExpr(box ref expr) =>
                self.compile_typeof(expr)
        }
    }
    /// Compile a unary operation
    fn compile_unary_op(&'a self, _:UnaryOp, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a binary operation
    fn compile_bin_op(&'a self, op:BinOp, left:&Expr, right:&Expr) -> Compiled {
        match op {
            BinNum(num) => self.compile_num_op(num, left, right),
            BinBit(bit) => self.compile_bit_op(bit, left, right),
            BinComp(comp) => self.compile_comp_op(comp, left, right),
            BinLog(log) => self.compile_log_op(log, left, right),
        }
    }
    /// Compile a numeric operation
    fn compile_num_op(&'a self, _:NumOp, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a bitwise operation
    fn compile_bit_op(&'a self, _:BitOp, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a comparitive operation
    fn compile_comp_op(&'a self, _:CompOp, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a logical operation
    fn compile_log_op(&'a self, _:LogOp, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a constant
    fn compile_const(&'a self, _:&Const) -> Compiled {
        unimplemented!()
    }
    /// Compile a local variable
    fn compile_local(&'a self, _:String) -> Compiled {
        unimplemented!()
    }
    /// Compile a block of expressions
    fn compile_block(&'a self, _:Vec<Expr>) -> Compiled {
        unimplemented!()
    }
    /// Compile constant field access for an object
    fn compile_get_const_field(&'a self, _:&Expr, _:String) -> Compiled {
        unimplemented!()
    }
    /// Compile field access for an object
    fn compile_get_field(&'a self, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a call to a function with some arguments
    fn compile_call(&'a self, _:&Expr, _:Vec<Expr>) -> Compiled {
        unimplemented!()
    }
    /// Compile a while loop
    fn compile_while_loop(&'a self, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile an if statement
    fn compile_if(&'a self, _:&Expr, _:&Expr, _:Option<Box<Expr>>) -> Compiled {
        unimplemented!()
    }
    /// Compile a switch statement
    fn compile_switch(&'a self, _:&Expr, Vec<(Expr, Vec<Expr>)>, Option<Box<Expr>>) -> Compiled {
        unimplemented!()
    }
    /// Compile an object declaration
    fn compile_object_decl(&'a self, &TreeMap<String, Expr>) -> Compiled {
        unimplemented!()
    }
    /// Compile an array declaration
    fn compile_array_decl(&'a self, Vec<Expr>) -> Compiled {
        unimplemented!()
    }
    /// Compile a function declaration
    fn compile_function_decl(&'a self, _:Option<String>, _:Vec<String>, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile an arrow function declaration
    fn compile_arrow_function_decl(&'a self, _:Vec<String>, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a construction of an object
    fn compile_construct(&'a self, _:&Expr, _:Vec<Expr>) -> Compiled {
        unimplemented!()
    }
    /// Compile a return expression
    fn compile_return(&'a self, _:Option<Box<Expr>>) -> Compiled {
        unimplemented!()
    }
    /// Compile a throw expression
    fn compile_throw(&'a self, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile an assignment
    fn compile_assign(&'a self, _:&Expr, _:&Expr) -> Compiled {
        unimplemented!()
    }
    /// Compile a variable declaration
    fn compile_var_decl(&'a self, _:Vec<(String, Option<Expr>)>) -> Compiled {
        unimplemented!()
    }
    /// Compile a typeof expression
    fn compile_typeof(&'a self, _:&Expr) -> Compiled {
        unimplemented!()
    }
}