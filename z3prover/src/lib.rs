#![deny(anonymous_parameters)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]
//#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_code)]


#[derive(Debug)]
pub struct Config {
    pub z3_cfg: z3_sys::Z3_config,    
    _private: () // shall not be used as a literal
}
mod config;


#[derive(Eq, PartialEq, Debug)]
pub struct Context {
    pub z3_ctx: z3_sys::Z3_context,
}
pub mod context;

#[derive(PartialEq, Debug)]
pub struct Ast<'ctx> {
    ctx: &'ctx Context,
    pub z3_ast: z3_sys::Z3_ast,
}
pub mod ast;


#[derive(PartialEq, Debug)]
pub struct Symbol<'ctx> {
    ctx: &'ctx Context,
    pub z3_symbol: z3_sys::Z3_symbol,
}
mod symbol;

#[derive(PartialEq, Debug)]
pub struct Sort<'ctx> {
    ctx: &'ctx Context,
    z3_sort: z3_sys::Z3_sort,
}
pub mod sort;

#[derive(PartialEq, Debug)]
pub struct Solver<'ctx> {
    ctx: &'ctx Context,
    z3_solver: z3_sys::Z3_solver,
}
mod solver;
