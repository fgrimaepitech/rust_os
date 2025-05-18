#![no_std] // ne pas lier la bibliothèque standard Rust
#![no_main] // désactiver tous les points d'entrée au niveau de Rust
#![feature(abi_x86_interrupt)]

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
    println!("Welcome to Rust OS!");
    println!("Type something (press Enter to submit):\n");
    
    let mut keyboard = Keyboard::new();
    print!("> ");
    
    loop {
        let scancode = keyboard.read_scancode();
        keyboard.handle_scancode(scancode);
        sleep();
    }
}

/// Cette fonction est appelée à chaque panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_err!("{}", _info);
    loop {}
}