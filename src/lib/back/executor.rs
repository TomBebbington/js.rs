use JITVal = jit::Value;
use jit::{
    get_type,
    Function,
    Compile,
    UByte,
    SysChar,
    SysBool,
    NInt,
    NUInt,
    Int,
    UInt,
    Pointer,
    Float64
};
use JSVal = front::stdlib::value::Value;
use front::stdlib::value::{VNull, to_value};
use front::stdlib::value::ResultValue;
use front::run::executor::{Executor, ExecutorConfig};
use std::gc::GC;
use std::c_str::CString;
/// A JIT executor
pub struct JitExecutor {
    global: JSVal
}
impl<'a> Executor<(JITVal<'a>, &'a Function<'a>)> for JitExecutor {
    #[inline(always)]
    fn new(config:&ExecutorConfig) -> JitExecutor {
        JitExecutor {
            global: config.global.clone()
        }
    }
    #[inline]
    fn get_global_obj(&self) -> JSVal {
        self.global
    }
    fn execute(&self, comp:&(JITVal<'a>, &'a Function<'a>)) -> ResultValue {
        let &(ref val, ref func) = comp;
        func.insn_return(&convert_to_value(*func, val));
        func.set_optimization_level(5);
        func.set_recompilable();
        func.compile();
        Ok(func.with_closure3(|run:fn(JSVal, JSVal, JSVal) -> JSVal| {
            run(self.global, self.global, self.global)
        }))
    }
}

fn convert_to_value<'a>(func:&Function<'a>, val:&'a JITVal<'a>) -> JITVal<'a> {
    let val_type = val.get_type();
    let val_kind = val_type.get_kind();
    if val_kind.contains(SysBool) || val_kind.contains(UByte) {
        let bool_value = to_value::<bool>;
        let sig = get_type::<fn(bool) -> *const int>();
        func.insn_call_native1(Some("bool_value"), bool_value, sig, &mut [val])
    } else if val_kind.contains(Pointer) {
        let ref_t = val_type.get_ref();
        if ref_t.get_kind().contains(SysChar) {
            fn string_value(val: &i8) -> JSVal {
                unsafe {
                    let text = CString::new(val, false);
                    to_value(text.as_str().unwrap().into_string())
                }
            }
            let sig = get_type::<fn(String) -> *const int>();
            func.insn_call_native1(Some("string_value"), string_value, sig, &mut [val])
        } else {
            fn ptr_value(ptr: &i8) -> JSVal {
                match ptr.to_uint() {
                    Some(0u) => JSVal::undefined(),
                    Some(1u) => JSVal {
                        ptr: box(GC) VNull
                    },
                    ptr => fail!("Invalid pointer: {}", ptr)
                }
            }
            let sig = get_type::<fn(*const i8) -> *const int>();
            func.insn_call_native1(Some("ptr_value"), ptr_value, sig, &mut [val])
        }
    } else if val_kind.contains(Int) || val_kind.contains(UInt) {
        let int_value = to_value::<i32>;
        let sig = get_type::<fn(i32) -> *const int>();
        func.insn_call_native1(Some("int_value"), int_value, sig, &mut [val])
    } else if val_kind.contains(NInt) || val_kind.contains(NUInt) {
        fn sys_int_value(num:int) -> JSVal {
            to_value::<i32>(num as i32)
        }
        let sig = get_type::<fn(int) -> *const int>();
        func.insn_call_native1(Some("sys_int_value"), sys_int_value, sig, &mut [val])
    } else if val_kind.contains(Float64) {
        let float_value = to_value::<f64>;
        let sig = get_type::<fn(f64) -> *const int>();
        func.insn_call_native1(Some("float_value"), float_value, sig, &mut [val])
    } else {
        fail!("Invalid kind {}", val_kind.bits())
    }
}