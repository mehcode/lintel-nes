#![feature(type_ascription)]

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

mod cartridge;

mod machine;

fn main() {
    // Log: Initialize (level set from environment variables)
    // TODO: Switch to use: https://github.com/slog-rs/slog
    env_logger::init().unwrap();

    let mut m = machine::Machine::new();

    m.open(&std::env::args().nth(1).unwrap());
    m.reset();

    loop {
        m.run();
    }
}
