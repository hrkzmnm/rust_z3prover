mod common;

#[test]
fn null_solver() {
    let mut ctx = common::make_default_context();
    let solver = z3prover::Solver::new(&mut ctx);
    println!("solver = {:?}", &solver);
}
