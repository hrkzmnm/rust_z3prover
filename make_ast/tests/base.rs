use make_ast::build;

use std::rc::Rc;

#[derive(Debug)]
struct Ctx {
    dummy_ptr: usize, 
}

impl Drop for Ctx {
    fn drop(&mut self) {
//        println!("!! dropping Ctx {:?}", self);
    }
}


#[derive(Debug)]
enum AstState<'c> {
    Ct(i32),
    UnOp(UnOp, Rc<Ast<'c>>),
    BinOp(BinOp, Rc<Ast<'c>>, Rc<Ast<'c>>),
}

#[derive(Debug)]
struct Ast<'c> {
    ctx: &'c Ctx, 
    state: AstState<'c>,
}

impl <'c> Ast<'c> {
    fn new(ctx: &'c Ctx, state: AstState<'c> ) -> Rc<Ast<'c>> {
        let ast = Rc::new(Ast {
            ctx: ctx,
            state: state,    
        });
        ast
    }
}
impl <'c> Drop for Ast<'c> {
    fn drop(&mut self) {
//        println!("! dropping ast {:?}", self);
    }
}

//////////////////////////////////////////////////////////////////////

macro_rules! for_all_unops {
    ($call:ident, $arg0:tt) => {
        $call!{$arg0, Neg Not}
    }
}
macro_rules! for_all_binops {
    ($call:ident, $arg0:tt) => {
        $call!{$arg0, Eq Ne Lt Gt Le Ge Add Sub Mul Div Rem And Or}
    }
}

macro_rules! call_with_name {
    ($call:tt, $($name:ident)*) => {
        $(
            $call!{$name}
        )*
    }
}
//////////////////////////////////////////////////////////////////////

macro_rules! make_enum {
    ($arg0:tt, $($name:ident)*) => {
        #[derive(Debug)]
        enum $arg0 {
            $($name,)*
        }        
    }
}
for_all_unops!(make_enum, UnOp); // Enum UnOp {Neq, Not};
for_all_binops!(make_enum, BinOp); // Enum Binop {...}

//////////////////////////////////////////////////////////////////////
macro_rules! decl_unop {
    ($name:ident) => {
        #[allow(non_snake_case)]
        fn $name<'c>(&'c self, arg: &Rc<Ast<'c>>) -> Rc<Ast<'c>>;    
    }
}
macro_rules! decl_binop {
    ($name:ident) => {
        #[allow(non_snake_case)]
        fn $name<'c>(&'c self, left: &Rc<Ast<'c>>, right: &Rc<Ast<'c>>) -> Rc<Ast<'c>>;
    }
}
trait Builder {
    for_all_unops!(call_with_name, decl_unop);
    for_all_binops!(call_with_name, decl_binop);

    fn ct_int(&self, val: i32) -> Rc<Ast>;
}
//////////////////////////////////////////////////////////////////////
macro_rules! impl_unop {
    ($name:ident) => {
        #[allow(non_snake_case)]
        fn $name<'c>(&'c self, arg: &Rc<Ast<'c>>) -> Rc<Ast<'c>> {
            Ast::new(self,
                AstState::UnOp(UnOp::$name,
                    Rc::clone(arg)
                )
            )
        }
    }    
}
macro_rules! impl_binop {
    ($name:ident) => {
        #[allow(non_snake_case)]
        fn $name<'c>(&'c self, left: &Rc<Ast<'c>>, right: &Rc<Ast<'c>>) -> Rc<Ast<'c>> {
            Ast::new(self,
                AstState::BinOp(BinOp::$name,
                    Rc::clone(left), Rc::clone(right)
                )
            )
        }
    }    
}
impl Builder for Ctx {
    for_all_unops!(call_with_name, impl_unop);
    for_all_binops!(call_with_name, impl_binop);

    fn ct_int(&self, val: i32) -> Rc<Ast> {
        Ast::new(self, AstState::Ct(val))
    }
}
//////////////////////////////////////////////////////////////////////

fn init_ctx() -> Ctx {
    Ctx {
        dummy_ptr: 0,
    }
}

#[test]
fn test_base() {
    let ctx = init_ctx();
    println!("\t ctx = {:?}", ctx);
}
#[test]
fn test_const_int() {
    // cargo test const_int -- --nocapture 
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1));
}
#[test]
fn test_binop_neg() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, -1));
}
#[test]
fn test_binop_neg2() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, --1));
}
#[test]
fn test_binop_not() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, !1));
}
#[test]
fn test_binop_eq() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 == 2));
}
#[test]
fn test_binop_ne() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 != 2));
}
#[test]
fn test_binop_lt() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 < 2));
}
#[test]
fn test_binop_gt() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 > 2));
}
#[test]
fn test_binop_le() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 <= 2));
}
#[test]
fn test_binop_ge() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 >= 2));
}
#[test]
fn test_binop_addd() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 + 2));
}
#[test]
fn test_binop_sub() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 - 2));
}
#[test]
fn test_binop_mul() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 * 2));
}
#[test]
fn test_binop_div() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 / 2));
}
#[test]
fn test_binop_rem() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, 1 % 2));
}
#[test]
fn test_binop_paren() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, (1 + 2) ));
}
#[test]
fn test_binop_paren2() {
    let ctx = init_ctx();
    println!("\t expanded = {:?}", build!(ctx, (1 + 2 + 3) * 4));
}

#[test]
fn test_synth() {
    let ctx = init_ctx();
    let ast = build!(ctx, 1 < 2);
    println!("built ast = {:?}", ast);
}
