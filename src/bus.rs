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
        let mut value: u8 = 0xFF;
        if cpu::Controller::try_read(&mut self.mmu, address, &mut value) {
            return value;
        }

        match address {
            // PPU Registers
            0x2000...0x2007 => self.ppu.read(&mut self.mmu, address),

            // Unimplemented I/O ports
            0x4000...0x401F => 0xFF,

            _ => {
                warn!("unhandled read at ${:04X}", address);

                0xFF
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if cpu::Controller::try_write(&mut self.mmu, address, value) {
            return;
        }

        match address {
            // PPU Registers
            0x2000...0x2007 => self.ppu.write(&mut self.mmu, address, value),

            // Unimplemented I/O ports
            0x4000...0x401F => {}

            _ => {
                warn!("unhandled write at ${:04X} with ${:02X} ({})",
                      address,
                      value,
                      value);
            }
        }
    }
}
