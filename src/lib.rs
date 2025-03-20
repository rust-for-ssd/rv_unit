#![no_std]
#![no_main]

use core::panic::PanicInfo;
use semihosting::{print, println};
use semihosting::process::exit;

pub mod colors {
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const BLUE: &str = "\x1b[34m";
    pub const RESET: &str = "\x1b[0m";
}

// Generic macro to print with a given color
#[macro_export]
macro_rules! print_colored {
    ($color:expr, $($arg:tt)*) => {
        println!("{}{}{}", $color, format_args!($($arg)*), $crate::colors::RESET);
    };
}

// Specific macros for convenience
#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => {
        $crate::print_colored!($crate::colors::RED, $($arg)*);
    };
}

#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => {
        $crate::print_colored!($crate::colors::GREEN, $($arg)*);
    };
}

#[macro_export]
macro_rules! print_blue {
    ($($arg:tt)*) => {
        $crate::print_colored!($crate::colors::BLUE, $($arg)*);
    };
}

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
        println!("{}", core::any::type_name::<T>());
        self();
        unsafe {
            TEST_PASSED += 1;
        }
        print_green!("[ok]");
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
        print!("[{}] - ", i);
        unsafe {
            TEST_COUNT += 1;
        }
        tests[i as usize].run();
    }
    
    print_blue!("Ran {} tests", { unsafe { TEST_COUNT }});
    print_green!("Passed: {}", unsafe { TEST_PASSED });
    print_red!("Failed: {}", 0.max(test_count - unsafe { TEST_PASSED }));
    
    exit(0);
}


pub fn test_panic_handler(info: &PanicInfo) -> () {
    print_red!("[failed]");
    print_red!("{}", info);
}

#[export_name = "ExceptionHandler"]
fn test_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    print_red!("RISC-V Exception caught with mcause code: {}!", riscv::register::mcause::read().code());
    print_red!("Trapframe: {:?}", trap_frame);
    print_red!("Exitting!");
    exit(-1)
}

#[export_name = "DefaultHandler"]
fn test_default_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    print_red!("RISC-V Exception caught with mcause code: {}!", riscv::register::mcause::read().code());
    print_red!("Trapframe: {:?}", trap_frame);
    print_red!("Exitting!");
    exit(-1)
}
