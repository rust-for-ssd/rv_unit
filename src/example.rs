use rv_unit::Testable;

pub fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

pub fn test_subtraction() {
    let result = 5 - 3;
    assert_eq!(result, 2);
}

pub fn test_failing_assertion() {
    assert_eq!(2 + 2, 5, "This test should fail");
}

pub fn test_failing_boolean() {
    assert!(false, "This boolean assertion should fail");
}

pub fn test_explicit_panic() {
    panic!("This test should panic explicitly");
}

pub fn get_test_suite() -> &'static [&'static dyn Testable] {
    &[
        &test_addition,
        &test_subtraction,
        &test_failing_assertion,
        &test_failing_boolean,
        &test_explicit_panic
    ]
}