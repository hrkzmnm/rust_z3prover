use super::Solver;
use super::Context;

impl <'ctx> Solver<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Solver<'ctx > {
        Solver {
            ctx,
            z3_solver: unsafe {
                let solver = z3_sys::Z3_mk_solver(ctx.z3_ctx);
                z3_sys::Z3_solver_inc_ref(ctx.z3_ctx, solver);
                solver
            },
        }
    }
    pub fn translate<'dst>(&self, dst: &'dst 
    Context) -> Solver<'dst> {
        Solver {
            ctx: dst,
            z3_solver: unsafe {
                let solver = z3_sys::Z3_solver_translate(self.ctx.z3_ctx, self.z3_solver, dst.z3_ctx);
                z3_sys::Z3_solver_inc_ref(dst.z3_ctx, solver);
                solver
            },
        }
    }

    pub fn get_context(&self) -> &'ctx Context {
        self.ctx
    }
}

impl <'ctx> Drop for Solver<'ctx> {
    fn drop(&mut self) {
        println!(" drop {:?}", &self);
        unsafe {
            z3_sys::Z3_solver_dec_ref(self.ctx.z3_ctx, self.z3_solver)
        }
    }
}