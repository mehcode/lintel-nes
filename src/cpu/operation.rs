use strfmt;
use std::fmt::Write;
use bus::Bus;
use cpu::Context;

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    // No operand beyond instruction semantics [eg. `CLC`]
    Implied,

    // Operates directly on Accumulator (and Accumulator only) [eg. `LRS A`]
    Accumulator,

    // Operates on an 8-bit constant (found immediately after the operation
    // code) [eg. `LDA #$10`]
    Immediate,

    // Operates on an 8-bit value in the first page of memory ($0...$FF) with the 8-bit address
    // found immediately after the operation code [eg. `LDA $00`]
    ZeroPage,

    // Operates on an 8-bit value in the first page of memory ($0...$FF) with the 8-bit address
    // found by the sum of X and the 8-bit value found immediately after the operation
    // code [eg. `STY $10, X`]
    ZeroPageX,

    // Operates on an 8-bit value in the first page of memory ($0...$FF) with the 8-bit address
    // found by the sum of Y and the 8-bit value found immediately after the operation
    // code [eg. `LDX $10, Y`]
    ZeroPageY,

    // Operates on a *signed* 8-bit value found immediately after the operation
    // code [eg. `BNE -6`]
    Relative,

    // [...]
    Absolute,

    // [...]
    AbsoluteX,

    // [...]
    AbsoluteY,

    // [...]
    Indirect,

    // [...]
    IndexedIndirect,

    // [...]
    IndirectIndexed,
}

pub struct Operation {
    // Function to handle the operation
    pub handle: Option<fn(&mut Context, &mut Bus) -> ()>,

    // String format of operation for disassembly
    pub disassembly: &'static str,

    // Number of bytes (incl. opcode)
    pub size: u8,
}

impl Operation {
    pub fn empty() -> Self {
        Operation {
            handle: None,
            disassembly: "",
            size: 0,
        }
    }

    pub fn new(handle: fn(&mut Context, &mut Bus) -> (),
               disassembly: &'static str,
               size: u8)
               -> Self {
        Operation {
            handle: Some(handle),
            disassembly: disassembly,
            size: size,
        }
    }

    pub fn format(&self, c: &Context, b: &mut Bus) -> Result<String, strfmt::FmtError> {
        let n0 = b.read(c.pc) as i64;
        let n1 = b.read(c.pc + 1) as i64;
        strfmt::strfmt_map(self.disassembly,
                           &|mut fmt: strfmt::Formatter| {
            if let Some(ty) = fmt.ty() {
                if ty == 'X' && fmt.key == "0" {
                    fmt.write_str(&format!("{:02X}", n0)).unwrap()
                } else if ty == 'X' && fmt.key == "1" {
                    fmt.write_str(&format!("{:02X}", n1)).unwrap()
                } else {
                    panic!(format!("unknown format: {:?}", fmt))
                }
            } else if fmt.key == "0" {
                fmt.write_str(&format!("{}", n0 as i8)).unwrap()
            } else {
                panic!(format!("unknown format: {:?}", fmt))
            }

            Ok(())
        })
    }
}
