#![no_std]
#![no_main]

#[cfg(feature = "test")]
use rv_unit::{test_runner, Testable};
use riscv_semihosting::hprintln;
use riscv_rt::entry;

mod example;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(feature = "test")]
    return rv_unit::test_panic_handler(example::get_test_suite(), info);

    #[cfg(not(feature = "test"))]
    loop {}
}

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
