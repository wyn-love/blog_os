// main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // 将测试环境，生成的 main 函数（因为 #![no_main] 忽略了 main 函数）更名为 test_main

use core::panic::PanicInfo;
use blog_os::println;


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    // 打印 panic 信息
    println!("{}",_info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info)
}

#[cfg(predicate)]
static HELLO: &[u8] = b"Hello world!";


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello agin").unwrap();
    // write!(vga_buffer::WRITER.lock(),", some numbers: {} {}",42,1.337).unwrap();
    // use println!

    println!("Hello world{}","!");

    // 当环境为 test 时，调用 test_main() 函数
    #[cfg(test)]
    test_main();

    // 手动触发 panic
    // panic!("Something make panic");
    loop {}
}

