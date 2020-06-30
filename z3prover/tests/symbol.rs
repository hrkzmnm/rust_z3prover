use z3prover;
mod common;

#[test]
fn test_string_symbol() {
    let mut ctx = common::make_default_context();
    let src = "SYM";
    let sym = z3prover::Symbol::from_cstring(&mut ctx, &std::ffi::CString::new(src).unwrap());
    let ret = sym.to_string().unwrap(); 
    println!("{:?}.to_string() -> {}", &sym, &ret);
    assert!(String::from(src) == ret);
}

#[test]
fn test_int_symbol() {
    let mut ctx = common::make_default_context();

    let i: i32 = 12345;
    let sym = z3prover::Symbol::from_int(&mut ctx, i);
    let s = sym.to_int().unwrap(); 
    println!("{:?}.to_int() -> {}", &sym, &s);
    assert!(i == s);
}