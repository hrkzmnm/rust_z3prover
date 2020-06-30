mod common;


#[test]
fn mk_int() {
    let ctx = common::make_default_context();
    let sort = ctx.mk_sort(z3prover::sort::SortKind::Int);
    println!("sort = {:?}", &sort);
}

#[test]
fn mk_bool() {
    let ctx = common::make_default_context();
    let sort = ctx.mk_sort(z3prover::sort::SortKind::Bool);

    println!("sort = {:?}", &sort);
}

