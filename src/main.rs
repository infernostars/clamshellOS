#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_operator::test_runner)]
#![reexport_test_harness_main = "test_main"]

use the_operator::{print, println};
use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("theOperator // version {}", env!("CARGO_PKG_VERSION"));
    println!();

    the_operator::init();


    #[cfg(test)]
    test_main();

    println!("no crash!");
    loop {
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    the_operator::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}