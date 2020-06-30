use super::Sort;
use super::Context;
pub use z3_sys::SortKind;

#[allow(non_snake_case)]
impl <'ctx> Sort <'ctx> {
    pub fn new(ctx: &'ctx Context, kind: SortKind) -> Sort<'ctx> {
        Sort::new_(ctx, &kind) // use borrow to circumvent https://github.com/rust-lang/rust/issues/67776
    }
    fn new_(ctx: &'ctx Context, kind: &SortKind) -> Sort<'ctx> {
        let z3_ctx = ctx.z3_ctx;
        unsafe {
            let z3_sort = match kind {
                //Uninterpreted => z3_sys::Z3_mk_uninterpreted_sort(z3_ctx, sym),
                z3_sys::SortKind::Bool => z3_sys::Z3_mk_bool_sort(z3_ctx),
                z3_sys::SortKind::Int => z3_sys::Z3_mk_int_sort(z3_ctx),
                z3_sys::SortKind::Real => z3_sys::Z3_mk_real_sort(z3_ctx),
                //BV => z3_sys::Z3_mk_bv_sort(z3_ctx, size),
                //Array => z3_sys::Z3_mk_array_sort(z3_ctx, domain: Z3_sort, range: Z3_sort),
                //Datatype => z3_sys::Z3_mk_datatype(c: Z3_context, name: Z3_symbol, num_constructors: ::std::os::raw::c_uint, constructors: *mut Z3_constructor)(z3_ctx),
                //RE => z3_sys::Z3_mk_re_sort(z3_ctx, seq: Z3_sort),
                _ =>  todo!("unimplemented"),
            };
            Sort {
                ctx: ctx,
                z3_sort: z3_sort,
            }
        }
    }
}

impl <'ctx> Drop for Sort<'ctx> {
    fn drop(&mut self) {
        unsafe {
            z3_sys::Z3_dec_ref(
                self.ctx.z3_ctx,
                z3_sys::Z3_sort_to_ast(self.ctx.z3_ctx, self.z3_sort),
            );
        }
    }
}
