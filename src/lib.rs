#![no_std]

use core::panic::PanicInfo;
use riscv_semihosting::hprintln;
use riscv_semihosting::debug::exit;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        hprintln!("{}..", core::any::type_name::<T>());
        self();
        hprintln!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let mut test_count = 0;
    hprintln!("Running {} tests", tests.len());
    
    for test in tests {
        test_count += 1;
        test.run();
    }
    
    hprintln!("\nAll {} tests completed!", test_count);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    hprintln!("[failed]\n");
    hprintln!("Error: {}\n", info);
    // Use semihosting to exit QEMU with a failure status
    unsafe {
        exit(Result::Err(()))
    }
    loop {}
}

