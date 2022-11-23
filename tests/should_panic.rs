// 集成测试  should_panic 测试

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os::{QemuExitCode,exit_qemu,serial_println,serial_print};


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    should_failed();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_failed() {
    serial_print!("should failed... ");
    assert_eq!(0,1); // 在此处发生一个 panic
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");

    exit_qemu(QemuExitCode::Success);
    loop {}
}

