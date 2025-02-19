#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_semihosting::{hprint, hprintln};
use riscv_semihosting::debug::exit;


static COLOR_RED: &str = "[31m";
static COLOR_GREEN: &str = "[32m";
static COLOR_YELLOW: &str = "[33m";
static COLOR_BLUE: &str = "[34m";
static COLOR_MAGENTA: &str = "[35m";

#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => {
        hprintln!("{}{}{}", COLOR_RED, format_args!($($arg)*), COLOR_RESET)
    };
}

#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => {
        hprintln!("{}{}{}", COLOR_GREEN, format_args!($($arg)*), COLOR_RESET)
    };
}

#[macro_export]
macro_rules! print_yellow {
    ($($arg:tt)*) => {
        hprintln!("{}{}{}", COLOR_YELLOW, format_args!($($arg)*), COLOR_RESET)
    };
}

#[macro_export]
macro_rules! print_blue {
    ($($arg:tt)*) => {
        hprintln!("{}{}{}", COLOR_BLUE, format_args!($($arg)*), COLOR_RESET)
    };
}


static COLOR_RESET: &str = "[0m";


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
 
    let test_count = unsafe {
        TEST_COUNT
    };

    if test_count == 0 {
        print_blue!("Running {} tests", tests.len());
    
    }

    for i in test_count..tests.len() as i32 {
        hprint!("[{}] - ", i);
        unsafe {
            TEST_COUNT += 1;
        }
        tests[i as usize].run();
    }
    
    print_blue!("Ran {} tests", { unsafe { TEST_COUNT }});
    print_green!("Passed: {}", unsafe { TEST_PASSED });
    print_red!("Failed: {}", 0.max(test_count - unsafe { TEST_PASSED }));
    
    exit(Result::Ok(()));
}


pub fn test_panic_handler(tests: &[&dyn Testable], info: &PanicInfo) -> ! {
    hprintln!("{}{}{}", COLOR_RED, "[failed]", COLOR_RESET);
    hprintln!("{}{}{}", COLOR_RED, info, COLOR_RESET);
    test_runner(tests);
    // exit(Result::Err(()));
    loop {}
}


