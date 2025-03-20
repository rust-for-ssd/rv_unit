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

#[macro_export]
macro_rules! println_colored {
    ($color:expr, $($arg:tt)*) => {
        println!("{}{}{}", $color, format_args!($($arg)*), $crate::colors::RESET);
    };
}

#[macro_export]
macro_rules! println_red {
    ($($arg:tt)*) => {
        $crate::println_colored!($crate::colors::RED, $($arg)*);
    };
}

#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => {
        $crate::println_colored!($crate::colors::GREEN, $($arg)*);
    };
}

#[macro_export]
macro_rules! println_blue {
    ($($arg:tt)*) => {
        $crate::println_colored!($crate::colors::BLUE, $($arg)*);
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
        println_green!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
 
    let test_count = unsafe {
        TEST_COUNT
    };

    if test_count == 0 {
        println_blue!("Running {} tests", tests.len());
    
    }

    for i in test_count..tests.len() as i32 {
        print!("[{}] - ", i);
        unsafe {
            TEST_COUNT += 1;
        }
        tests[i as usize].run();
    }
    
    println_blue!("Ran {} tests", { unsafe { TEST_COUNT }});
    println_green!("Passed: {}", unsafe { TEST_PASSED });
    println_red!("Failed: {}", 0.max(unsafe {TEST_COUNT -  TEST_PASSED }));
    
    exit(0);
}


pub fn test_panic_handler(info: &PanicInfo) -> () {
    println_red!("[failed]");
    println_red!("{}", info);
}

#[export_name = "ExceptionHandler"]
fn test_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    println_red!("RISC-V Exception caught with mcause code: {}!", riscv::register::mcause::read().code());
    println_red!("Trapframe: {:?}", trap_frame);
    println_red!("Exitting!");
    exit(-1)
}

#[export_name = "DefaultHandler"]
fn test_default_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    println_red!("RISC-V Exception caught with mcause code: {}!", riscv::register::mcause::read().code());
    println_red!("Trapframe: {:?}", trap_frame);
    println_red!("Exitting!");
    exit(-1)
}
