use cartridge::Cartridge;

/// Memory Management Controller (commonly known as MMC or mapper) contained within each
/// game cartridge
pub trait Controller {
    fn reset(&mut self);

    /// The mapper _attempts_ to read address (from CPU). If unmapped, return false. If mapped,
    /// insert value at `ptr` and return true.
    fn cpu_read(&mut self,
                ram: &mut [u8],
                cartridge: &mut Cartridge,
                address: u16,
                ptr: &mut u8)
                -> bool;

    /// The mapper _attempts_ to write address (to CPU). If unmapped, return false;
    /// otherwise, true.
    fn cpu_write(&mut self,
                 ram: &mut [u8],
                 cartridge: &mut Cartridge,
                 address: u16,
                 value: u8)
                 -> bool;

    /// The mapper _attempts_ to read address (from PPU). If unmapped, return false. If mapped,
    /// insert value at `ptr` and return true.
    fn ppu_read(&mut self,
                ram: &mut [u8],
                palette: &mut [u8],
                cartridge: &mut Cartridge,
                address: u16,
                ptr: &mut u8)
                -> bool;

    /// The mapper _attempts_ to write address (to PPU). If unmapped, return false;
    /// otherwise, true.
    fn ppu_write(&mut self,
                 ram: &mut [u8],
                 palette: &mut [u8],
                 cartridge: &mut Cartridge,
                 address: u16,
                 value: u8)
                 -> bool;
}


pub fn from_cartridge(cartridge: &Cartridge) -> Box<Controller> {
    match cartridge.ines_mapper {
        0 => Box::new(NROM {}),

        _ => {
            // Only NROM (0) supported right now
            panic!("unknown/unsupported iNES Mapper: {}", cartridge.ines_mapper);
        }
    }
}

// NOTE: NROM is directly implemented in this file. To support additional mappers this file
//       should be broken up into `controller/nrom.rs` and `controller/mod.rs`

#[derive(Default)]
pub struct NROM {
}

impl Controller for NROM {
    fn reset(&mut self) {}

    fn cpu_read(&mut self,
                ram: &mut [u8],
                cartridge: &mut Cartridge,
                address: u16,
                ptr: &mut u8)
                -> bool {
        *ptr = match address {
            // Internal RAM (2KiB; mirrored 3 times)
            0x0000...0x1FFF => ram[(address & 0x07FF) as usize],

            // PRG-RAM
            0x6000...0x7FFF => cartridge.prg_ram[(address - 0x6000) as usize],

            // PRG-ROM #1
            0x8000...0xBFFF => cartridge.prg_rom[(address - 0x8000) as usize],

            // PRG-ROM #2
            0xC000...0xFFFF => {
                if cartridge.prg_rom.len() <= 0x4000 {
                    // 16-KiB PRG-ROM; mirror of 0x8000...0xBFFF
                    cartridge.prg_rom[(address - 0xC000) as usize]
                } else {
                    // 32-KiB PRG-ROM
                    cartridge.prg_rom[(address - 0x8000) as usize]
                }
            }

            _ => {
                return false;
            }
        };

        true
    }

    fn cpu_write(&mut self,
                 ram: &mut [u8],
                 cartridge: &mut Cartridge,
                 address: u16,
                 value: u8)
                 -> bool {
        match address {
            // Internal RAM (2KiB; mirrored 3 times)
            0x0000...0x1FFF => {
                ram[(address & 0x07FF) as usize] = value;
            }

            // PRG-RAM
            0x6000...0x7FFF => {
                cartridge.prg_ram[(address - 0x6000) as usize] = value;
            }

            _ => {
                return false;
            }
        }

        true
    }

    fn ppu_read(&mut self,
                ram: &mut [u8],
                palette: &mut [u8],
                cartridge: &mut Cartridge,
                address: u16,
                ptr: &mut u8)
                -> bool {
        *ptr = match address {
            // CHR-ROM
            0x0000...0x1FFF => cartridge.chr_rom[address as usize],

            // Internal RAM (2KiB; mirrored once)
            // TODO: Limit size access
            0x2000...0x3EFF => ram[((address as usize) - 0x2000) & 0x0FFF],

            // Palette RAM
            0x3F00...0x3F1F => palette[((address as usize) - 0x3F00) & 0x1F],

            _ => {
                return false;
            }
        };

        true
    }

    fn ppu_write(&mut self,
                 ram: &mut [u8],
                 palette: &mut [u8],
                 _: &mut Cartridge,
                 address: u16,
                 value: u8)
                 -> bool {
        match address {
            // Internal RAM (2KiB; mirrored once)
            // TODO: Limit size access
            0x2000...0x3EFF => {
                ram[((address as usize) - 0x2000) & 0x0FFF] = value;
            }

            // Palette RAM
            0x3F00...0x3F1F => {
                palette[((address as usize) - 0x3F00) & 0x1F] = value;
            }

            _ => {
                return false;
            }
        }

        true
    }
}
