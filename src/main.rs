#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod keyboard;

use core::panic::PanicInfo;
use keyboard::Keyboard;

fn sleep() {
    for _ in 0..10000 {
        core::hint::spin_loop();
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_err!("{}", _info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}