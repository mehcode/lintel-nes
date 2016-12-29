use bus::Bus;
use super::Context;
use super::Flags;
use super::operation::AddressingMode;
pub use super::operation::AddressingMode::*;

// TODO: Refactor out the effective address determination (it turns out its _almost_ identical for R/W/M)
// TODO: Move read/write/modify to methods on `cpu::Context`

/// Read
pub fn read(c: &mut Context, b: &mut Bus, mode: AddressingMode) -> u8 {
    // Determine _effective_ address
    let address = match mode {
        Immediate => {
            // Immediate addressing fetches value from PC; so just make address = PC
            let address = c.pc;
            c.pc = c.pc.wrapping_add(1);

            address
        }

        ZeroPage => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            address as u16
        }

        ZeroPageX => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add X
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.x) as u16
        }

        ZeroPageY => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add Y
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.y) as u16
        }

        Absolute => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut address = b.read(c.pc) as u16;
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; increment PC
            c.step(b);
            address |= (b.read(c.pc) as u16) << 8;
            c.pc = c.pc.wrapping_add(1);

            address
        }

        AbsoluteX => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add X to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.x as u16) >= 0x100;
            l = l.wrapping_add(c.x);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            if overflow_pg {
                // Read from effective address; fix high byte of effective address
                c.step(b);
                b.read(address);
                address = address.wrapping_add(0x100);
            }

            address
        }

        AbsoluteY => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add Y to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            if overflow_pg {
                // Read from effective address; fix high byte of effective address
                c.step(b);
                b.read(address);
                address = address.wrapping_add(0x100);
            }

            address
        }

        IndexedIndirect => {
            // Fetch pointer address; increment PC
            c.step(b);
            let mut ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from pointer address; add X to pointer
            c.step(b);
            b.read(ptr as u16);
            ptr = ptr.wrapping_add(c.x);

            // Fetch low byte of effective address
            c.step(b);
            let l = b.read(ptr as u16);

            // Fetch high byte of effective address
            c.step(b);
            let h = b.read(ptr.wrapping_add(1) as u16);

            l as u16 | ((h as u16) << 8)
        }

        IndirectIndexed => {
            // Fetch pointer address; increment PC
            c.step(b);
            let ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch low byte of effective address
            c.step(b);
            let mut l = b.read(ptr as u16);

            // Fetch high byte of effective address; add Y to effective address
            // TODO: Cleaner method of doing the page overflow
            c.step(b);
            let h = b.read((ptr.wrapping_add(1)) as u16);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);

            if overflow_pg {
                // Read from effective address; fix high byte of effective address
                c.step(b);
                b.read(address);
                address = address.wrapping_add(0x100);
            }

            address
        }
    };

    // Read from effective address
    c.step(b);
    b.read(address)
}

/// Write
pub fn write(c: &mut Context, b: &mut Bus, mode: AddressingMode, r: u8) {
    // Determine _effective_ address
    let address = match mode {
        ZeroPage => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            address as u16
        }

        ZeroPageX => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add X
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.x) as u16
        }

        ZeroPageY => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add Y
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.y) as u16
        }

        Absolute => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut address = b.read(c.pc) as u16;
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; increment PC
            c.step(b);
            address |= (b.read(c.pc) as u16) << 8;
            c.pc = c.pc.wrapping_add(1);

            address
        }

        AbsoluteX => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add X to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.x as u16) >= 0x100;
            l = l.wrapping_add(c.x);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        AbsoluteY => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add Y to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        IndexedIndirect => {
            // Fetch pointer address; increment PC
            c.step(b);
            let mut ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from pointer address; add X to pointer
            c.step(b);
            b.read(ptr as u16);
            ptr = ptr.wrapping_add(c.x);

            // Fetch low byte of effective address
            c.step(b);
            let l = b.read(ptr as u16);

            // Fetch high byte of effective address
            c.step(b);
            let h = b.read(ptr.wrapping_add(1) as u16);

            l as u16 | ((h as u16) << 8)
        }

        IndirectIndexed => {
            // Fetch pointer address; increment PC
            c.step(b);
            let ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch low byte of effective address
            c.step(b);
            let mut l = b.read(ptr as u16);

            // Fetch high byte of effective address; add Y to effective address
            // TODO: Cleaner method of doing the page overflow
            c.step(b);
            let h = b.read((ptr.wrapping_add(1)) as u16);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        _ => {
            panic!("unsupported addressing mode for Write operation: {:?}",
                   mode);
        }
    };

    // Write to effective address
    c.step(b);
    b.write(address, r);
}

