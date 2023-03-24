use chapter11_3;

mod common;

// From the Rust book:
// Each file in the tests directory is a separate crate,
// so we need to bring our library into each test crate’s scope.
// For that reason we add use adder at the top of the code,
// which we didn’t need in the unit tests.

#[test]
fn two_plus_two() {
    common::setup_tests();

    let two_plus_two = chapter11_3::add(2, 2);
    assert_eq!(two_plus_two, 4);
}
