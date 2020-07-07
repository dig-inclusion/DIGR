
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test_non_fields/*.rs");
}
