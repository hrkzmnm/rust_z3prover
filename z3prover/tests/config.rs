use z3prover::{Config, Context};

#[test]
fn ctor() {
    let cfg = Config::new();
    let mtx = std::sync::Mutex::new(());
    let lock = mtx.lock().unwrap();

    let _ctx = Context::new(&cfg, &lock);
}

