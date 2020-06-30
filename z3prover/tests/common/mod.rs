#[allow(dead_code)]
pub fn make_default_context() -> z3prover::Context {
    let cfg = z3prover::Config::new();
    let mtx = std::sync::Mutex::new(());
    let lock = mtx.lock().unwrap();
    z3prover::Context::new(&cfg, &lock)
}
