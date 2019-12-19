#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

// Called on panic, global exception handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn print_sample_vga_text() {
    for i in 0x21..0x7e {
        print!("{}", i as u8 as char);
    }
    println!("The numbers are {} and {}", 41, 1.0 / 3.0);
    panic!("ah shit it's all borked now innit");
}

// rust mangles function names by default, can't do
// that or else linux bindings won't work, system expects _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // linker looks for this function name, explicitly delcare that
    // we're exposing a C binding

    #[cfg(not(test))]
    print_sample_vga_text();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} test(s)", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion...");
    assert_eq!(1, 1);
    print!("[OK]")
}
