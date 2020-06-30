use super::{Context, Symbol, Sort, Ast};

#[macro_export]
macro_rules! ast_assert {
    ($x:expr) => { println!("{}", $x); }
}


impl <'ctx> Ast<'ctx> {
    pub fn mk_const(ctx: &'ctx Context, symbol: &Symbol<'ctx >, sort: &Sort<'ctx >,
        ) -> Ast<'ctx> {
        Ast {
            ctx: ctx,
            z3_ast: unsafe {
                let z3_ast = z3_sys::Z3_mk_const(ctx.z3_ctx, symbol.z3_symbol, sort.z3_sort);
                z3_sys::Z3_inc_ref(ctx.z3_ctx, z3_ast);
                z3_ast
            }               
        }
    } 
}

impl <'ctx> Drop for Ast<'ctx> {
    fn drop(&mut self) {
        println!(" drop {:?}", &self);
        unsafe {
            z3_sys::Z3_dec_ref(self.ctx.z3_ctx, self.z3_ast)
        }
    }
}