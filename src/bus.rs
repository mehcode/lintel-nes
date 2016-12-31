use mmu;
use cpu;
use ppu;
use apu;
use input;
use cartridge;

#[derive(Default)]
pub struct Bus {
    /// Component: PPU
    pub ppu: ppu::PPU,

    /// Component: APU
    apu: apu::APU,

    /// Component: Memory Controller
    mmu: mmu::MMU,

    /// Component: Input
    pub input: input::Input,

    /// NMI occurred (signal); set by the PPU and read by the CPU
    pub nmi_occurred: bool,
}

impl Bus {
    pub fn take_cartridge(&mut self, cartridge: cartridge::Cartridge) {
        self.mmu.take_cartridge(cartridge);
    }

    pub fn reset(&mut self) {
        self.nmi_occurred = false;

        self.ppu.reset();
        self.apu.reset();
        self.mmu.reset();
        self.input.reset();
    }

    pub fn step(&mut self) {
        self.apu.step();

        // 3 PPU Steps ("dots") to 1 CPU Step ("cycle")
        self.ppu.step(&mut self.mmu, &mut self.nmi_occurred);
        self.ppu.step(&mut self.mmu, &mut self.nmi_occurred);
        self.ppu.step(&mut self.mmu, &mut self.nmi_occurred);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let mut value: u8 = 0xFF;
        if cpu::Controller::try_read(&mut self.mmu, address, &mut value) {
            return value;
        }

        match address {
            // PPU Registers
            0x2000...0x3FFF => self.ppu.read(&mut self.mmu, address),

            // APU Registers
            0x4000...0x4013 | 0x4015 => self.apu.read(address),

            // Input
            0x4016 | 0x4017 => self.input.read(address),

            _ => {
                warn!("unhandled read at ${:04X}", address);

                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address == 0x6000 {
            warn!("write: $6000 <- ${:02X}", value);

            let mut i = 0;
            loop {
                let c = self.read(0x6004 + i);
                if c == 0x00 {
                    break;
                }

                print!("{}", c as char);

                i += 1;
            }
        }

        if cpu::Controller::try_write(&mut self.mmu, address, value) {
            return;
        }

        match address {
            // PPU Registers
            0x2000...0x3FFF => {
                self.ppu.write(&mut self.mmu, address, value);
            }

            // OAM DMA
            0x4014 => {
                // TODO: Time this right
                let mut src = (value as u16) << 8;
                let src_end = src + 0xFF;
                while src < src_end {
                    let r = self.read(src);
                    self.write(0x2004, r);

                    src += 1;
                }
            }

            // APU Registers
            0x4000...0x4013 | 0x4015 => {
                self.apu.write(address, value);
            }

            // Input
            0x4016 | 0x4017 => {
                self.input.write(address, value);
            }

            _ => {
                warn!("unhandled write at ${:04X} with ${:02X} ({})",
                      address,
                      value,
                      value);
            }
        }
    }
}
