mod common;

#[test]
fn test_config() {
    let cfg = z3prover::Config::new();
    println!("cfg = {:?}", &cfg);
}

#[test]
fn test_assert() {
    assert_eq!(2, 1 + 1);
}

#[test]
#[should_panic]
fn test_badname() {
    let badname = vec![1,0,2];
    std::ffi::CString::new(badname).unwrap();
}