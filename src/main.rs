#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod keyboard;
mod serial;

use core::panic::PanicInfo;
use keyboard::Keyboard;

fn sleep() {
    for _ in 0..10000 {
        core::hint::spin_loop();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("    ____             __     ____  _____");
    println!("   / __ \\__  _______/ /_   / __ \\/ ___/");
    println!("  / /_/ / / / / ___/ __/  / / / /\\__ \\ ");
    println!(" / _, _/ /_/ (__  ) /_   / /_/ /___/ / ");
    println!("/_/ |_|\\__,_/____/\\__/   \\____//____/  ");
    println!("                                       ");
    println!("Type something (press Enter to submit):\n");
    
    let mut keyboard = Keyboard::new();
    print!("> ");

    #[cfg(test)]
    test_main();
    
    loop {
        let scancode = keyboard.read_scancode();
        keyboard.handle_scancode(scancode);
        sleep();
    }
}

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    serial_println!("[ok] All tests passed!");
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#[test_case]
fn failed_assertion() {
    serial_println!("failed assertion... ");
    assert_eq!(1, 2);
    serial_println!("[failed]");
}