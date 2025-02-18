#![no_std]
#![no_main]

use rv_unit::test_runner;
use riscv_semihosting::hprintln;

use panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    hprintln!("Hello World!");

    loop { }
}
