# RV Unit Testing Framework

A lightweight, no_std unit testing framework for RISC-V bare metal applications.

## Features

- No standard library dependencies (`no_std`)
- Simple test definition using function attributes
- Automatic test discovery and execution
- Clear test output using semihosting
- Support for both passing and failing test cases
- Custom panic handler for embedded rust

## Usage






### 5. Running Tests

To run the tests, use the following command:

```bash
cargo run --features rv_test
```

### Full example

```rust
#![no_std]
#![no_main]
#[cfg(feature = "rv_test")]
use rv_unit::Testable;

#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    #[cfg(feature = "rv_test")]
    return rv_unit::test_panic_handler(example::get_test_suite(), info);
    #[cfg(not(feature = "rv_test"))]
    loop {}
}

// Define test suite in a conditional block
#[cfg(feature = "rv_test")]
{
    pub fn test_addition() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }


    pub fn test_failing_assertion() {
        assert_eq!(2 + 2, 5, "This test should fail");
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
}

#[entry]
fn main() ->! {
    #[cfg(feature = "rv_test")]
    {
        // Run the test suite from example module
        test_runner(example::get_test_suite());
    }
    #[cfg(not(feature = "rv_test"))]
    hprintln!("Running in normal mode");
    loop {}
}
```

### Test Output

The framework provides clear output through semihosting:

- Shows the total number of tests being run
- Displays each test name as it executes
- Marks successful tests with `[ok]`
- Shows detailed error messages for failed tests
- Provides a summary of test results

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
