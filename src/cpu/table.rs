use std::ops::Index;
use std::vec::Vec;
use super::op;
use super::operation::Operation;
use super::operation::AddressingMode::*;

pub struct Table {
    // Operation table
    //  + 0x000 - 0x00 - 0xFF
    operations: Vec<Operation>,
}

impl Default for Table {
    fn default() -> Self {
        // TODO: Use vec![...,...] notation; this is for WIP
        let mut operations = Vec::with_capacity(0x100);

        // Fill operations table with empty operations
        // TODO(rust): Figure out how to do this with operations.resize
        while operations.len() < 0x100 {
            operations.push(Operation::empty());
        }

        // Set Flag
        // ========

        operations[0x38] = Operation::new(op::_38, "SEC", 1, Implied);
        operations[0x78] = Operation::new(op::_78, "SED", 1, Implied);
        operations[0xF8] = Operation::new(op::_F8, "SEI", 1, Implied);

        // Clear Flag
        // ==========

        operations[0x18] = Operation::new(op::_18, "CLC", 1, Implied);
        operations[0x58] = Operation::new(op::_58, "CLD", 1, Implied);
        operations[0xB8] = Operation::new(op::_B8, "CLV", 1, Implied);
        operations[0xD8] = Operation::new(op::_D8, "CLD", 1, Implied);

        // Transfer
        // ========

        operations[0xAA] = Operation::new(op::_AA, "TAX", 1, Implied);
        operations[0xA8] = Operation::new(op::_A8, "TAY", 1, Implied);
        operations[0xBA] = Operation::new(op::_BA, "TSX", 1, Implied);

        operations[0x8A] = Operation::new(op::_8A, "TXA", 1, Implied);
        operations[0x9A] = Operation::new(op::_9A, "TXS", 1, Implied);
        operations[0x98] = Operation::new(op::_98, "TYA", 1, Implied);

        // Load
        // ====

        operations[0xA9] = Operation::new(op::_A9, "LDA #${0:X}", 2, Immediate);

        operations[0xA2] = Operation::new(op::_A2, "LDX #${0:X}", 2, Immediate);

        // Store
        // =====

        operations[0x8D] = Operation::new(op::_8D, "STA ${1:X}{0:X}", 3, Absolute);

        operations[0x8E] = Operation::new(op::_8E, "STX ${1:X}{0:X}", 3, Absolute);

        // Increment
        // =========

        operations[0xE6] = Operation::new(op::_E6, "INC ${0:X}", 2, ZeroPage);

        operations[0xE8] = Operation::new(op::_E8, "INX", 1, Implied);

        // Jump
        // ====

        operations[0x4C] = Operation::new(op::_4C, "JMP ${1:X}{0:X}", 3, Absolute);

        Table { operations: operations }
    }
}

impl Index<u8> for Table {
    type Output = Operation;

    fn index(&self, index: u8) -> &Operation {
        &self.operations[index as usize]
    }
}
