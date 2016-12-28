use cartridge::Cartridge;
use bus;
use cpu;

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
        let prev_vblank = self.bus.ppu.vblank;

        self.cpu.run_next(&mut self.bus);

        // TODO: Figure out a better system for NMIs
        if !prev_vblank && self.bus.ppu.vblank && self.bus.ppu.nmi_enable {
            // Enter V-Blank; signal NMI
            info!("NMI");

            // Push PCH on stack; decrement S
            self.cpu.ctx.step(&mut self.bus);
            self.bus.write(0x100 + self.cpu.ctx.s as u16, (self.cpu.ctx.pc >> 8) as u8);
            self.cpu.ctx.s = self.cpu.ctx.s.wrapping_sub(1);

            // Push PCL on stack; decrement S
            self.cpu.ctx.step(&mut self.bus);
            self.bus.write(0x100 + self.cpu.ctx.s as u16, self.cpu.ctx.pc as u8);
            self.cpu.ctx.s = self.cpu.ctx.s.wrapping_sub(1);

            // Push P on stack (with BRK and UNUSED set); decrement S
            self.cpu.ctx.step(&mut self.bus);
            self.bus.write(0x100 + self.cpu.ctx.s as u16,
                           (self.cpu.ctx.p | cpu::BREAK).bits() | 0x20);
            self.cpu.ctx.s = self.cpu.ctx.s.wrapping_sub(1);

            // Fetch PCL
            self.cpu.ctx.step(&mut self.bus);
            let l = self.bus.read(0xFFFA);

            // Fetch PCH
            self.cpu.ctx.step(&mut self.bus);
            let h = self.bus.read(0xFFFB);
            self.cpu.ctx.pc = l as u16 | ((h as u16) << 8);

            // Set the IRQ Disable flag
            self.cpu.ctx.p.insert(cpu::IRQ_DISABLE);
        }
    }
}
