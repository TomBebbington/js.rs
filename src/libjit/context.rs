use bindings::{
	jit_context_t,
	jit_context_create,
	jit_context_destroy,
	jit_context_build_start,
	jit_context_build_end,
	jit_function_t,
	jit_function_next
};
use function::Function;
use std::kinds::marker::ContravariantLifetime;
use util::NativeRef;
/// Holds all of the functions you have built and compiled. There can be multiple, but normally there is only one.
native_ref!(Context, _context, jit_context_t)
impl Context {
	/// Create a new JIT Context
	pub fn new() -> Context {
		unsafe {
			NativeRef::from_ptr(jit_context_create())
		}
	}
	/// Run a closure that can generate IR
	pub fn build<R>(&self, cb: || -> R) -> R {
		unsafe {
			jit_context_build_start(self.as_ptr());
		}
		let rv = cb();
		unsafe {
			jit_context_build_end(self.as_ptr());
		}
		rv
	}
	/// Iterate through all the functions in this context
	pub fn iter_funcs<'t>(&'t self) -> Functions<'t> {
		Functions::new(self)
	}
}

impl Drop for Context {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			jit_context_destroy(self.as_ptr());
		}
	}
}


/// An iterator over a context's functions
pub struct Functions<'a> {
	ctx: jit_context_t,
	last: jit_function_t,
	marker: ContravariantLifetime<'a>
}
impl<'a> Functions<'a> {
	fn new(ctx:&'a Context) -> Functions<'a> {
		unsafe {
			Functions {
				ctx: ctx.as_ptr(),
				last: RawPtr::null(),
				marker: ContravariantLifetime::<'a>
			}
		}
	}
}
impl<'a> Iterator<Function> for Functions<'a> {
	fn next(&mut self) -> Option<Function> {
		unsafe {
			let native_next = jit_function_next(self.ctx, self.last);
			self.last = native_next;
			NativeRef::from_ptr_opt(native_next)
		}
	}
	fn size_hint(&self) -> (uint, Option<uint>) {
		unsafe {
			let mut size : uint = 0;
			let mut last = self.last;
			loop {
				last = jit_function_next(self.ctx, last);
				if last.is_null() {
					break;
				} else {
					size += 1;
				}
			}
			(size, Some(size))
		}
	}
}