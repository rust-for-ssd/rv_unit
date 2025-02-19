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

// /// Entry point for the test runner
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     // Example test function
//     fn sample_test() {
//         hprintln!("This is a sample test");
//     }
    
//     // Run the test
//     let tests: &[&dyn Testable] = &[&sample_test];
//     test_runner(tests);
    
//     loop {}
// }

