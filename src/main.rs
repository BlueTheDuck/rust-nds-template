#![no_std]
#![no_main]

use embedded_canvas::Canvas;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{iso_8859_10::FONT_10X20, MonoTextStyle},
    pixelcolor::{raw::LittleEndian, Bgr555},
    prelude::*,
    primitives::Rectangle,
    text::Text,
};
use nds_rs::{
    background::{BitmapLayer, MainGraphicsModeSettings, SubGraphicsModeSettings},
    interrupts::swi_wait_for_v_blank,
    nds_proc_macros::entry,
    system::Screen,
    Hw,
};
use nds_rs::{
    background::{GraphicsMode, RenderTargetBitmap},
    println,
};

extern "C" {
    pub fn vramSetPrimaryBanks(a: u32, b: u32, c: u32, d: u32) -> u32;
}

static FERRIS: ImageRaw<'static, Bgr555, LittleEndian> =
    ImageRaw::new(include_bytes!("../assets/ferris.img.bin"), 256);

// The entry point must be defined with #[entry]
// It can either return an int or loop forever
#[entry]
pub fn main(mut hw: Hw) -> ! {
    if nds_rs::debug::log_to_nocash() {
        println!("Logged to NO$GBA");
        println!("PC = %pc%");
    }

    // Enable graphic 2D engines A and B, doesn't include 3D
    hw.system.enable_engines(Some(true), Some(true));
    hw.system.main_engine_on(Screen::Top);
    unsafe {
        // Safety

        // When running in real hardware, turning on or off the LCD may damage the LCD circuitry depending on when the setting is changed.

        // Call this function only once at the start of the program.
        hw.system.enable_lcd(true);
    }
    unsafe {
        // No safe wrappers yet :(
        vramSetPrimaryBanks(1, 0, 4, 0);
    }

    let video_mode = GraphicsMode {
        layer3: BitmapLayer::new_big(),
        layer2: BitmapLayer::new_fullscreen(),
        mode_settings: MainGraphicsModeSettings::new((false, false, false, true), 0, 0),
    };
    let video_mode_sub = GraphicsMode {
        layer2: BitmapLayer::new_big(),
        layer3: BitmapLayer::new_fullscreen(),
        mode_settings: SubGraphicsModeSettings::new((false, false, false, true)),
    };
    let mut fb = video_mode.layer3_framebuffer();
    let mut fb_sub = video_mode_sub.layer3_framebuffer();
    hw.video.set_graphics_mode(video_mode);
    hw.video.set_sub_graphics_mode(video_mode_sub);

    let span = Rectangle::new(Point::zero(), fb.size());
    println!("screen size {}", fb.size());
    println!("screen size {}", fb_sub.size());

    // We can either draw on the framebuffer itself...
    swi_wait_for_v_blank();
    fb.fill_solid(&span, Bgr555::GREEN).unwrap();
    swi_wait_for_v_blank();
    fb_sub.fill_solid(&span, Bgr555::BLUE).unwrap();

    println!("Finished painting");
    let style = MonoTextStyle::new(&FONT_10X20, Bgr555::WHITE);

    // ... or on a canvas!
    let mut canvas: Canvas<<RenderTargetBitmap as DrawTarget>::Color> = Canvas::new(fb.size());
    Image::new(&FERRIS, Point::new(0, -50))
        .draw(&mut canvas)
        .unwrap();
    Text::new("hello rust!", Point::new(10, 96), style)
        .draw(&mut canvas)
        .unwrap();

    swi_wait_for_v_blank();
    canvas.place_at(Point::zero()).draw(&mut fb).unwrap();
    fb.flush_cache();
    swi_wait_for_v_blank();
    fb.flush_cache();

    loop {
        swi_wait_for_v_blank();
    }
}
