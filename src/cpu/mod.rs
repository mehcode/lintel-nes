use ::bus::Bus;

#[macro_use]
mod om;
mod op;
mod operation;
mod table;

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

    fn step(&mut self, b: &mut Bus) {
        // TODO: b.step();
        self.total_cycles += 1;
        if self.total_cycles % 10000000 == 0 {
            self.print_test_output(b);
        }
    }

    fn print_test_output(&mut self, b: &mut Bus) {
        println!("-------- [instr-test v4] -----------");
        let mut i = 0;
        loop {
            let c = b.read(0x6004 + i);
            if c == 0x00 {
                break;
            }

            print!("{}", c as char);

            i += 1;
        }
        println!("\n------------------------------------");
    }
}

#[derive(Default)]
pub struct CPU {
    ctx: Context,
    table: table::Table,
}

impl CPU {
    pub fn reset(&mut self, b: &mut Bus) {
        self.ctx.reset(b);
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
            self.ctx.print_test_output(b);
            panic!(format!("unknown opcode ${:02X} at ${:04X}", opcode, _pc))
        }
    }
}
