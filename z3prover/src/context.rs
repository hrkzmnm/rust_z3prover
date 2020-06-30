use super::{Context, Config, Solver, Sort, Symbol, Ast};

impl <'ctx> Context {
    pub fn new<T> (cfg: &Config, _mtx: &std::sync::MutexGuard<'_, T>) -> Context {
        Context {
            z3_ctx: unsafe {
                z3_sys::Z3_mk_context_rc(cfg.z3_cfg)
            },
        }
    }
    pub fn interrupt(&self) {
        unsafe {
            z3_sys::Z3_interrupt(self.z3_ctx)
        }
    }
    pub fn set_error_handler(&self, h: z3_sys::Z3_error_handler) {
        unsafe {
            z3_sys::Z3_set_error_handler(self.z3_ctx, h)
        }
    }
    pub fn mk_sort(&'ctx self, kind: z3_sys::SortKind) -> Sort<'ctx> {
        Sort::new(self, kind)
    }
    pub fn mk_string_symbol(&'ctx self, name: &str) -> Option<Symbol<'ctx>> {
        match std::ffi::CString::new(name) {
            Ok(name) => Some(Symbol::from_cstring(self, &name)),
            Err(_) => None,
        }
    }
    pub fn mk_int_symbol<T: Into<i32>>(&'ctx self, i: T) -> Symbol<'ctx> {
        Symbol::from_int(self, i)
    }
    pub fn mk_const(&'ctx self, symbol: &Symbol<'ctx>, sort: &Sort<'ctx>) -> Ast<'ctx> {
        Ast::mk_const(self, &symbol, &sort)
    }
    pub fn mk_solver(&'ctx self) -> Solver<'ctx> {
        Solver::new(self)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_del_context(self.z3_ctx) };
    }
}
