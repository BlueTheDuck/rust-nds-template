#![no_std]
#![no_main]

extern crate nds;

use nds::video::{self,VramA};

#[no_mangle]
extern "C" fn main(argc: isize, argv: *const *const u8) -> ! {
    video::set_modevideo::Mode(ModeFb0);
    VramA::set_bank_mode(VramA::BankMode::Lcd);
    loop {
        nds::interrupts::swi_wait_for_v_blank();
    }
}
