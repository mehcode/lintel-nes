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

        // No Operation
        // ============

        operations[0xEA] = Operation::new(op::_EA, "NOP", 1);

        // Set Flag
        // ========

        operations[0x38] = Operation::new(op::_38, "SEC", 1);
        operations[0x78] = Operation::new(op::_78, "SED", 1);
        operations[0xF8] = Operation::new(op::_F8, "SEI", 1);

        // Clear Flag
        // ==========

        operations[0x18] = Operation::new(op::_18, "CLC", 1);
        operations[0x58] = Operation::new(op::_58, "CLD", 1);
        operations[0xB8] = Operation::new(op::_B8, "CLV", 1);
        operations[0xD8] = Operation::new(op::_D8, "CLD", 1);

        // Transfer
        // ========

        operations[0xAA] = Operation::new(op::_AA, "TAX", 1);
        operations[0xA8] = Operation::new(op::_A8, "TAY", 1);
        operations[0xBA] = Operation::new(op::_BA, "TSX", 1);

        operations[0x8A] = Operation::new(op::_8A, "TXA", 1);
        operations[0x9A] = Operation::new(op::_9A, "TXS", 1);
        operations[0x98] = Operation::new(op::_98, "TYA", 1);

        // Load
        // ====

        operations[0xA9] = Operation::new(op::_A9, "LDA #${0:X}", 2);
        operations[0xA5] = Operation::new(op::_A5, "LDA ${0:X}", 2);
        operations[0xB5] = Operation::new(op::_B5, "LDA ${0:X}, X", 2);
        operations[0xAD] = Operation::new(op::_AD, "LDA ${1:X}{0:X}", 2);
        operations[0xBD] = Operation::new(op::_BD, "LDA ${1:X}{0:X}, X", 2);
        operations[0xB9] = Operation::new(op::_B9, "LDA ${1:X}{0:X}, Y", 2);
        operations[0xA1] = Operation::new(op::_A1, "LDA (${0:X}, X)", 2);
        operations[0xB1] = Operation::new(op::_B1, "LDA (${0:X}), Y", 2);

        operations[0xA2] = Operation::new(op::_A2, "LDX #${0:X}", 2);
        operations[0xA6] = Operation::new(op::_A6, "LDX ${0:X}", 2);
        operations[0xB6] = Operation::new(op::_B6, "LDX ${0:X}, Y", 2);
        operations[0xAE] = Operation::new(op::_AE, "LDX ${1:X}{0:X}", 2);
        operations[0xBE] = Operation::new(op::_BE, "LDX ${1:X}{0:X}, Y", 2);

        operations[0xA0] = Operation::new(op::_A0, "LDY #${0:X}", 2);
        operations[0xA4] = Operation::new(op::_A4, "LDX ${0:X}", 2);
        operations[0xB4] = Operation::new(op::_B4, "LDX ${0:X}, X", 2);
        operations[0xAC] = Operation::new(op::_AC, "LDX ${1:X}{0:X}", 2);
        operations[0xBC] = Operation::new(op::_BC, "LDX ${1:X}{0:X}, X", 2);

        // Store
        // =====

        operations[0x85] = Operation::new(op::_85, "STA ${0:X}", 2);
        operations[0x95] = Operation::new(op::_95, "STA ${0:X}, X", 2);
        operations[0x8D] = Operation::new(op::_8D, "STA ${1:X}{0:X}", 3);
        operations[0x9D] = Operation::new(op::_9D, "STA ${1:X}{0:X}, X", 3);
        operations[0x99] = Operation::new(op::_99, "STA ${1:X}{0:X}, Y", 3);
        operations[0x81] = Operation::new(op::_81, "STA (${0:X}, X)", 2);
        operations[0x91] = Operation::new(op::_91, "STA (${0:X}), Y", 2);

        operations[0x86] = Operation::new(op::_86, "STX ${0:X}", 2);
        operations[0x96] = Operation::new(op::_96, "STX ${0:X}, Y", 3);
        operations[0x8E] = Operation::new(op::_8E, "STX ${1:X}{0:X}", 3);

        operations[0x84] = Operation::new(op::_84, "STY ${0:X}", 2);
        operations[0x94] = Operation::new(op::_94, "STY ${0:X}, X", 3);
        operations[0x8C] = Operation::new(op::_8C, "STY ${1:X}{0:X}", 3);

        // Add w/Carry
        // ===========

        operations[0x69] = Operation::new(op::_69, "ADC #${0:X}", 2);
        operations[0x65] = Operation::new(op::_65, "ADC ${0:X}", 2);
        operations[0x75] = Operation::new(op::_75, "ADC ${0:X}, X", 2);
        operations[0x6D] = Operation::new(op::_6D, "ADC ${1:X}{0:X}", 3);
        operations[0x7D] = Operation::new(op::_7D, "ADC ${1:X}{0:X}, X", 3);
        operations[0x79] = Operation::new(op::_79, "ADC ${1:X}{0:X}, Y", 3);
        operations[0x61] = Operation::new(op::_61, "ADC (${0:X}, X)", 2);
        operations[0x71] = Operation::new(op::_71, "ADC (${0:X}), Y", 2);

        // Subtract w/Carry
        // ================

        operations[0xE9] = Operation::new(op::_E9, "SBC #${0:X}", 2);
        operations[0xE5] = Operation::new(op::_E5, "SBC ${0:X}", 2);
        operations[0xF5] = Operation::new(op::_F5, "SBC ${0:X}, X", 2);
        operations[0xED] = Operation::new(op::_ED, "SBC ${1:X}{0:X}", 3);
        operations[0xFD] = Operation::new(op::_FD, "SBC ${1:X}{0:X}, X", 3);
        operations[0xF9] = Operation::new(op::_F9, "SBC ${1:X}{0:X}, Y", 3);
        operations[0xE1] = Operation::new(op::_E1, "SBC (${0:X}, X)", 2);
        operations[0xF1] = Operation::new(op::_F1, "SBC (${0:X}), Y", 2);

        // Compare
        // =======

        operations[0xC9] = Operation::new(op::_C9, "CMP #${0:X}", 2);
        operations[0xC5] = Operation::new(op::_C5, "CMP ${0:X}", 2);
        operations[0xD5] = Operation::new(op::_D5, "CMP ${0:X}, X", 2);
        operations[0xCD] = Operation::new(op::_CD, "CMP ${1:X}{0:X}", 3);
        operations[0xDD] = Operation::new(op::_DD, "CMP ${1:X}{0:X}, X", 3);
        operations[0xD9] = Operation::new(op::_D9, "CMP ${1:X}{0:X}, Y", 3);
        operations[0xC1] = Operation::new(op::_C1, "CMP (${0:X}, X)", 2);
        operations[0xD1] = Operation::new(op::_D1, "CMP (${0:X}), Y", 2);

        operations[0xE0] = Operation::new(op::_E0, "CPX #${0:X}", 2);
        operations[0xE4] = Operation::new(op::_E4, "CPX ${0:X}", 2);
        operations[0xEC] = Operation::new(op::_EC, "CPX ${1:X}{0:X}", 3);

        operations[0xC0] = Operation::new(op::_C0, "CPY #${0:X}", 2);
        operations[0xC4] = Operation::new(op::_C4, "CPY ${0:X}", 2);
        operations[0xCC] = Operation::new(op::_CC, "CPY ${1:X}{0:X}", 3);

        // Increment
        // =========

        operations[0xE6] = Operation::new(op::_E6, "INC ${0:X}", 2);
        operations[0xF6] = Operation::new(op::_F6, "INC ${0:X}, X", 2);
        operations[0xEE] = Operation::new(op::_EE, "INC ${1:X}{0:X}", 3);
        operations[0xFE] = Operation::new(op::_FE, "INC ${1:X}{0:X}, X", 3);

        operations[0xE8] = Operation::new(op::_E8, "INX", 1);
        operations[0xC8] = Operation::new(op::_C8, "INY", 1);

        // Decrement
        // =========

        operations[0xC6] = Operation::new(op::_C6, "DEC ${0:X}", 2);
        operations[0xD6] = Operation::new(op::_D6, "DEC ${0:X}, X", 2);
        operations[0xCE] = Operation::new(op::_CE, "DEC ${1:X}{0:X}", 3);
        operations[0xDE] = Operation::new(op::_DE, "DEC ${1:X}{0:X}, X", 3);

        operations[0xCA] = Operation::new(op::_CA, "DEX", 1);
        operations[0x88] = Operation::new(op::_88, "DEY", 1);

        // Logical Inclusive OR
        // ====================

        operations[0x09] = Operation::new(op::_09, "ORA #${0:X}", 2);
        operations[0x05] = Operation::new(op::_05, "ORA ${0:X}", 2);
        operations[0x15] = Operation::new(op::_15, "ORA ${0:X}, X", 2);
        operations[0x0D] = Operation::new(op::_0D, "ORA ${1:X}{0:X}", 3);
        operations[0x1D] = Operation::new(op::_1D, "ORA ${1:X}{0:X}, X", 3);
        operations[0x19] = Operation::new(op::_19, "ORA ${1:X}{0:X}, Y", 3);
        operations[0x01] = Operation::new(op::_01, "ORA (${0:X}, X)", 2);
        operations[0x11] = Operation::new(op::_11, "ORA (${0:X}), Y", 2);

        // Logical AND
        // ===========

        operations[0x29] = Operation::new(op::_29, "AND #${0:X}", 2);
        operations[0x25] = Operation::new(op::_25, "AND ${0:X}", 2);
        operations[0x35] = Operation::new(op::_35, "AND ${0:X}, X", 2);
        operations[0x2D] = Operation::new(op::_2D, "AND ${1:X}{0:X}", 3);
        operations[0x3D] = Operation::new(op::_3D, "AND ${1:X}{0:X}, X", 3);
        operations[0x39] = Operation::new(op::_39, "AND ${1:X}{0:X}, Y", 3);
        operations[0x21] = Operation::new(op::_21, "AND (${0:X}, X)", 2);
        operations[0x31] = Operation::new(op::_31, "AND (${0:X}), Y", 2);

        // Exclusive OR
        // ============

        operations[0x49] = Operation::new(op::_49, "EOR #${0:X}", 2);
        operations[0x45] = Operation::new(op::_45, "EOR ${0:X}", 2);
        operations[0x55] = Operation::new(op::_55, "EOR ${0:X}, X", 2);
        operations[0x4D] = Operation::new(op::_4D, "EOR ${1:X}{0:X}", 3);
        operations[0x5D] = Operation::new(op::_5D, "EOR ${1:X}{0:X}, X", 3);
        operations[0x59] = Operation::new(op::_59, "EOR ${1:X}{0:X}, Y", 3);
        operations[0x41] = Operation::new(op::_41, "EOR (${0:X}, X)", 2);
        operations[0x51] = Operation::new(op::_51, "EOR (${0:X}), Y", 2);

        // Arithmetic Shift Left
        // =====================

        operations[0x0A] = Operation::new(op::_0A, "ASL A", 1);
        operations[0x06] = Operation::new(op::_06, "ASL ${0:X}", 2);
        operations[0x16] = Operation::new(op::_16, "ASL ${0:X}, X", 2);
        operations[0x0E] = Operation::new(op::_0E, "ASL ${1:X}{0:X}", 3);
        operations[0x1E] = Operation::new(op::_1E, "ASL ${1:X}{0:X}, X", 3);

        // Logical Shift Right
        // ===================

        operations[0x4A] = Operation::new(op::_4A, "LRS A", 1);
        operations[0x46] = Operation::new(op::_46, "LRS ${0:X}", 2);
        operations[0x56] = Operation::new(op::_56, "LRS ${0:X}, X", 2);
        operations[0x4E] = Operation::new(op::_4E, "LRS ${1:X}{0:X}", 3);
        operations[0x5E] = Operation::new(op::_5E, "LRS ${1:X}{0:X}, X", 3);

        // Rotate Left
        // ===========

        operations[0x2A] = Operation::new(op::_2A, "ROL A", 1);
        operations[0x26] = Operation::new(op::_26, "ROL ${0:X}", 2);
        operations[0x36] = Operation::new(op::_36, "ROL ${0:X}, X", 2);
        operations[0x2E] = Operation::new(op::_2E, "ROL ${1:X}{0:X}", 3);
        operations[0x3E] = Operation::new(op::_3E, "ROL ${1:X}{0:X}, X", 3);

        // Rotate Right
        // ============

        operations[0x6A] = Operation::new(op::_6A, "ROR A", 1);
        operations[0x66] = Operation::new(op::_66, "ROR ${0:X}", 2);
        operations[0x76] = Operation::new(op::_76, "ROR ${0:X}, X", 2);
        operations[0x6E] = Operation::new(op::_6E, "ROR ${1:X}{0:X}", 3);
        operations[0x7E] = Operation::new(op::_7E, "ROR ${1:X}{0:X}, X", 3);

        // Bit
        // ===

        operations[0x24] = Operation::new(op::_24, "BIT ${0:X}", 2);
        operations[0x2C] = Operation::new(op::_2C, "BIT ${1:X}{0:X}", 3);

        // Push
        // ====

        operations[0x48] = Operation::new(op::_48, "PHA", 1);
        operations[0x08] = Operation::new(op::_08, "PHP", 1);

        // Pull
        // ====

        operations[0x68] = Operation::new(op::_68, "PLA", 1);
        operations[0x28] = Operation::new(op::_28, "PLP", 1);

        // Jump
        // ====

        operations[0x20] = Operation::new(op::_20, "JSR ${1:X}{0:X}", 3);
        operations[0x4C] = Operation::new(op::_4C, "JMP ${1:X}{0:X}", 3);
        operations[0x6C] = Operation::new(op::_6C, "JMP (${1:X}{0:X})", 3);

        // Return
        // ======

        operations[0x60] = Operation::new(op::_60, "RTS", 1);
        operations[0x40] = Operation::new(op::_40, "RTI", 1);

        // Branch
        // ======

        operations[0x90] = Operation::new(op::_90, "BCC {0}", 2);
        operations[0xB0] = Operation::new(op::_B0, "BCS {0}", 2);
        operations[0xF0] = Operation::new(op::_F0, "BEQ {0}", 2);
        operations[0xD0] = Operation::new(op::_D0, "BNE {0}", 2);
        operations[0x30] = Operation::new(op::_30, "BMI {0}", 2);
        operations[0x10] = Operation::new(op::_10, "BPL {0}", 2);
        operations[0x50] = Operation::new(op::_50, "BVC {0}", 2);
        operations[0x70] = Operation::new(op::_70, "BVS {0}", 2);

        Table { operations: operations }
    }
}

impl Index<u8> for Table {
    type Output = Operation;

    fn index(&self, index: u8) -> &Operation {
        &self.operations[index as usize]
    }
}
