#![feature(type_ascription)]
#![feature(range_contains)]

extern crate sdl2;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate bitflags;

extern crate strfmt;

mod controller;

#[macro_use]
mod mmu;

mod bus;
mod cpu;
mod ppu;
mod apu;

mod cartridge;

mod machine;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::video::WindowBuilder;
use sdl2::render::RendererBuilder;

fn main() {
    // Log: Initialize (level set from environment variables)
    // TODO: Switch to use: https://github.com/slog-rs/slog
    env_logger::init().unwrap();

    let c = sdl2::init().unwrap();
    let mut events = c.event_pump().unwrap();
    let video = c.video().unwrap();
    let mut is_running = true;

    // Create window
    let width = ppu::WIDTH as u32;
    let height = ppu::HEIGHT as u32;
    let window = WindowBuilder::new(&video, "Lintel", width * 2, height * 2).build().unwrap();

    // Create 2D renderer
    // TODO: Do not use present_vsync and instead limit frame rate manually
    let mut renderer = RendererBuilder::new(window).accelerated().present_vsync().build().unwrap();

    // Initially clear the renderer
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    // Create texture for framebuffer
    let mut texture =
        renderer.create_texture_streaming(sdl2::pixels::PixelFormatEnum::ARGB8888, width, height)
            .unwrap();

    let mut m = machine::Machine::new();

    m.set_on_video_refresh(Box::new(move |frame| {
        renderer.clear();

        // Render: Update texture and flip
        texture.update(None, frame.data, frame.pitch).unwrap();
        renderer.copy(&texture, None, None).unwrap();

        // Render: Present
        renderer.present();
    }));

    m.open(&std::env::args().nth(1).unwrap());
    m.reset();

    while is_running {
        // Poll events
        if let Some(evt) = events.poll_event() {
            match evt {
                Event::Quit { .. } => {
                    // Quit the program
                    is_running = false;
                }

                // Event::KeyDown { scancode, repeat, .. } => {
                //     if !repeat {
                //         if let Some(scancode) = scancode {
                //             m.on_key_down(scancode);
                //         }
                //     }
                // }
                //
                // Event::KeyUp { scancode, repeat, .. } => {
                //     if !repeat {
                //         if let Some(scancode) = scancode {
                //             m.on_key_up(scancode);
                //         }
                //     }
                // }
                _ => {
                    // Unhandled event
                }
            }
        }

        // Run: Machine (for 5000 cycles)
        for _ in 1..5000 {
            m.run();
        }
    }
}
