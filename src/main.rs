#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
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

    rust_os::init();
    x86_64::instructions::interrupts::int3();
    
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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

