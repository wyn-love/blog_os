// main.rs

#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    use core::fmt::Write;

    vga_buffer::WRITER.lock().write_str("Hello agin").unwrap();

    write!(vga_buffer::WRITER.lock(),", some numbers: {} {}",42,1.337).unwrap();
    
    loop {}
}

