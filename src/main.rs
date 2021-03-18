#![no_std]
#![no_main]

extern crate nds;

use nds::video::{self, VramA};
use video::{colors, registers::VRAM_A, WIDTH};

static TEXT: [[u8; 20]; 5] = [
    [1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
    [1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
    [1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
    [1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0],
];

#[no_mangle]
extern "C" fn main() -> ! {
    // Display directly from VRAM on top screen
    video::set_mode(video::Mode::ModeFb0);
    // Use Bank A (aka VRAM A) as source
    VramA::set_bank_mode(VramA::BankMode::Lcd);

    let colors = [colors::BLACK, colors::RED];
    for (y, row) in TEXT.iter().enumerate() {
        for (x, &pix) in row.iter().enumerate() {
            let pixels = {
                // Letters are made of 4 pixels each "pixel"
                let top_left = x + y * WIDTH;
                let top_right = top_left + 1;
                let bot_left = top_left + WIDTH;
                let bot_right = bot_left + 1;
                [top_left, top_right, bot_left, bot_right]
            };
            for &pixel in pixels.iter() {
                unsafe {
                    VRAM_A
                        .add(pixel)
                        .write_volatile(colors[pix as usize].0);
                }
            }
        }
    }
    loop {
        nds::interrupts::swi_wait_for_v_blank();
    }
}