/// Read-Modify-Write
pub fn modify<F: Fn(&mut Context, &mut Bus, u8) -> u8>(c: &mut Context,
                                                       b: &mut Bus,
                                                       mode: AddressingMode,
                                                       cb: F) {
    // Determine _effective_ address
    let address = match mode {
        ZeroPage => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            address as u16
        }

        ZeroPageX => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add X
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.x) as u16
        }

        ZeroPageY => {
            // Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from address; add Y
            c.step(b);
            b.read(address as u16);

            address.wrapping_add(c.y) as u16
        }

        Absolute => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut address = b.read(c.pc) as u16;
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; increment PC
            c.step(b);
            address |= (b.read(c.pc) as u16) << 8;
            c.pc = c.pc.wrapping_add(1);

            address as u16
        }

        AbsoluteX => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add X to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.x as u16) >= 0x100;
            l = l.wrapping_add(c.x);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        AbsoluteY => {
            // Fetch low byte of address; increment PC
            c.step(b);
            let mut l = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch high byte of address; add Y to low address byte; increment PC
            c.step(b);
            let h = b.read(c.pc);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);
            c.pc = c.pc.wrapping_add(1);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        IndexedIndirect => {
            // Fetch pointer address; increment PC
            c.step(b);
            let mut ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Read from pointer address; add X to pointer
            c.step(b);
            b.read(ptr as u16);
            ptr = ptr.wrapping_add(c.x);

            // Fetch low byte of effective address
            c.step(b);
            let l = b.read(ptr as u16);

            // Fetch high byte of effective address
            c.step(b);
            let h = b.read(ptr.wrapping_add(1) as u16);

            l as u16 | ((h as u16) << 8)
        }

        IndirectIndexed => {
            // Fetch pointer address; increment PC
            c.step(b);
            let ptr = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            // Fetch low byte of effective address
            c.step(b);
            let mut l = b.read(ptr as u16);

            // Fetch high byte of effective address; add Y to effective address
            // TODO: Cleaner method of doing the page overflow
            c.step(b);
            let h = b.read((ptr.wrapping_add(1)) as u16);
            let overflow_pg = (l as u16 + c.y as u16) >= 0x100;
            l = l.wrapping_add(c.y);
            let mut address = l as u16 | ((h as u16) << 8);

            // Read from effective address; fix high byte of effective address
            c.step(b);
            b.read(address);
            if overflow_pg {
                address = address.wrapping_add(0x100);
            }

            address
        }

        _ => {
            panic!("unsupported addressing mode for Read-Modify-Write operation: {:?}",
                   mode);
        }
    };

    // Read from effective address
    c.step(b);
    let mut value = b.read(address);

    // Write the (same) value back to effective address [..]
    c.step(b);
    b.write(address, value);

    // [..] and do the operation on it
    value = cb(c, b, value);

    // Write the new value to effective address
    c.step(b);
    b.write(address, value);
}

// TODO: Read/Write/Modify opeations likely belong on cpu::Context
// -------------–-------------–-------------–-------------–-------------–-------------–------------

macro_rules! om_load (($c:expr, $b:expr; $addr_mode:expr; $r:ident) => {
    {
        $c.$r = om::read($c, $b, $addr_mode);

        $c.p.set(cpu::ZERO, $c.$r == 0);
        $c.p.set(cpu::SIGN, $c.$r & 0x80 != 0);
    }
});

macro_rules! om_store (($c:expr, $b:expr; $addr_mode:expr; $r:ident) => {
    {
        let r = $c.$r;
        om::write($c, $b, $addr_mode, r);
    }
});

macro_rules! om_compare (($c:expr, $b:expr; $addr_mode:expr; $r:ident) => {
    {
        let value = $c.$r;
        let operand = om::read($c, $b, $addr_mode);
        let r = value.wrapping_sub(operand);

        $c.p.set(cpu::CARRY, value >= operand);
        $c.p.set(cpu::ZERO, value == operand);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);
    }
});

macro_rules! om_adc (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        let value = $c.a as u16;
        let operand = om::read($c, $b, $addr_mode) as u16;
        let c = if $c.p.contains(cpu::CARRY) { 1 } else { 0 };
        let r = value.wrapping_add(operand).wrapping_add(c);

        $c.p.set(cpu::ZERO, (r & 0xFF) == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);
        $c.p.set(cpu::CARRY, ((r >> 8) & 1) != 0);
        $c.p.set(cpu::OVERFLOW, ((value ^ r) & (operand ^ r) & 0x80) != 0);

        $c.a = r as u8;
    }
});

macro_rules! om_sbc (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        let value = $c.a as u16;
        let operand = (om::read($c, $b, $addr_mode) as u16) ^ 0xFF;
        let c = if $c.p.contains(cpu::CARRY) { 1 } else { 0 };
        let r = value.wrapping_add(operand).wrapping_add(c);

        $c.p.set(cpu::ZERO, (r & 0xFF) == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);
        $c.p.set(cpu::CARRY, ((r >> 8) & 1) != 0);
        $c.p.set(cpu::OVERFLOW, ((value ^ r) & (operand ^ r) & 0x80) != 0);

        $c.a = r as u8;
    }
});

