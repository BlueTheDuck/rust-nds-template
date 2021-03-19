#![no_std]
#![no_main]

#[macro_use]
extern crate nds;

use nds::{
    input::{KeyState, Keypad},
    video::{self, VramA},
};
use video::{colors, registers::VRAM_A, set_backdrop_color, WIDTH};

static TEXT: [[u8; 50]; 5] = [
    [
        1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0,
        1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1,
    ],
    [
        1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0,
        1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0,
    ],
    [
        1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0,
        1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0,
    ],
    [
        1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0,
        1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
    ],
    [
        1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0,
        1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0,
    ],
];

// The entry point must be defined with #[entry]
// It can actually return anything that `impl Debug`, and in case it returns
// (although is not recommended), the value will be printed on the NO$GBA debug TTY
#[entry]
fn main() -> ! {
    // Display directly from VRAM on top screen
    video::set_mode(video::Mode::ModeFb0);
    // Use Bank A (aka VRAM A) as source, mapping the first 192*256*2 bytes of Bank A to pixels on screen
    VramA::set_bank_mode(VramA::BankMode::Lcd);

    let colors = [colors::RED, colors::GREEN, colors::BLUE];
    let mut color_idx = 0;
    let mut keypad = Keypad::default();
    loop {
        for (y, row) in TEXT.iter().enumerate() {
            for (x, &pix) in row.iter().enumerate() {
                if pix == 0 {
                    continue;
                }
                let pixels = {
                    // Letters are made of 4 pixels each "pixel"
                    let top_left = x * 2 + y * 2 * WIDTH;
                    let top_right = top_left + 1;
                    let bot_left = top_left + WIDTH;
                    let bot_right = bot_left + 1;
                    [top_left, top_right, bot_left, bot_right]
                };
                for &pixel in pixels.iter() {
                    unsafe {
                        VRAM_A.add(pixel).write_volatile(colors[color_idx].0);
                    }
                }
            }
        }
        // Ask the ARM7 for the state of the keys
        keypad.scan();
        if keypad.up == KeyState::Down {
            color_idx += 1;
            color_idx %= colors.len();
        } else if keypad.down == KeyState::Down {
            color_idx = core::cmp::min(color_idx.wrapping_sub(1), colors.len() - 1);
        }
        // Wait for the next vertical blank
        nds::interrupts::swi_wait_for_v_blank();
    }
}
