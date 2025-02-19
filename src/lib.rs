#![no_std]

use core::panic::PanicInfo;
use riscv_semihosting::hprintln;
use riscv_semihosting::debug::exit;

static mut TEST_COUNT: i32 = 0;
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

    let test_count = unsafe {
        TEST_COUNT
    };

    if test_count == 0 {
        hprintln!("Running {} tests", tests.len());
    }

    for i in test_count..tests.len() as i32 {
        hprintln!("{}..", i);
        unsafe {
            TEST_COUNT += 1;
        }
        tests[i as usize].run();
    }
    
    hprintln!("\nAll {} tests completed!", test_count);
    exit(Result::Err(()));
}

pub fn test_panic_handler(tests: &[&dyn Testable], info: &PanicInfo) -> ! {
    hprintln!("[failed]\n");
    hprintln!("Error: {}\n", info);
    // Use semihosting to exit QEMU with a failure status
    test_runner(tests);
    loop {} // we need the loop just for the return type
}

