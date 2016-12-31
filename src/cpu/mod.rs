#[macro_use]
mod om;
mod op;
mod operation;
mod table;
use bus::Bus;

// Generate memory Controller trait for CPU
make_controller!();

bitflags!(
    #[derive(Default)]
    pub flags Flags: u8 {
        const CARRY         = 0b_0000_0001, // C
        const ZERO          = 0b_0000_0010, // Z
        const IRQ_DISABLE   = 0b_0000_0100, // I
        const DECIMAL_MODE  = 0b_0000_1000, // D — Not used by any instruction on NES
        const BREAK         = 0b_0001_0000, // B — Normally unaccessible except when pushed during /NMI or /BRK

        const OVERFLOW      = 0b_0100_0000, // V
        const SIGN          = 0b_1000_0000, // N
    }
);

impl Flags {
    // https://github.com/rust-lang-nursery/bitflags/pull/55
    fn set(&mut self, flags: Flags, value: bool) {
        if value {
            self.insert(flags);
        } else {
            self.remove(flags);
        }
    }
}

/// CPU Registers and other misc. contextual information required by operations
#[derive(Default)]
pub struct Context {
    /// Accumulator — A
    pub a: u8,

    // Index — X
    pub x: u8,

    // Index — Y
    pub y: u8,

    // Program Counter — PC
    pub pc: u16,

    // Stack Pointer — S
    pub s: u8,

    // Processor Status — P
    pub p: Flags,

    // Running clock cycle counter (used for reference in debug)
    total_cycles: u32,
}

impl Context {
    fn reset(&mut self, b: &mut Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.s = 0xFD;
        self.p.bits = 0x34;

        // Get /RESET address
        // TODO: Should have helper methods in om to get vectors/16-bit data
        self.pc = b.read(0xFFFC) as u16 | ((b.read(0xFFFD) as u16) << 8);
        // self.pc = 0xC000;
    }

    pub fn step(&mut self, b: &mut Bus) {
        b.step();
        self.total_cycles += 1;
    }
}

#[derive(Default)]
pub struct CPU {
    pub ctx: Context,
    table: table::Table,
    nmi_pending: bool,
}

impl CPU {
    pub fn reset(&mut self, b: &mut Bus) {
        self.ctx.reset(b);
        self.nmi_pending = false;
    }

    /// Run Next Instruction
    pub fn run_next(&mut self, b: &mut Bus) {
        // Fetch opcode
        let _pc = self.ctx.pc;
        self.ctx.step(b);
        let opcode = b.read(self.ctx.pc);
        self.ctx.pc = self.ctx.pc.wrapping_add(1);

        // Decode operation
        let op = &self.table[opcode];
        if let Some(handle) = op.handle {
            // Trace: Operation
            trace!("{:>10}: {:<25} PC: ${:04X} A: ${:02X} X: ${:02X} Y: ${:02X} S: ${:02X} P: ${:02X}",
                     self.ctx.total_cycles,
                     op.format(&self.ctx, b).unwrap(),
                     _pc,
                     self.ctx.a,
                     self.ctx.x,
                     self.ctx.y,
                     self.ctx.s,
                     self.ctx.p.bits);

            // The 65xx _always_ reads at least 2 bytes per instructions.
            if op.size <= 1 {
                self.ctx.step(b);
                b.read(self.ctx.pc);
            }

            // Execute: Operation
            (handle)(&mut self.ctx, b);
        } else {
            panic!(format!("unknown opcode ${:02X} at ${:04X}", opcode, _pc))
        }

        // Check for pending NMI IRQ
        if b.nmi_occurred {
            // Push PCH on stack; decrement S
            self.ctx.step(b);
            b.write(0x100 + self.ctx.s as u16, (self.ctx.pc >> 8) as u8);
            self.ctx.s = self.ctx.s.wrapping_sub(1);

            // Push PCL on stack; decrement S
            self.ctx.step(b);
            b.write(0x100 + self.ctx.s as u16, self.ctx.pc as u8);
            self.ctx.s = self.ctx.s.wrapping_sub(1);

            // Push P on stack (with BRK and UNUSED set); decrement S
            self.ctx.step(b);
            b.write(0x100 + self.ctx.s as u16,
                    (self.ctx.p | BREAK).bits() | 0x20);
            self.ctx.s = self.ctx.s.wrapping_sub(1);

            // Fetch PCL
            self.ctx.step(b);
            let l = b.read(0xFFFA);

            // Fetch PCH
            self.ctx.step(b);
            let h = b.read(0xFFFB);
            self.ctx.pc = l as u16 | ((h as u16) << 8);

            // Set the IRQ Disable flag
            self.ctx.p.insert(IRQ_DISABLE);

            b.nmi_occurred = false;
        }

        // Check for NMI; schedule IRQ after this next instruction
        if b.nmi_occurred {
            self.nmi_pending = true;
            b.nmi_occurred = false;
        }
    }
}
