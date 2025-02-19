# RV Unit Testing Framework

A lightweight, no_std unit testing framework for RISC-V bare metal applications.

## Features

- No standard library dependencies (`no_std`)
- Simple test definition using function attributes
- Automatic test discovery and execution
- Clear test output using semihosting
- Support for both passing and failing test cases
- Custom panic handler for embedded rust

## Installation 
add the following to your `Cargo.toml`:
```
[dependencies]
rv_unit = { git = "https://github.com/rust-for-ssd/rv_unit.git" }
```

## Configuration
Add the following to your `Cargo.toml`:
```toml
[lib]
test = false
harness = false
```


### Usage
In your `test/example_test.rs` file:
```rust
// -- Imports and setup ---
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use riscv_rt::entry;
use core::panic::PanicInfo;
use rv_unit::Testable;

// -- Custom Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv_unit::test_panic_handler(TEST_SUITE, info)
}

// -- Run the tests
#[entry]
fn main() -> ! {
    rv_unit::test_runner(TEST_SUITE);
    loop {}
}

// --- Example: basic test suite ---

pub fn test_basic_positive() {
    assert_eq!(1, 1);
    assert_eq!(42, 42);
    assert!(true);
}

pub fn test_basic_negative() {
    assert_ne!(1, 2);
    assert_ne!(42, 0);
    assert!(!false);
}

pub fn test_basic_zero() {
    assert_eq!(0, 0);
    assert_ne!(0, 1);
}

pub fn test_negative (){
    assert_eq!(1, 2);
}

const TEST_SUITE: &[&dyn Testable] = &[
        &test_basic_positive, 
        &test_basic_negative, 
        &test_basic_zero,
        &test_negative];
```

To run the tests, use the following command:

```bash
cargo test
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
