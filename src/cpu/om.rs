use bus::Bus;
use super::Context;
use super::operation::AddressingMode;
pub use super::operation::AddressingMode::*;

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

        _ => {
            panic!("unsupported addressing mode for Read operation: {:?}", mode);
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
        Absolute => {
            // 2: Fetch low byte of address; increment PC
            c.step(b);
            let mut address = b.read(c.pc) as u16;
            c.pc = c.pc.wrapping_add(1);

            // 3: Fetch high byte of address; increment PC
            c.step(b);
            address |= (b.read(c.pc) as u16) << 8;
            c.pc = c.pc.wrapping_add(1);

            address
        }

        _ => {
            panic!("unsupported addressing mode for Write operation: {:?}",
                   mode);
        }
    };

    // 4 (Absolute): Write to effective address
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
            // 2: Fetch address; increment PC
            c.step(b);
            let address = b.read(c.pc);
            c.pc = c.pc.wrapping_add(1);

            address as u16
        }

        _ => {
            panic!("unsupported addressing mode for Read-Modify-Write operation: {:?}",
                   mode);
        }
    };

    // 3 (ZeroPage): Read from effective address
    c.step(b);
    let mut value = b.read(address);

    // 4 (ZeroPage): Write the (same) value back to effective address [..]
    c.step(b);
    b.write(address, value);

    // [..] and do the operation on it
    value = cb(c, b, value);

    // 5 (ZeroPage): Write the new value to effective address
    c.step(b);
    b.write(address, value);
}
