mod common;

#[test]
fn new() {
    let cfg = z3prover::Config::new();
    let ctx = {
        let mtx = std::sync::Mutex::new(());
        let lock = mtx.lock().unwrap();
        z3prover::Context::new(&cfg, &lock)
    };
    println!("ctx = {:?}", &ctx);
}

#[test]
fn interrupt() {
    let ctx = common::make_default_context();
    ctx.interrupt();
}

