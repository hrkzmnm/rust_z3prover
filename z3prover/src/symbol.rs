use super::{Context, Symbol};

impl <'ctx> Symbol <'ctx> {
    pub fn from_cstring(ctx: &'ctx Context, name: &std::ffi::CString) -> Self {
        unsafe {
            let z3_symbol = z3_sys::Z3_mk_string_symbol(ctx.z3_ctx, name.as_ptr());
            Symbol {
                ctx: ctx,
                z3_symbol: z3_symbol,
            }
        }
    }
    pub fn to_string(&self) -> Option<String> {
        unsafe {
            match z3_sys::Z3_get_symbol_kind(self.ctx.z3_ctx, self.z3_symbol) {
                z3_sys::SymbolKind::Int => None,
                z3_sys::SymbolKind::String => {
                    let p = z3_sys::Z3_get_symbol_string(self.ctx.z3_ctx, self.z3_symbol);
                    match std::ffi::CStr::from_ptr(p).to_str() {
                        Ok(s) => Some(String::from(s)),
                        _ => None,
                    }
                }
            }
        }
    }
    pub fn from_int<T: Into<i32>> (ctx: &'ctx Context, i: T) -> Self  {
        // todo: i shall be < 2^30
        unsafe {
            let z3_symbol = z3_sys::Z3_mk_int_symbol(ctx.z3_ctx, i.into());
            Symbol {
                ctx: ctx,
                z3_symbol: z3_symbol,
            }
        }
    }
    pub fn to_int(&self) -> Option<i32> {
        unsafe {
            match z3_sys::Z3_get_symbol_kind(self.ctx.z3_ctx, self.z3_symbol) {
                z3_sys::SymbolKind::String => None,
                z3_sys::SymbolKind::Int => {
                    Some(z3_sys::Z3_get_symbol_int(self.ctx.z3_ctx, self.z3_symbol))
                }
            }
        }
    }
}

impl <'ctx> Drop for Symbol<'ctx> {
    fn drop(&mut self) {
        // no way to free a symbol?
    }
}