macro_rules! om_ora (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        let value = $c.a as u16;
        let operand = om::read($c, $b, $addr_mode) as u16;
        let r = value | operand;

        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        $c.a = r as u8;
    }
});

macro_rules! om_inc (($c:expr, $b:expr; $addr_mode:expr) => {
    om::modify($c, $b, $addr_mode, |c, _, value| {
        let r = value.wrapping_add(1);

        c.p.set(cpu::ZERO, r == 0);
        c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    });
});

macro_rules! om_dec (($c:expr, $b:expr; $addr_mode:expr) => {
    om::modify($c, $b, $addr_mode, |c, _, value| {
        let r = value.wrapping_sub(1);

        c.p.set(cpu::ZERO, r == 0);
        c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    });
});

macro_rules! om_eor (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        let value = $c.a;
        let operand = om::read($c, $b, $addr_mode);
        let r = value ^ operand;

        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        $c.a = r;
    }
});

macro_rules! om_and (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        let value = $c.a;
        let operand = om::read($c, $b, $addr_mode);
        let r = value & operand;

        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        $c.a = r;
    }
});

macro_rules! om_asl (($c:expr; $operand:expr) => {
    {
        let operand = $operand;
        let r = operand << 1;

        $c.p.set(cpu::CARRY, operand & 0x80 != 0);
        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    }
});

macro_rules! om_asl_a (($c:expr) => {
    {
        let operand = $c.a;
        $c.a = om_asl!($c; operand);
    }
});

macro_rules! om_asl_m (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        om::modify($c, $b, $addr_mode, |c, _, operand| {
            om_asl!(c; operand)
        });
    }
});

macro_rules! om_lsr (($c:expr; $operand:expr) => {
    {
        let operand = $operand;
        let r = operand >> 1;

        $c.p.set(cpu::CARRY, operand & 0x1 != 0);
        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    }
});

macro_rules! om_lsr_a (($c:expr) => {
    {
        let operand = $c.a;
        $c.a = om_lsr!($c; operand);
    }
});

macro_rules! om_lsr_m (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        om::modify($c, $b, $addr_mode, |c, _, operand| {
            om_lsr!(c; operand)
        });
    }
});

macro_rules! om_ror (($c:expr; $operand:expr) => {
    {
        let operand = $operand;
        let c = if $c.p.contains(cpu::CARRY) { 1 } else { 0 };
        let r = (operand >> 1) | (c << 7);

        $c.p.set(cpu::CARRY, operand & 0x1 != 0);
        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    }
});

macro_rules! om_ror_a (($c:expr) => {
    {
        let operand = $c.a;
        $c.a = om_ror!($c; operand);
    }
});

macro_rules! om_ror_m (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        om::modify($c, $b, $addr_mode, |c, _, operand| {
            om_ror!(c; operand)
        });
    }
});

macro_rules! om_rol (($c:expr; $operand:expr) => {
    {
        let operand = $operand;
        let c = if $c.p.contains(cpu::CARRY) { 1 } else { 0 };
        let r = (operand << 1) | c;

        $c.p.set(cpu::CARRY, operand & 0x80 != 0);
        $c.p.set(cpu::ZERO, r == 0);
        $c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    }
});

macro_rules! om_rol_a (($c:expr) => {
    {
        let operand = $c.a;
        $c.a = om_rol!($c; operand);
    }
});

macro_rules! om_rol_m (($c:expr, $b:expr; $addr_mode:expr) => {
    {
        om::modify($c, $b, $addr_mode, |c, _, operand| {
            om_rol!(c; operand)
        });
    }
});

pub fn branch(c: &mut Context, b: &mut Bus, flag: Flags, flag_check: bool) {
    // Fetch operand; increment PC
    c.step(b);
    let operand = b.read(c.pc) as i8;
    c.pc = c.pc.wrapping_add(1);

    if c.p.contains(flag) == flag_check {
        // If branch is taken; add operand to PCL
        c.step(b);
        let mut pcl = (c.pc & 0xFF) as u8;
        let overflow_pg: bool;
        if operand > 0 {
            overflow_pg = (pcl as i16 + operand as i16) >= 0x100;
            pcl = pcl.wrapping_add(operand as u8);
        } else {
            overflow_pg = (pcl as i16 + operand as i16) < 0;
            pcl = pcl.wrapping_add(operand as u8);
        }
        c.pc = (c.pc & !0xFF) | (pcl as u16);

        // Fix PCH (if needed)
        if overflow_pg {
            c.step(b);
            if operand > 0 {
                c.pc = c.pc.wrapping_add(0x100);
            } else {
                c.pc = c.pc.wrapping_sub(0x100);
            }
        }
    }
}
