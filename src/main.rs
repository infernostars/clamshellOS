#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![test_runner(clamshell::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use core::panic::PanicInfo;
use clamshell::{println, print_color, println_color, print};
use clamshell::vga_buffer::{ColorCode, Color};
use raw_cpuid::CpuId;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use clamshell::memory::{self, BootInfoFrameAllocator};
    use clamshell::allocator;
    print_color!(ColorCode::new(Color::Yellow, Color::Black), "clamshellOS ");
    println!("// version {}", env!("CARGO_PKG_VERSION"));
    println!();

    clamshell::init();

    let cpuid = CpuId::new();
    if let Some(vf) = cpuid.get_vendor_info() {
        let vfstr = vf.as_str();
        match vfstr {
            "GenuineIntel" => print_color!(ColorCode::new(Color::LightBlue, Color::Black), "Intel"),
            "AuthenticAMD" => print_color!(ColorCode::new(Color::LightRed, Color::Black), "AMD"),
            _ => print_color!(ColorCode::new(Color::LightGreen, Color::Black), "Unknown"),
        }
    }

    print!(" // {}", cpuid.get_processor_brand_string().expect("Should be on both platforms").as_str());
    println!();
    println!();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
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