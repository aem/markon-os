#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod serial;
mod vga_buffer;

// Called on panic, global exception handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]");
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
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
        port.write(exit_code as u32)
    }
}

#[cfg(not(test))]
fn print_sample_vga_text() {
    for i in 0x21..0x7e {
        print!("{}", i as u8 as char);
    }
    println!("The numbers are {} and {}", 41, 1.0 / 3.0);
    panic!("ah shit it's all borked now innit");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} test(s)", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion...");
    assert_eq!(1, 1);
    serial_println!("[OK]");
}
