mod common;


#[test]
fn ast_new_const() {
    let ctx = common::make_default_context();
    let sort = ctx.mk_sort(z3_sys::SortKind::Int);
    let sym = ctx.mk_string_symbol("a").unwrap();
    let a = ctx.mk_const(&sym, &sort);
    println!("a = {:?}", &a);
}


// cargo test --test ast assert -- --nocapture
use z3prover::ast_assert;
#[test]
fn ast_assert() {
    let ctx = common::make_default_context();
    println!("ctx = {:?}", &ctx);
    ast_assert!(1);
}

