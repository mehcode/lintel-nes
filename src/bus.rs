use std::vec::Vec;
use cartridge;

#[derive(Default)]
pub struct Bus {
    /// RAM (2 KiB)
    ram: Vec<u8>,

    /// Component: Cartridge
    pub cartridge: cartridge::Cartridge,
}

impl Bus {
    pub fn reset(&mut self) {
        self.ram.clear();
        self.ram.resize(2 * 1024, 0);
    }

    /// Memory: Read
    pub fn read(&self, address: u16) -> u8 {
        // TODO: Mappers are taking 0 consideration right now
        match address {
            // Internal RAM (2KiB; mirrored 3 times)
            0x0000...0x1FFF => self.ram[(address & 0x07FF) as usize],

            // [NROM] PRG-ROM #1
            0x8000...0xBFFF => self.cartridge.prg_rom[(address - 0x8000) as usize],

            // [NROM] PRG-ROM #2
            0xC000...0xFFFF => {
                if self.cartridge.prg_rom.len() <= 0x4000 {
                    // 16-KiB PRG-ROM; mirror of 0x8000...0xBFFF
                    self.cartridge.prg_rom[(address - 0xC000) as usize]
                } else {
                    // 32-KiB PRG-ROM
                    self.cartridge.prg_rom[(address - 0x8000) as usize]
                }
            }

            _ => 0xFF,
        }
    }

    /// Memory: Write
    pub fn write(&mut self, address: u16, value: u8) {
        // TODO: Mappers are taking 0 consideration right now
        match address {
            // Internal RAM (2KiB; mirrored 3 times)
            0x0000...0x1FFF => {
                self.ram[(address & 0x07FF) as usize] = value;
            }

            // [NROM] PRG-ROM #1
            0x8000...0xBFFF => {
                self.cartridge.prg_rom[(address - 0x8000) as usize] = value;
            }

            // [NROM] PRG-ROM #2
            0xC000...0xFFFF => {
                if self.cartridge.prg_rom.len() <= 0x4000 {
                    // 16-KiB PRG-ROM; mirror of 0x8000...0xBFFF
                    self.cartridge.prg_rom[(address - 0xC000) as usize] = value;
                } else {
                    // 32-KiB PRG-ROM
                    self.cartridge.prg_rom[(address - 0x8000) as usize] = value;
                }
            }

            _ => {}
        }
    }
}
