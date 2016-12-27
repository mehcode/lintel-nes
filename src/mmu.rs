use cpu;
use ppu;
use cartridge::Cartridge;
use controller;

/// Used to generate sub-controller trait definitions in the CPU and PPU.
macro_rules! make_controller {() => {
    pub trait Controller {
        /// The controller _attempts_ to read address. If unmapped, return false. If mapped,
        /// insert value at `ptr` and return true.
        fn try_read(&mut self, address: u16, ptr: &mut u8) -> bool;

        fn read(&mut self, address: u16) -> u8 {
            let mut value: u8 = 0xFF;
            if self.try_read(address, &mut value) {
                return value;
            }

            warn!("unhandled read at ${:04X}", address);

            value
        }

        /// The controller _attempts_ to write address. If unmapped, return false; otherwise, true.
        fn try_write(&mut self, address: u16, value: u8) -> bool;

        fn write(&mut self, address: u16, value: u8) {
            if self.try_write(address, value) {
                return;
            }

            warn!("unhandled write at ${:04X} with ${:02X} ({})", address, value, value);
        }
    }
};}

/// Memory Management Unit is responsible for coordinating the active memory controller.
#[derive(Default)]
pub struct MMU {
    // Cartridge (and associated CHR-R*M and PRG-R*M)
    cartridge: Cartridge,

    // Reference to the active memory controller
    controller: Option<Box<controller::Controller>>,

    // [CPU] Internal RAM ~ 2 KiB
    cpu_ram: Vec<u8>,

    // [PPU] Internal RAM ~ 2 KiB
    ppu_ram: Vec<u8>,
}

impl MMU {
    pub fn take_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
        self.controller = Some(controller::from_cartridge(&self.cartridge));
    }

    pub fn reset(&mut self) {
        // Reset: RAM
        self.cpu_ram.clear();
        self.ppu_ram.clear();
        self.cpu_ram.resize(1024 * 2, 0);
        self.ppu_ram.resize(1024 * 2, 0);

        // TODO? self.cartridge.reset()

        // Reset: Controller
        if let Some(ref mut controller) = self.controller {
            controller.reset();
        }
    }
}

impl cpu::Controller for MMU {
    fn try_read(&mut self, address: u16, ptr: &mut u8) -> bool {
        if let Some(ref mut controller) = self.controller {
            if controller.cpu_read(&mut self.cpu_ram, &mut self.cartridge, address, ptr) {
                return true;
            }
        }

        false
    }

    fn try_write(&mut self, address: u16, value: u8) -> bool {
        if let Some(ref mut controller) = self.controller {
            if controller.cpu_write(&mut self.cpu_ram, &mut self.cartridge, address, value) {
                return true;
            }
        }

        false
    }
}

impl ppu::Controller for MMU {
    fn try_read(&mut self, address: u16, ptr: &mut u8) -> bool {
        if let Some(ref mut controller) = self.controller {
            if controller.ppu_read(&mut self.ppu_ram, &mut self.cartridge, address, ptr) {
                return true;
            }
        }

        false
    }

    fn try_write(&mut self, address: u16, value: u8) -> bool {
        if let Some(ref mut controller) = self.controller {
            if controller.ppu_write(&mut self.ppu_ram, &mut self.cartridge, address, value) {
                return true;
            }
        }

        false
    }
}
