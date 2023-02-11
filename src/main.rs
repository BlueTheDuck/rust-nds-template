#![no_std]
#![no_main]

use core::mem::size_of;
use core::mem::size_of_val;

use nds::debug::NOCASH;
use nds::input::{KeyState, Keypad};
use nds::interrupts::swi_wait_for_v_blank;
use nds::sys::video;
use nds::video::api::TextTileData;
use nds::video::{api, banks};

#[macro_use]
extern crate nds;
extern crate alloc;

// Use bindgen to generate the correct information (len, link names, etc.)
// then copy it by hand on your project
const FERRIS_TILES_LEN: usize = 14976;
const FERRIS_MAP_LEN: usize = 2048 / size_of::<TextTileData>();
const FERRIS_PAL_LEN: usize = 512;
// These will be provided by the build file
// The grit file is configured to generate a byte array with the tiles, map and palette of the image
// and the build file will compile and embed it into the binary
extern "C" {
    #[link_name = "ferris_pngTiles"]
    static FERRIS_TILES: [u8; FERRIS_TILES_LEN];

    #[link_name = "ferris_pngMap"]
    static FERRIS_MAP: [TextTileData; FERRIS_MAP_LEN];

    #[link_name = "ferris_pngPal"]
    static FERRIS_PAL: [u8; FERRIS_PAL_LEN];

}

// The entry point must be defined with #[entry]
// It can actually return anything that `impl Debug`, and in case it returns
// (although is not recommended), the value will be printed on the NO$GBA debug TTY
#[entry]
fn main() -> ! {
    // Configure banks
    unsafe {
        banks::A::Mode::MainBgSlot0.set();
        banks::E::Mode::Lcd.set();
    }

    // Initialize the video subsystem
    // We want the Main engine to use the second block (block 1)
    // of VRAM for map data, and the first (block 0) one for the tiles
    // (these blocks are 64KiB each)
    let mut setting = api::create_main_mode0(1, 0);
    // Remember to set the backgrounds to visible!!!
    setting.set_visibility(api::BackgroundId::Bg0, true);
    // The data for background 0 is at the start of the block
    setting.set_bg_offsets(api::BackgroundId::Bg0, 0, 0);
    unsafe {
        // Tell the engine to use these settings
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
            FERRIS_TILES_LEN / size_of::<u16>(), // Half-words are twice as big as an u8, so we divide by 2 the len
        );
    }
    unsafe {
        nds::dma::wait_for(nds::dma::Channel::Ch3);
        nds::dma::copy_half_words(
            nds::dma::Channel::Ch3,
            FERRIS_PAL.as_ptr() as *const u16,
            video::BG_PALETTE,
            FERRIS_PAL_LEN / size_of::<u16>(), // Half-words are twice as big as an u8, so we divide by 2 the len
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
            let id = map[offset].tile_index().wrapping_add(1);
            map[offset].set_tile_index(id);
        } else if keypad.b == KeyState::Down {
            let id = map[offset].tile_index().wrapping_sub(1);
            map[offset].set_tile_index(id);
        }
        offset &= map.len();
        swi_wait_for_v_blank();
    }
}
