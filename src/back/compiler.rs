use front::run::compiler::Compiler;
use syntax::ast::constant::*;
use syntax::ast::op::*;
use syntax::ast::expr::Expr;
use jit::{
    Context,
    Compile,
    Function,
    Value,
    get_type,
    SysBool,
    Int,
    UInt,
    NInt,
    NUInt,
    Float64,
    Pointer
};
type CompiledValue<'a> = (Value<'a>, &'a Function<'a>);
/// A compiler using the LibJIT backend
pub struct JitCompiler<'a> {
    curr: Function<'a>
}
impl<'a> JitCompiler<'a> {
    /// Construct a new JIT Compiler on the given context
    pub fn new(context: &'a Context) -> JitCompiler<'a> {
        let main_t = get_type::<fn(*int, *int, *int) -> *int>();
        JitCompiler {
            curr: Function::new(context, &main_t)
        }
    }
    fn convert_bool(&'a self, val:Value<'a>) -> Value<'a> {
        let bool_t = get_type::<bool>();
        let val_kind = val.get_type().get_kind();
        let convert = |v| self.curr.insn_convert(&v, &bool_t, false);
        if val_kind.contains(SysBool) {
            val
        } else if val_kind.contains(Float64) {
            let zero = 0.0f64.compile(&self.curr);
            let not_zero = self.curr.insn_neq(&val, &zero);
            let not_nan = !self.curr.insn_is_nan(&val);
            convert(not_zero & not_nan)
        } else if val_kind.contains(Int) || val_kind.contains(UInt) || val_kind.contains(NInt) || val_kind.contains(NUInt) {
            let zero = 0i.compile(&self.curr); 
            convert(self.curr.insn_neq(&val, &zero))
        } else if val_kind.contains(Pointer) {
            let one = 1i.compile(&self.curr);
            convert(self.curr.insn_gt(&val, &one))
        } else {
            convert(val)
        }
    }
    fn undefined(&'a self) -> Value<'a> {
        let ptr = Value::new(&self.curr, &get_type::<*int>());
        let val = 0u8.compile(&self.curr);
        self.curr.insn_store(&ptr, &val);
        ptr
    }
}
impl<'a> Compiler<'a, (Value<'a>, &'a Function<'a>)> for JitCompiler<'a> {
    fn compile_const(&'a self, constant:&Const) -> CompiledValue<'a> {
        (match constant.clone() {
            CString(v) =>
                v.compile(&self.curr),
            CNum(v) =>
                v.compile(&self.curr),
            CInt(v) =>
                v.compile(&self.curr),
            CBool(v) =>
                v.compile(&self.curr),
            CNull => {
                let ptr = Value::new(&self.curr, &get_type::<*int>());
                let val = 1u8.compile(&self.curr);
                self.curr.insn_store(&ptr, &val);
                ptr
            },
            CUndefined => {
                self.undefined()
            },
            _ => unimplemented!()
        }, &self.curr)
    }
    fn compile_block(&'a self, block:Vec<Expr>) -> CompiledValue<'a> {
        let last = block.last();
        for expr in block.iter() {
            let comp = self.compile(expr);
            if expr == last.unwrap() {
                return comp
            }
        }
        unreachable!()
    }
    fn compile_num_op(&'a self, op:NumOp, left:&Expr, right:&Expr) -> CompiledValue<'a> {
        let (c_left, _) = self.compile(left);
        let (c_right, _) = self.compile(right);
        (match op {
            OpAdd => c_left + c_right,
            OpSub => c_left - c_right,
            OpDiv => c_left / c_right,
            OpMul => c_left * c_right,
            OpMod => c_left % c_right
        }, &self.curr)
    }
    fn compile_bit_op(&'a self, op:BitOp, left:&Expr, right:&Expr) -> CompiledValue<'a> {
        let int_t = get_type::<i32>();
        let (c_left, _) = self.compile(left);
        let c_left = self.curr.insn_convert(&c_left, &int_t, false);
        let (c_right, _) = self.compile(right);
        let c_right = self.curr.insn_convert(&c_right, &int_t, false);
        (match op {
            BitAnd => c_left & c_right,
            BitOr => c_left | c_right,
            BitXor => c_left ^ c_right,
            BitShl => c_left << c_right,
            BitShr => c_left >> c_right
        }, &self.curr)
    }
    fn compile_log_op(&'a self, op:LogOp, left:&Expr, right:&Expr) -> CompiledValue<'a> {
        let (c_left, _) = self.compile(left);
        let c_left = self.convert_bool(c_left);
        let (c_right, _) = self.compile(right);
        let c_right = self.convert_bool(c_right);
        (match op {
            LogAnd => c_left & c_right,
            LogOr => c_left | c_right
        }, &self.curr)
    }
    fn compile_comp_op(&'a self, op:CompOp, left:&Expr, right:&Expr) -> CompiledValue<'a> {
        let (c_left, _) = self.compile(left);
        let (c_right, _) = self.compile(right);
        let val = match op {
            CompEqual | CompStrictEqual =>
                self.curr.insn_eq(&c_left, &c_right),
            CompNotEqual | CompStrictNotEqual =>
                self.curr.insn_neq(&c_left, &c_right),
            CompGreaterThan =>
                self.curr.insn_gt(&c_left, &c_right),
            CompGreaterThanOrEqual =>
                self.curr.insn_geq(&c_left, &c_right),
            CompLessThan =>
                self.curr.insn_lt(&c_left, &c_right),
            CompLessThanOrEqual =>
                self.curr.insn_leq(&c_left, &c_right),
        };
        let bool_val = self.curr.insn_convert(&val, &get_type::<bool>(), false);
        (bool_val, &self.curr)
    }
    fn compile_unary_op(&'a self, op:UnaryOp, val:&Expr) -> CompiledValue<'a> {
        let (c_val, _) = self.compile(val);
        (match op {
            UnaryMinus => -c_val,
            UnaryPlus => c_val,
            UnaryNot => {
                let c_not = !c_val;
                self.curr.insn_convert(&c_not, &get_type::<bool>(), false)
            },
            _ => unimplemented!()
        }, &self.curr)
    }
    fn compile_return(&'a self, val:Option<Box<Expr>>) -> CompiledValue<'a> {
        match val {
            Some(val) => {
                let (c_val, _) = self.compile(val);
                self.curr.insn_return(&c_val)
            },
            None => {
                self.curr.insn_default_return()
            }
        };
        (self.undefined(), &self.curr)
    }
}