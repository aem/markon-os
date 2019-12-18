#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

// Called on panic, global exception handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// rust mangles function names by default, can't do
// that or else linux bindings won't work, system expects _start
#[no_mangle]
// linker looks for this function name, explicitly delcare that
// we're exposing a C binding
pub extern "C" fn _start() -> ! {
    for i in 0x21..0x7e {
        print!("{}", i as u8 as char);
    }
    println!("The numbers are {} and {}", 41, 1.0 / 3.0);
    panic!("ah shit it's all borked now innit");
}
