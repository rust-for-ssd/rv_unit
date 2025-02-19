# RV Unit Testing Framework

A lightweight, no_std unit testing framework for RISC-V bare metal applications.

## Features

- No standard library dependencies (`no_std`)
- Simple test definition using function attributes
- Automatic test discovery and execution
- Clear test output using semihosting
- Support for both passing and failing test cases

## Usage

### Conditional import of the framework
In order to use `rv_unit` in your project, you can conditionally import it based on a feature flag. For example:
```rust
#[cfg(feature = "rv_test")]
use rv_unit::Testable;

#[cfg(feature = "rv_test")]
pub fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
```


### Conditional panic handler
It's important to import the custom testing panic handler into your project. See the following example fond in `src/main.rs`:
```rust
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "test")]
    return rv_unit::test_panic_handler(example::get_test_suite(), info);

    #[cfg(not(feature = "test"))]
    loop {}
}
```

### Exectuing the handler
To execute the handler, put it in the main function as the project example in `src/main.rs`: 
```rust
#[entry]
fn main() -> ! {
    #[cfg(feature = "test")]
    {
        // Run the test suite from example module
        test_runner(example::get_test_suite());
    }

    #[cfg(not(feature = "test"))]
    hprintln!("Running in normal mode");

    loop {}
}
```


### Writing Tests

Tests are regular Rust functions that use assertions to verify expected behavior. Here's how to write tests:

```rust
// Simple test function
pub fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

// Test that should fail
pub fn test_failing_assertion() {
    assert_eq!(2 + 2, 5, "This test should fail");
}

// Test with explicit panic
pub fn test_explicit_panic() {
    panic!("This test should panic explicitly");
}
```

### Running Tests

To run tests, create an array of test functions and pass it to the test runner:

```rust
use rv_unit::Testable;

// Create an array of test functions
let tests: &[&dyn Testable] = &[
    &test_addition,
    &test_subtraction,
    &test_failing_assertion
];

// Run the tests
test_runner(tests);
```

### Test Output

The framework provides clear output through semihosting:

- Shows the total number of tests being run
- Displays each test name as it executes
- Marks successful tests with `[ok]`
- Shows detailed error messages for failed tests
- Provides a summary of test results

### Example Test Suite

Here's a complete example of a test suite:

```rust
use rv_unit::Testable;

pub fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

pub fn test_subtraction() {
    let result = 5 - 3;
    assert_eq!(result, 2);
}

pub fn get_test_suite() -> &'static [&'static dyn Testable] {
    &[
        &test_addition,
        &test_subtraction
    ]
}
```

## Error Handling

The framework includes a panic handler that provides detailed error information when tests fail. Failed tests will:

- Display the test name
- Show the failure message
- Indicate the location of the failure

## Best Practices

1. Write small, focused test functions
2. Use descriptive test names
3. Include meaningful assertion messages
4. Group related tests together
5. Test both success and failure cases

## License

This project is open source and available under the MIT license.