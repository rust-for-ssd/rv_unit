#![no_std]

use core::panic::PanicInfo;

use riscv_semihosting::hprintln;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        hprintln!("{}...\t", core::any::type_name::<T>());
        self();
        hprintln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    hprintln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    hprintln!("[failed]\n");
    hprintln!("Error: {}\n", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hprintln!("HERE!\n");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

