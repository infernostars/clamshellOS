#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_operator::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use clamshell::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("clamshellOS // version {}", env!("CARGO_PKG_VERSION"));
    println!();

    clamshell::init();


    #[cfg(test)]
    test_main();

    println!("started successfully");
    clamshell::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) {
    println!("{}", info);
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