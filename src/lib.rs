#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use riscv_semihosting::hprintln;
use riscv_semihosting::debug::exit;

static TEST_FAILED: AtomicBool = AtomicBool::new(false);

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        hprintln!("{}...\t", core::any::type_name::<T>());
        let prev_failed = TEST_FAILED.load(Ordering::SeqCst);
        TEST_FAILED.store(false, Ordering::SeqCst);
        
        self();
        
        if !TEST_FAILED.load(Ordering::SeqCst) {
            hprintln!("[ok]");
        }
        TEST_FAILED.store(prev_failed, Ordering::SeqCst);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let mut failed_count = 0;
    hprintln!("Running {} tests", tests.len());
    
    for test in tests {
        let prev_failed = TEST_FAILED.load(Ordering::SeqCst);
        test.run();
        if TEST_FAILED.load(Ordering::SeqCst) && !prev_failed {
            failed_count += 1;
        }
    }
    
    if failed_count > 0 {
        hprintln!("\nTests failed: {}/{}", failed_count, tests.len());
    } else {
        hprintln!("\nAll tests passed!");
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    TEST_FAILED.store(true, Ordering::SeqCst);
    hprintln!("[failed]\n");
    hprintln!("Error: {}\n", info);
    // Use semihosting to exit QEMU with a failure status
    unsafe {
        exit(Result::Err(()))
    }
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

