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
    rv_unit::test_panic_handler(info)
}

pub fn test_basic() {
    hprint!("Hello, World!");
    assert_eq!(1, 1);
}

#[entry]
fn main() -> ! {
    let tests: &[&dyn Testable] = &[&test_basic];
    rv_unit::test_runner(tests);
    loop {}
}
