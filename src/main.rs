#![no_std]
#![no_main]

use rv_unit::{test_runner, Testable};
use riscv_semihosting::hprintln;
use riscv_rt::entry;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv_unit::test_panic_handler(info)
}

#[entry]
fn main() -> ! {
    // Define some test functions
    fn test_addition() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn test_subtraction() {
        let result = 5 - 3;
        assert_eq!(result, 2);
    }

    fn test_failing_assertion() {
        assert_eq!(2 + 2, 5, "This test should fail");
    }

    fn test_failing_boolean() {
        assert!(false, "This boolean assertion should fail");
    }

    fn test_explicit_panic() {
        panic!("This test should panic explicitly");
    }

    // Create an array of test functions
    let tests: &[&dyn Testable] = &[
        &test_addition,
        &test_subtraction,
        &test_failing_assertion,
        &test_failing_boolean,
        &test_explicit_panic
    ];
    
    // Run the tests
    test_runner(tests);

    loop {}
}
