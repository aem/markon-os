#![no_std]
#![no_main]
use core::panic::PanicInfo;

// Called on panic, global exception handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // just loop infinitely for now, no need to do anything
    // interesting here yet
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// rust mangles function names by default, can't do
// that or else linux bindings won't work, system expects _start
#[no_mangle]
// linker looks for this function name, explicitly delcare that
// we're exposing a C binding
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
