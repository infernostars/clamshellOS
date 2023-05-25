#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_operator::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use clamshell::{println, print_color, println_color, print};
use clamshell::vga_buffer::{ColorCode, Color};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_color!(ColorCode::new(Color::Yellow, Color::Black), "clamshellOS ");
    println!("// version {}", env!("CARGO_PKG_VERSION"));
    println!();

    clamshell::init();

    #[cfg(test)]
    test_main();

    print!("started successfully\n\n>");
    clamshell::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_color!(ColorCode::new(Color::LightRed, Color::Black), "{}\n", info);
    print!("Stopping...");
    clamshell::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    clamshell::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}