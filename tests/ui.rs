use trybuild;

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/functional/fail/*.rs");
    t.pass("tests/functional/pass/*.rs");
}
