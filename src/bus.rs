use mmu;
use cpu;
use ppu;
use cartridge;

#[derive(Default)]
pub struct Bus {
    /// Component: PPU
    ppu: ppu::PPU,

    /// Component: Memory Controller
    mmu: mmu::MMU,
}

impl Bus {
    pub fn take_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        self.mmu.take_cartridge(cartridge);
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
        self.mmu.reset();
    }

    pub fn step(&mut self) {
        // 3 PPU Steps ("dots") to 1 CPU Step ("cycle")
        self.ppu.step(&mut self.mmu);
        self.ppu.step(&mut self.mmu);
        self.ppu.step(&mut self.mmu);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        cpu::Controller::read(&mut self.mmu, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        cpu::Controller::write(&mut self.mmu, address, value);
    }
}
