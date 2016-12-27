use std::fs::File;
use std::io::Read;
use std::vec::Vec;

#[derive(Default)]
pub struct Cartridge {
    /// Program ROM (PRG-ROM)
    pub prg_rom: Vec<u8>,

    /// Program RAM (PRG-RAM)
    pub prg_ram: Vec<u8>,

    /// Character ROM (CHR-ROM)
    pub chr_rom: Vec<u8>,

    /// iNES Mapper Number
    pub ines_mapper: u16,
}

impl Cartridge {
    pub fn open(&mut self, filename: &str) {
        // TODO: Error handling
        let mut stream = File::open(filename).unwrap();

        // Read in file header
        let mut header = vec![0u8; 0x10];
        stream.read_exact(&mut header).unwrap();

        // Validate file ID
        if !(header[0] == 0x4E && header[1] == 0x45 && header[2] == 0x53 && header[3] == 0x1A) {
            // Bad file header; not iNES
            // TODO: Support other ROM formats?
            panic!("unknown or unsupported ROM-Image format");
        }

        // Read in PRG-ROM
        self.prg_rom.clear();
        if header[4] > 0 {
            self.prg_rom.resize((header[4] as usize) * 16 * 1024, 0);
            stream.read_exact(&mut self.prg_rom).unwrap();
        }

        // Allocate PRG-RAM (A size of $0 indicates $1 because this header format is _old_)
        let prg_ram_size = (if header[8] == 0 { 8 } else { header[8] }) as usize * 8 * 1024;
        self.prg_ram.resize(prg_ram_size, 0);

        // Read in CHR-ROM
        self.chr_rom.clear();
        if header[5] > 0 {
            self.chr_rom.resize((header[5] as usize) * 8 * 1024, 0);
            stream.read_exact(&mut self.chr_rom).unwrap();
        }

        // Build iNes 1.0 mapper number
        // TODO: Flesh this out more
        self.ines_mapper = ((header[6] >> 4) | (header[7] & 0xF0)) as u16;
    }
}
