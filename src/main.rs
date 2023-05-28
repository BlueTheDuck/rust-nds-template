#![no_std]
#![no_main]

use core::slice::from_raw_parts_mut;

use nds::background::Layer;
use nds::dma::{self, Channel};
use nds::interrupts::swi_wait_for_v_blank;
use nds::sys::video::{enable_main_background, BG_GFX};
use nds::video::background::{background_init, BitmapLayerSize, DirectBitmapLayer, Mode};
use nds::video::banks;

#[macro_use]
extern crate nds;
extern crate alloc;

static FERRIS_IMAGE: &'static [u16] = &include_bytes_as!(u16, "../assets/ferris.img.bin");

// The entry point must be defined with #[entry]
// It can actually return anything that `impl Debug`, and in case it returns
// (although is not recommended), the value will be printed on the NO$GBA debug TTY
#[entry]
fn main() -> ! {
    // Configure banks
    unsafe {
        banks::A::Mode::MainBgSlot0.set();
        banks::B::Mode::MainBgSlot1.set();
    }

    println!("Banks configured");

    background_init(Mode::Mode5 {
        layer0: None,
        layer1: None,
        layer2: None,
        layer3: Some(DirectBitmapLayer::new(BitmapLayerSize::Medium, 0).unwrap()),
    });

    println!("Backgrounds configured");

    let bitmap: &'static mut [u16] = unsafe { from_raw_parts_mut(BG_GFX, 256 * 256) };

    println!("Filling with white");
    dma::fill(0b1_11111_11111_11111u16, bitmap);
    dma::wait_for(Channel::Ch3);
    println!("Writing bitmap");
    dma::copy(FERRIS_IMAGE, bitmap);
    dma::wait_for(Channel::Ch3);
    println!("Bitmap written");

    enable_main_background(Layer::Layer3, true);

    loop {
        swi_wait_for_v_blank();
    }
}
