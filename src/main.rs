
#[macro_use]
extern crate bitflags;

extern crate strfmt;

mod bus;
mod cpu;
mod cartridge;
mod machine;

fn main() {
    let mut m = machine::Machine::new();

    m.open("01-basics.nes");
    m.reset();

    loop {
        m.run();
    }
}
