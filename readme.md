# RV Unit Testing Framework

A lightweight, no_std unit testing framework for RISC-V bare metal applications.

## Features

- No standard library dependencies (`no_std`)
- Simple test definition using function attributes
- Automatic test discovery and execution
- Clear test output using semihosting
- Support for both passing and failing test cases
- Custom panic handler for embedded rust

## Requirements

The project relies on an experimental feature `#![feature(custom_test_frameworks)]` that requires a *nightly* version of the Rust compiler.

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


## Usage

In your `test/example_test.rs` file:
```rust
// -- Imports and setup ---
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use riscv_rt::entry;

// -- Custom panic handler 
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    rv_unit::test_panic_handler(info);
    test_main();
    loop {}
}

#[entry]
fn main() -> ! {
    #[cfg(test)] test_main();
    loop {}
}

// --- Example: basic test cases ---

#[test_case]
pub fn test_basic_positive() {
    assert_eq!(1, 1);
    assert_eq!(42, 42);
    assert!(true);
}

#[test_case]
pub fn test_basic_negative() {
    assert_ne!(1, 2);
    assert_ne!(42, 0);
    assert!(!false);
}

#[test_case]
pub fn test_basic_zero() {
    assert_eq!(0, 0);
    assert_ne!(0, 1);
}

#[test_case]
pub fn test_negative (){
    assert_eq!(1, 2);
}

```

To run the tests, use the following command:

```bash
cargo test
```

## Test Output

The framework provides clear output through semihosting:

- Shows the total number of tests being run
- Displays each test name as it executes
- Marks successful tests with `[ok]`
- Shows detailed error messages for failed tests
- Provides a summary of test results


Example output:
```bash
Running 4 tests
[0] - example_test::test_basic_negative
[ok]
[1] - example_test::test_basic_positive
[ok]
[2] - example_test::test_basic_zero
[ok]
[3] - example_test::test_negative
[failed]
panicked at tests/example_test.rs:48:5:
assertion `left == right` failed
  left: 1
 right: 2
Ran 4 tests
Passed: 3
Failed: 1
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
