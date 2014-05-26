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
#![allow(missing_doc)]
#![allow(raw_pointer_deriving)]
#![stable]

extern crate libc;
extern crate core;


use std::ptr;
use core::mem::transmute;
use libc::{FILE, STDOUT_FILENO, c_int, fdopen, c_void, c_uint, c_char, c_float};

pub enum ABI {
    CDECL = 0
}

pub enum CallFlags {
    JitCallNothrow = 1,
    JitCallNoReturn = 2,
    JitCallTail = 4,
}

#[link(name = "jit")]
extern {
    fn jit_context_create() -> *c_void;
    fn jit_context_destroy(context: *c_void);
    fn jit_context_build_start(context: *c_void);
    fn jit_context_build_end(context: *c_void);
    fn jit_function_create(context: *c_void, signature: *c_void) -> *c_void;
    fn jit_function_compile(function: *c_void);
    fn jit_type_create_signature(abi: c_int, return_type: *c_void, params: **c_void, num_params: c_uint, incref: c_int) -> *c_void;
    fn jit_type_create_struct(fields: **c_void, num_fields: c_uint, incref: c_int) -> *c_void;
    fn jit_type_create_union(fields: **c_void, num_fields: c_uint, incref: c_int) -> *c_void;
    fn jit_type_create_pointer(pointee: *c_void, incref: c_int) -> *c_void;
    fn jit_type_get_size(ty: *c_void) -> c_uint;
    fn jit_value_get_param(function: *c_void, param: c_uint) -> *c_void;
    fn jit_insn_return(function: *c_void, value: *c_void);
    fn jit_function_apply(function: *c_void, args: **c_void, return_area: *mut c_void);
    fn jit_insn_add(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_mul(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_sub(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_div(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_and(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_or(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_xor(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_not(function: *c_void, value: *c_void) -> *c_void;
    fn jit_insn_neg(function: *c_void, value: *c_void) -> *c_void;
    fn jit_insn_load(function: *c_void, value: *c_void) -> *c_void;
    fn jit_value_create(function: *c_void, value_type: *c_void) -> *c_void;
    fn jit_insn_label(function: *c_void, label: *mut c_void);
    fn jit_insn_branch(function: *c_void, label: *mut c_void);
    fn jit_insn_le(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_ge(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_lt(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_gt(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_eq(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_ne(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
    fn jit_insn_branch_if(function: *c_void, value: *c_void, label: *mut c_void);
    fn jit_insn_branch_if_not(function: *c_void, value: *c_void, label: *mut c_void);
    fn jit_insn_store(function: *c_void, dest: *c_void, src: *c_void);
    fn jit_insn_store_relative(function: *c_void, dest: *c_void, offset: c_int, src: *c_void);
    fn jit_insn_call_native(function: *c_void, name: *c_char, native_func: *u8, signature: *c_void, args: **c_void, num_args: c_uint, flags: c_int) -> *c_void;
    fn jit_insn_alloca(function: *c_void, size: *c_void) -> *c_void;
    fn jit_dump_function (stream: *FILE, funcion: *c_void, name: *c_char);
    fn jit_value_create_float32_constant(function: *c_void, value_type: *c_void, value: c_float) -> *c_void;
    fn jit_value_create_nint_constant(function: *c_void, value_type: *c_void, value: c_int) -> *c_void;
    fn jit_function_to_closure(function: *c_void) -> *c_void;

    static jit_type_void: *c_void;
    static jit_type_int: *c_void;
    static jit_type_float32: *c_void;
    static jit_type_float64: *c_void;
    static jit_type_void_ptr: *c_void;
}

pub struct Context {
    _context: *c_void
}

impl Context {
    pub fn new() -> Box<Context> {
        unsafe {
            let context = jit_context_create();
            box Context { _context: context }
        }
    }

    pub fn build_start(&self) {
        unsafe {
            jit_context_build_start(self._context);
        }
    }

    pub fn build_end(&self) {
        unsafe {
            jit_context_build_end(self._context);
        }
    }

    pub fn create_function(&self, signature: &Type) -> Box<Function> {
        unsafe {
            let function = jit_function_create(self._context, signature._type);
            box Function { _context: self, _function: function }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            jit_context_destroy(self._context);
        }
    }
}

pub struct Type {
    _type: *c_void
}

impl Type {
    pub fn create_signature(abi: ABI, return_type: &Type, params: &[&Type]) -> Box<Type> {
        unsafe {
            let mut ps: Vec<*c_void> = vec!();

            for param in params.iter() {
                ps.push(param._type);
            }

            let params = if ps.len() > 0 { ps.as_ptr() } else { 0 as **c_void };

            let signature = jit_type_create_signature(abi as c_int, return_type._type, params, ps.len() as c_uint, 1);
            box Type { _type: signature }
        }
    }

    fn create_complex(fields: &[&Type], union: bool) -> Box<Type> {
        unsafe {
            let mut fs: Vec<*c_void> = vec!();

            for field in fields.iter() {
                fs.push(field._type);
            }

            let fields = if fs.len() > 0 { fs.as_ptr() } else { 0 as **c_void };
            let f = if union { jit_type_create_union } else { jit_type_create_struct };
            let ty = f(fields, fs.len() as c_uint, 1);
            box Type { _type: ty }
        }
    }

    pub fn create_struct(fields: &[&Type]) -> Box<Type> {
        Type::create_complex(fields, false)
    }

    pub fn create_union(fields: &[&Type]) -> Box<Type> {
        let inner = Type::create_complex(fields, true);
        Type::create_struct(&[&*Types::get_int(), &*inner])
    }

    pub fn create_pointer(pointee: &Type) -> Box<Type> {
        unsafe {
            let ptr = jit_type_create_pointer(pointee._type, 1);
            box Type { _type: ptr }
        }
    }

    pub fn get_size(&self) -> c_uint {
        unsafe {
            jit_type_get_size(self._type)
        }
    }
}

pub struct Function {
    _context: *Context,
    _function: *c_void
}

impl Function {
    fn insn_binop(&self, v1: &Value, v2: &Value, f: unsafe extern "C" fn(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void) -> Box<Value> {
        unsafe {
            let value = f(self._function, v1._value, v2._value);
            box Value { _value: value }
        }
    }

    fn insn_unop(&self, value: &Value, f: unsafe extern "C" fn(function: *c_void, value: *c_void) -> *c_void) -> Box<Value> {
        unsafe {
            let value = f(self._function, value._value);
            box Value { _value: value }
        }
    }

    pub fn dump(&self, name: &str) {
        unsafe {
            let c_str = name.to_c_str().unwrap();
            let mode = "w";
            let stdout = fdopen(STDOUT_FILENO, transmute(&mode[0]));
            jit_dump_function(stdout, self._function, c_str);
        }
    }

    pub fn compile(&self) {
        unsafe {
            jit_function_compile(self._function);
        }
    }

    pub fn get_param(&self, param: uint) -> Value {
        unsafe {
            let value = jit_value_get_param(self._function, param as c_uint);
            Value { _value: value }
        }
    }

    pub fn insn_return(&self, retval: &Value) {
        unsafe {
            jit_insn_return(self._function, retval._value);
        }
    }

    pub fn insn_mul(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_mul)
    }

    pub fn insn_add(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_add)
    }

    pub fn insn_sub(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_sub)
    }

    pub fn insn_div(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_div)
    }

    pub fn insn_leq(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_le)
    }

    pub fn insn_geq(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_ge)
    }

    pub fn insn_lt(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_lt)
    }

    pub fn insn_gt(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_gt)
    }

    pub fn insn_eq(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_eq)
    }

    pub fn insn_neq(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_ne)
    }

    pub fn insn_and(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_and)
    }

    pub fn insn_or(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_or)
    }

    pub fn insn_xor(&self, v1: &Value, v2: &Value) -> Box<Value> {
        self.insn_binop(v1, v2, jit_insn_xor)
    }

    pub fn insn_not(&self, value: &Value) -> Box<Value> {
        self.insn_unop(value, jit_insn_not)
    }

    pub fn insn_neg(&self, value: &Value) -> Box<Value> {
        self.insn_unop(value, jit_insn_neg)
    }

    pub fn insn_dup(&self, value: &Value) -> Box<Value> {
        unsafe {
            let dup_value = jit_insn_load(self._function, value._value);
            box Value { _value: dup_value }
        }
    }

    pub fn insn_store(&self, dest: &Value, src: &Value) {
        unsafe {
            jit_insn_store(self._function, dest._value, src._value);
        }
    }

    pub fn insn_store_relative(&self, dest: &Value, offset: c_int, src: &Value) {
        unsafe {
            jit_insn_store_relative(self._function, dest._value, offset, src._value);
        }
    }

    pub fn insn_set_label(&self, label: &mut Label) {
        unsafe {
            // TODO: messy; I suspect transmute is REALLY BAD PRACTICE
            let label_ptr: *mut c_void = transmute(&label._label);
            jit_insn_label(self._function, label_ptr);
        }
    }

    pub fn insn_branch(&self, label: &mut Label) {
        unsafe {
            let label_ptr: *mut c_void = transmute(&label._label);
            jit_insn_branch(self._function, label_ptr);
        }
    }

    pub fn insn_branch_if(&self, value: &Value, label: &mut Label) {
        unsafe {
            let label_ptr: *mut c_void = transmute(&label._label);
            jit_insn_branch_if(self._function, value._value, label_ptr);
        }
    }

    pub fn insn_branch_if_not(&self, value: &Value, label: &mut Label) {
        unsafe {
            let label_ptr: *mut c_void = transmute(&label._label);
            jit_insn_branch_if_not(self._function, value._value, label_ptr);
        }
    }

    fn insn_call_native(&self, name: &'static str, native_func: *u8,
                        signature: &Type, args: &[&Value]) -> Box<Value> {
        unsafe {
            let mut as_: Vec<*c_void> = vec!();

            for arg in args.iter() {
                as_.push(arg._value);
            }

            let args = if as_.len() > 0 { as_.as_ptr() } else { 0 as **c_void };
            name.with_c_str(|name| {
                box Value {
                    _value: jit_insn_call_native(self._function, name, native_func,
                                                 signature._type, args, as_.len() as c_uint,
                                                 JitCallNothrow as c_int)
                }
            })
        }
    }

    pub fn insn_call_native0<R>(&self, name: &'static str,
                                native_func: fn() -> R,
                                signature: &Type, args: &[&Value]) -> Box<Value> {
        self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
    }

    pub fn insn_call_native1<A,R>(&self, name: &'static str,
                                  native_func: fn(A) -> R,
                                  signature: &Type, args: &[&Value]) -> Box<Value> {
        self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
    }

    pub fn insn_alloca(&self, size: &Value) -> Box<Value> {
        unsafe {
            box Value { _value: jit_insn_alloca(self._function, size._value) }
        }
    }

    pub fn apply<T>(&self, args: &[*c_void], retval: &mut T) {
        unsafe {
            let pargs = args;
            jit_function_apply(self._function, pargs.as_ptr(), transmute(retval));
        }
    }

    pub fn execute(&self, args: &[*c_void]) {
        unsafe {
            let pargs = args;
            jit_function_apply(self._function, pargs.as_ptr(), ptr::mut_null());
        }
    }

    pub fn closure<T>(&self) -> T {
        unsafe {
            transmute(jit_function_to_closure(self._function))
        }
    }

    pub fn constant_float32(&self, constant: f32) -> Box<Value> {
        unsafe {
            let value = jit_value_create_float32_constant(self._function, jit_type_float32, constant);
            box Value { _value: value }
        }
    }

    pub fn constant_int32(&self, constant: i32) -> Box<Value> {
        unsafe {
            let value = jit_value_create_nint_constant(self._function, jit_type_int, constant);
            box Value { _value: value }
        }
    }

    pub fn create_value(&self, value_type: &Type) -> Box<Value> {
        unsafe {
            let value = jit_value_create(self._function, value_type._type);
            box Value { _value: value }
        }
    }
}

#[deriving(Clone)]
pub struct Value {
    _value: *c_void
}

#[deriving(Hash, Eq)]
pub struct Label {
    _label: *c_void
}

impl Label {
    fn undefined() -> *c_void {
        !0u32 as *c_void
    }

    pub fn new() -> Box<Label> {
        box Label { _label: Label::undefined() }
    }
}

pub struct Types;
impl Types {
    pub fn get_void() -> Box<Type> {
        box Type { _type: jit_type_void }
    }

    pub fn get_int() -> Box<Type> {
        box Type { _type: jit_type_int }
    }

    pub fn get_float32() -> Box<Type> {
        box Type { _type: jit_type_float32 }
    }

    pub fn get_float64() -> Box<Type> {
        box Type { _type: jit_type_float64 }
    }

    pub fn get_void_ptr() -> Box<Type> {
        box Type { _type: jit_type_void_ptr }
    }
}
