#![no_std]
#![no_main]
// #![cfg_attr(test, no_main)]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use riscv_semihosting::{hprint, hprintln};
use riscv_semihosting::debug::exit;

static COLOR_RED: &str = "\u{001b}[31m";
static COLOR_GREEN: &str = "\u{001b}[32m";
static COLOR_YELLOW: &str = "\u{001b}[33m";
static COLOR_BLUE: &str = "\u{001b}[34m";
static COLOR_MAGENTA: &str = "\u{001b}[35m";
static COLOR_RESET: &str = "\u{001b}[0m";

static mut TEST_COUNT: i32 = 0;
static mut TEST_PASSED: i32 = 0;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        hprintln!("{}", core::any::type_name::<T>());
        self();
        unsafe {
            TEST_PASSED += 1;
        }
        hprintln!("{}{}{}", COLOR_GREEN, "[ok]", COLOR_RESET);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    hprintln!("{}{} tests{}", COLOR_BLUE, tests.len(), COLOR_RESET);
    
    for test in tests {
        unsafe {
            TEST_COUNT += 1;
        }
        test.run();
    }
    
    hprintln!("{}{} tests run{}", COLOR_BLUE, unsafe { TEST_COUNT }, COLOR_RESET);
    hprintln!("{}{} passed{}", COLOR_GREEN, unsafe { TEST_PASSED }, COLOR_RESET);
    hprintln!("{}{} failed{}", COLOR_RED, unsafe { TEST_COUNT - TEST_PASSED }, COLOR_RESET);
    
    exit(if unsafe { TEST_PASSED == TEST_COUNT } { Result::Ok(()) } else { Result::Err(()) });
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    hprintln!("{}{}{}", COLOR_RED, "[failed]", COLOR_RESET);
    hprintln!("{}{}{}", COLOR_RED, info, COLOR_RESET);
    exit(Result::Err(()));
    loop {}
}


