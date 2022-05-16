//! Demonstration of using common code amongst multiple integration tests.

mod common;

#[test]
fn another_integration_test() {
    common::some_test_setup_code();
    // Write your test code here...
    assert!(true);
    common::some_test_breakdown_code();
}
