use std::fs::File;
use std::io::Read;
use std::vec::Vec;

#[derive(Default)]
pub struct Cartridge {
    /// Program ROM (PRG-ROM)
    pub prg_rom: Vec<u8>,

    /// Character ROM (CHR-ROM)
    pub chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn open(&mut self, filename: &str) {
        // TODO: Error handling
        let mut stream = File::open(filename).unwrap();

        // Read and validate file ID
        let mut file_id = vec![0u8; 4];
        stream.read_exact(&mut file_id).unwrap();
        if file_id != vec![0x4E, 0x45, 0x53, 0x1A] {
            // Bad file header; not iNES
            // TODO: Support other ROM formats?
            panic!("unknown or unsupported ROM-Image format");
        }

        // Read in file header
        let mut header = vec![0u8; 0xC];
        stream.read_exact(&mut header).unwrap();

        // Read in PRG-ROM
        self.prg_rom.clear();
        if header[0] > 0 {
            self.prg_rom.resize((header[0] as usize) * 16 * 1024, 0);
            stream.read_exact(&mut self.prg_rom).unwrap();
        }

        // Read in CHR-ROM
        self.chr_rom.clear();
        if header[1] > 0 {
            self.chr_rom.resize((header[1] as usize) * 8 * 1024, 0);
            stream.read_exact(&mut self.chr_rom).unwrap();
        }

        // Build iNes 1.0 mapper number
        // TODO: Flesh this out
        let mapper = (header[2] >> 4) | (header[3] & 0xF0);
        if mapper != 0 {
            // Only NROM (0) supported right now
            panic!("unknown/unsupported mapper: {}", mapper);
        }
    }
}
