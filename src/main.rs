use std::arch::asm;


fn main() {
    print!("Bootloader\n");

    // let mut __bootrom_start__: u32 = 0;
    // let mut __approm_start__: u32 = 0;
    // let mut _app_code:u32 = __approm_start__;
    // let c: u8 = 10;
    // let ax = u16::from(c) | 0x0e00;

    unsafe {

        asm!(
            "push bx",
            "mov bx, 0",
            "int 0x10",
            "pop bx",
            in("ax") ax
        );
        
    }

}