#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/error_spanned.rs");
}
