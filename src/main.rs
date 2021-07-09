#![no_std]
#![no_main]

use nds::debug::NOCASH;
use nds::input::{KeyState, Keypad};
use nds::interrupts::swi_wait_for_v_blank;
use nds::sys::video;
use nds::video::api;

#[macro_use]
extern crate nds;
extern crate alloc;

const FERRIS_MAP_LEN: usize = 2048;
const FERRIS_PAL_LEN: usize = 512;
const FERRIS_TILES_LEN: usize = 14976;
extern "C" {
    #[link_name = "ferrisTiles"]
    static FERRIS_TILES: [u8; FERRIS_TILES_LEN];

    #[link_name = "ferrisMap"]
    static FERRIS_MAP: [api::TextBGMapData; FERRIS_MAP_LEN / 2];

    #[link_name = "ferrisPal"]
    static FERRIS_PAL: [u8; FERRIS_PAL_LEN];

}

// The entry point must be defined with #[entry]
// It can actually return anything that `impl Debug`, and in case it returns
// (although is not recommended), the value will be printed on the NO$GBA debug TTY
#[entry]
fn main() -> ! {
    unsafe {
        NOCASH.find_debugger();
    }
    nds::video::banks::A::Mode::MainBgSlot0.set();
    nds::video::banks::E::Mode::Lcd.set();
    let mut setting = api::create_main_mode0(1, 0);
    setting.set_visibility(api::BackgroundId::Bg0, true);
    setting.set_bg_offsets(api::BackgroundId::Bg0, 0, 0);
    unsafe {
        setting.commit();
    }

    let tiles_ptr = setting.get_tiles_ptr(api::BackgroundId::Bg0);
    let map = setting.get_map(api::BackgroundId::Bg0);

    unsafe {
        nds::dma::wait_for(nds::dma::Channel::Ch3);
        nds::dma::copy_half_words(
            nds::dma::Channel::Ch3,
            FERRIS_TILES.as_ptr() as *const u16,
            tiles_ptr,
            FERRIS_TILES_LEN / 2,
        );
    }
    unsafe {
        nds::dma::wait_for(nds::dma::Channel::Ch3);
        nds::dma::copy_half_words(
            nds::dma::Channel::Ch3,
            FERRIS_PAL.as_ptr() as *const u16,
            video::BG_PALETTE,
            FERRIS_PAL_LEN / 2,
        );
    }

    unsafe {
        nds::dma::copy(&FERRIS_MAP, map);
    }

    let mut keypad = Keypad::default();
    let mut offset: usize = 0;

    loop {
        keypad.scan();
        if keypad.left == KeyState::Down {
            offset = offset.wrapping_sub(2);
            println!("{:02X}", offset);
        } else if keypad.right == KeyState::Down {
            offset = offset.wrapping_add(2);
            println!("{:02X}", offset);
        } else if keypad.up == KeyState::Down {
            offset = offset.wrapping_sub(0x40);
            println!("{:02X}", offset);
        } else if keypad.down == KeyState::Down {
            offset = offset.wrapping_add(0x40);
            println!("{:02X}", offset);
        } else if keypad.a == KeyState::Down {
            let id = map[offset].get_tile_id().wrapping_add(1);
            map[offset].set_tile_id(id);
        } else if keypad.b == KeyState::Down {
            let id = map[offset].get_tile_id().wrapping_sub(1);
            map[offset].set_tile_id(id);
        }
        offset &= map.len();
        swi_wait_for_v_blank();
    }
}
