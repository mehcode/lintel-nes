use cpu;
use bus;

#[derive(Default)]
pub struct Machine {
    cpu: cpu::CPU,
    bus: bus::Bus,
}

impl Machine {
    pub fn new() -> Machine {
        Default::default()
    }

    pub fn open(&mut self, filename: &str) {
        self.bus.cartridge.open(filename);
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.cpu.reset(&mut self.bus);
    }

    pub fn run(&mut self) {
        self.cpu.run_next(&mut self.bus);
    }
}
