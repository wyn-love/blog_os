use crate::{print, println,serial_print,serial_println};


#[test_case]
fn trivial_assertion() {
    print!("trival assertion... ");
    assert_eq!(1,1);
    println!("[ok]");
}

#[test_case]
fn trivial_assertion_1() {
    serial_print!("trival assertion... ");
    
    // loop {} // 测试超时
    assert_eq!(1,1);
    serial_println!("[ok]");
}