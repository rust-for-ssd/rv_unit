#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use riscv_rt::entry;
use riscv_semihosting::hprint;
use core::panic::PanicInfo;
use rv_unit::Testable;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv_unit::test_panic_handler(TESTS, info)
}

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

const TESTS: &[&dyn Testable] = &[
        &test_basic_positive, 
        &test_basic_negative, 
        &test_basic_zero,
        &test_negative];

#[entry]
fn main() -> ! {
    rv_unit::test_runner(TESTS);
    loop {}
}
