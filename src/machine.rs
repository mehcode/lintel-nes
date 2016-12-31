use cartridge::Cartridge;
use bus;
use cpu;
use ppu;

#[derive(Default)]
pub struct Machine {
    /// Interconnect/Bus
    bus: bus::Bus,

    /// CPU
    cpu: cpu::CPU,
}

impl Machine {
    pub fn new() -> Machine {
        Default::default()
    }

    pub fn set_on_video_refresh(&mut self, callback: Box<FnMut(ppu::Frame) -> ()>) {
        self.bus.ppu.set_on_refresh(callback);
    }

    pub fn open(&mut self, filename: &str) {
        // TODO: Cleanup with `Cartridge::with_rom(...)`
        let mut cartridge: Cartridge = Default::default();
        cartridge.open(filename);

        // Give cartridge to Bus
        self.bus.take_cartridge(cartridge);
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.cpu.reset(&mut self.bus);
    }

    pub fn run(&mut self) {
        self.cpu.run_next(&mut self.bus);
    }
}
