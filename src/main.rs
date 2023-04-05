use roma::avr::arduino::tools::Math;

use roma::print::printable::{log_};

fn main() {
    print!("Bootloader\n");
    let _absolute_:i32 = Math::abs(2);
    log_("This is a working log.", "Warning");
    log_("This is a working log.", "Info");

    // unsafe {

    //     asm!(
    //         "push bx",
    //         "mov bx, 0",
    //         "int 0x10",
    //         "pop bx",
    //         in("ax") ax
    //     );
        
    // }

}