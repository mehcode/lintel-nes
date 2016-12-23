#![allow(non_snake_case)]
use bus::Bus;
use cpu;
use super::Context;
use super::om;

// No Operation
// ============

// NOP [-------] {2}
pub fn _EA(_: &mut Context, _: &mut Bus) {
    // Do nothing
}

// Set Flag
// ========

// SEC [1------] {2}
pub fn _38(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::CARRY);
}

// SEI [--1----] {2}
pub fn _78(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::IRQ_DISABLE);
}

// SED [---1---] {2}
pub fn _F8(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::DECIMAL_MODE);
}

// Clear Flag
// ==========

// CLC [0------] {2}
pub fn _18(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::CARRY);
}

// CLI [--0----] {2}
pub fn _58(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::IRQ_DISABLE);
}

// CLV [-----0-] {2}
pub fn _B8(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::OVERFLOW);
}

// CLD [---0---] {2}
pub fn _D8(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::DECIMAL_MODE);
}

// Transfer
// ========

// TAX [-z----n] {2}
pub fn _AA(c: &mut Context, _: &mut Bus) {
    c.x = c.a;

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// TAY [-z----n] {2}
pub fn _A8(c: &mut Context, _: &mut Bus) {
    c.y = c.a;

    c.p.set(cpu::ZERO, c.y == 0);
    c.p.set(cpu::SIGN, c.y & 0x80 != 0);
}

// TSX [-z----n] {2}
pub fn _BA(c: &mut Context, _: &mut Bus) {
    c.x = c.s;

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// TXA [-z----n] {2}
pub fn _8A(c: &mut Context, _: &mut Bus) {
    c.a = c.x;

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// TXS [-z----n] {2}
pub fn _9A(c: &mut Context, _: &mut Bus) {
    c.s = c.x;
}

// TYA [-z----n] {2}
pub fn _98(c: &mut Context, _: &mut Bus) {
    c.a = c.y;

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// Load [-z----n]
// ================================================================================================

pub fn _A9(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Immediate; a);
}

pub fn _A5(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPage; a);
}

pub fn _B5(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPageX; a);
}

pub fn _AD(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Absolute; a);
}

pub fn _BD(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::AbsoluteX; a);
}

pub fn _B9(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::AbsoluteY; a);
}

pub fn _A1(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::IndexedIndirect; a);
}

pub fn _B1(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::IndirectIndexed; a);
}

pub fn _A2(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Immediate; x);
}

pub fn _A6(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPage; x);
}

pub fn _B6(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPageY; x);
}

pub fn _AE(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Absolute; x);
}

pub fn _BE(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::AbsoluteY; x);
}

pub fn _A0(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Immediate; y);
}

pub fn _A4(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPage; y);
}

pub fn _B4(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::ZeroPageX; y);
}

pub fn _AC(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::Absolute; y);
}

pub fn _BC(c: &mut Context, b: &mut Bus) {
    om_load!(c, b; om::AbsoluteX; y);
}

// Store [-------]
// ================================================================================================

pub fn _85(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPage; a);
}

pub fn _95(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPageX; a);
}

pub fn _8D(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::Absolute; a);
}

pub fn _9D(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::AbsoluteX; a);
}

pub fn _99(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::AbsoluteY; a);
}

pub fn _81(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::IndexedIndirect; a);
}

pub fn _91(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::IndirectIndexed; a);
}

pub fn _86(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPage; x);
}

pub fn _96(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPageY; x);
}

pub fn _8E(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::Absolute; x);
}

pub fn _84(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPage; y);
}

pub fn _94(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::ZeroPageX; y);
}

pub fn _8C(c: &mut Context, b: &mut Bus) {
    om_store!(c, b; om::Absolute; y);
}

// Add w/Carry [cz---vn]
// ================================================================================================

pub fn _69(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::Immediate);
}

pub fn _65(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::ZeroPage);
}

pub fn _75(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::ZeroPageX);
}

pub fn _6D(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::Absolute);
}

pub fn _7D(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::AbsoluteX);
}

pub fn _79(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::AbsoluteY);
}

pub fn _61(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::IndexedIndirect);
}

pub fn _71(c: &mut Context, b: &mut Bus) {
    om_adc!(c, b; om::IndirectIndexed);
}

// Subtract w/Carry [cz---vn]
// ================================================================================================

pub fn _E9(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::Immediate);
}

pub fn _E5(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::ZeroPage);
}

pub fn _F5(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::ZeroPageX);
}

pub fn _ED(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::Absolute);
}

pub fn _FD(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::AbsoluteX);
}

pub fn _F9(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::AbsoluteY);
}

pub fn _E1(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::IndexedIndirect);
}

pub fn _F1(c: &mut Context, b: &mut Bus) {
    om_sbc!(c, b; om::IndirectIndexed);
}

// Compare [cz----n]
// ================================================================================================

pub fn _C9(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Immediate; a);
}

pub fn _C5(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::ZeroPage; a);
}

pub fn _D5(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::ZeroPageX; a);
}

pub fn _CD(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Absolute; a);
}

pub fn _DD(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::AbsoluteX; a);
}

pub fn _D9(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::AbsoluteY; a);
}

pub fn _C1(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::IndexedIndirect; a);
}

pub fn _D1(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::IndirectIndexed; a);
}

pub fn _E0(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Immediate; x);
}

pub fn _E4(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::ZeroPage; x);
}

pub fn _EC(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Absolute; x);
}

pub fn _C0(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Immediate; y);
}

pub fn _C4(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::ZeroPage; y);
}

pub fn _CC(c: &mut Context, b: &mut Bus) {
    om_compare!(c, b; om::Absolute; y);
}

// Increment
// ================================================================================================

pub fn _E6(c: &mut Context, b: &mut Bus) {
    om_inc!(c, b; om::ZeroPage);
}

pub fn _F6(c: &mut Context, b: &mut Bus) {
    om_inc!(c, b; om::ZeroPageX);
}

pub fn _EE(c: &mut Context, b: &mut Bus) {
    om_inc!(c, b; om::Absolute);
}

pub fn _FE(c: &mut Context, b: &mut Bus) {
    om_inc!(c, b; om::AbsoluteX);
}

pub fn _E8(c: &mut Context, _: &mut Bus) {
    c.x = c.x.wrapping_add(1);

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

pub fn _C8(c: &mut Context, _: &mut Bus) {
    c.y = c.y.wrapping_add(1);

    c.p.set(cpu::ZERO, c.y == 0);
    c.p.set(cpu::SIGN, c.y & 0x80 != 0);
}

// Decrement
// ================================================================================================

pub fn _C6(c: &mut Context, b: &mut Bus) {
    om_dec!(c, b; om::ZeroPage);
}

pub fn _D6(c: &mut Context, b: &mut Bus) {
    om_dec!(c, b; om::ZeroPageX);
}

pub fn _CE(c: &mut Context, b: &mut Bus) {
    om_dec!(c, b; om::Absolute);
}

pub fn _DE(c: &mut Context, b: &mut Bus) {
    om_dec!(c, b; om::AbsoluteX);
}

pub fn _CA(c: &mut Context, _: &mut Bus) {
    c.x = c.x.wrapping_sub(1);

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

pub fn _88(c: &mut Context, _: &mut Bus) {
    c.y = c.y.wrapping_sub(1);

    c.p.set(cpu::ZERO, c.y == 0);
    c.p.set(cpu::SIGN, c.y & 0x80 != 0);
}

// Logical Inclusive OR [-z----n]
// ================================================================================================

pub fn _09(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::Immediate);
}

pub fn _05(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::ZeroPage);
}

pub fn _15(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::ZeroPageX);
}

pub fn _0D(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::Absolute);
}

pub fn _1D(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::AbsoluteX);
}

pub fn _19(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::AbsoluteY);
}

pub fn _01(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::IndexedIndirect);
}

pub fn _11(c: &mut Context, b: &mut Bus) {
    om_ora!(c, b; om::IndirectIndexed);
}

// Logical AND [-z----n]
// ================================================================================================

pub fn _29(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::Immediate);
}

pub fn _25(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::ZeroPage);
}

pub fn _35(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::ZeroPageX);
}

pub fn _2D(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::Absolute);
}

pub fn _3D(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::AbsoluteX);
}

pub fn _39(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::AbsoluteY);
}

pub fn _21(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::IndexedIndirect);
}

pub fn _31(c: &mut Context, b: &mut Bus) {
    om_and!(c, b; om::IndirectIndexed);
}

// Exclusive OR [-z----n]
// ================================================================================================

pub fn _49(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::Immediate);
}

pub fn _45(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::ZeroPage);
}

pub fn _55(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::ZeroPageX);
}

pub fn _4D(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::Absolute);
}

pub fn _5D(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::AbsoluteX);
}

pub fn _59(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::AbsoluteY);
}

pub fn _41(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::IndexedIndirect);
}

pub fn _51(c: &mut Context, b: &mut Bus) {
    om_eor!(c, b; om::IndirectIndexed);
}

// Arithmetic Shift Left [cz----n]
// ================================================================================================

pub fn _0A(c: &mut Context, _: &mut Bus) {
    om_asl_a!(c);
}

pub fn _06(c: &mut Context, b: &mut Bus) {
    om_asl_m!(c, b; om::ZeroPage);
}

pub fn _16(c: &mut Context, b: &mut Bus) {
    om_asl_m!(c, b; om::ZeroPageX);
}

pub fn _0E(c: &mut Context, b: &mut Bus) {
    om_asl_m!(c, b; om::Absolute);
}

pub fn _1E(c: &mut Context, b: &mut Bus) {
    om_asl_m!(c, b; om::AbsoluteX);
}

// Logical Shift Right [cz----n]
// ================================================================================================

pub fn _4A(c: &mut Context, _: &mut Bus) {
    om_lsr_a!(c);
}

pub fn _46(c: &mut Context, b: &mut Bus) {
    om_lsr_m!(c, b; om::ZeroPage);
}

pub fn _56(c: &mut Context, b: &mut Bus) {
    om_lsr_m!(c, b; om::ZeroPageX);
}

pub fn _4E(c: &mut Context, b: &mut Bus) {
    om_lsr_m!(c, b; om::Absolute);
}

pub fn _5E(c: &mut Context, b: &mut Bus) {
    om_lsr_m!(c, b; om::AbsoluteX);
}

// Rotate Left [cz----n]
// ================================================================================================

pub fn _2A(c: &mut Context, _: &mut Bus) {
    om_rol_a!(c);
}

pub fn _26(c: &mut Context, b: &mut Bus) {
    om_rol_m!(c, b; om::ZeroPage);
}

pub fn _36(c: &mut Context, b: &mut Bus) {
    om_rol_m!(c, b; om::ZeroPageX);
}

pub fn _2E(c: &mut Context, b: &mut Bus) {
    om_rol_m!(c, b; om::Absolute);
}

pub fn _3E(c: &mut Context, b: &mut Bus) {
    om_rol_m!(c, b; om::AbsoluteX);
}

// Rotate Right [cz----n]
// ================================================================================================

pub fn _6A(c: &mut Context, _: &mut Bus) {
    om_ror_a!(c);
}

pub fn _66(c: &mut Context, b: &mut Bus) {
    om_ror_m!(c, b; om::ZeroPage);
}

pub fn _76(c: &mut Context, b: &mut Bus) {
    om_ror_m!(c, b; om::ZeroPageX);
}

pub fn _6E(c: &mut Context, b: &mut Bus) {
    om_ror_m!(c, b; om::Absolute);
}

pub fn _7E(c: &mut Context, b: &mut Bus) {
    om_ror_m!(c, b; om::AbsoluteX);
}

// Bit [-z---vn]
// ================================================================================================

pub fn _24(c: &mut Context, b: &mut Bus) {
    let mask = c.a;
    let value = om::read(c, b, om::ZeroPage);
    let r = value & mask;

    c.p.set(cpu::ZERO, r == 0);
    c.p.set(cpu::OVERFLOW, value & 0x40 != 0);
    c.p.set(cpu::SIGN, value & 0x80 != 0);
}

pub fn _2C(c: &mut Context, b: &mut Bus) {
    let mask = c.a;
    let value = om::read(c, b, om::Absolute);
    let r = value & mask;

    c.p.set(cpu::ZERO, r == 0);
    c.p.set(cpu::OVERFLOW, value & 0x40 != 0);
    c.p.set(cpu::SIGN, value & 0x80 != 0);
}

// Push
// ================================================================================================

pub fn _48(c: &mut Context, b: &mut Bus) {
    // Push register on stack; decrement S
    c.step(b);
    b.write(0x100 + c.s as u16, c.a);
    c.s = c.s.wrapping_sub(1);
}

pub fn _08(c: &mut Context, b: &mut Bus) {
    // Push register on stack; decrement S
    //  BRK always pushed as 1 as well as the unused one
    c.step(b);
    b.write(0x100 + c.s as u16, (c.p | cpu::BREAK).bits | 0x20);
    c.s = c.s.wrapping_sub(1);
}

// Pull
// ================================================================================================

// PLA
pub fn _68(c: &mut Context, b: &mut Bus) {
    // Increment S
    c.step(b);
    c.s = c.s.wrapping_add(1);

    // Pull register from stack
    c.a = b.read(0x100 + c.s as u16);

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// PLP
pub fn _28(c: &mut Context, b: &mut Bus) {
    // Increment S
    c.step(b);
    c.s = c.s.wrapping_add(1);

    // Pull register from stack
    c.p = cpu::Flags::from_bits_truncate(b.read(0x100 + c.s as u16));
    c.p.remove(cpu::BREAK);
}

// Jump
// ================================================================================================

pub fn _20(c: &mut Context, b: &mut Bus) {
    // Fetch low address byte; increment PC
    c.step(b);
    let l = b.read(c.pc);
    c.pc = c.pc.wrapping_add(1);

    // Delay
    c.step(b);

    // Push PCH on stack; decrement S
    c.step(b);
    b.write(0x100 + c.s as u16, (c.pc >> 8) as u8);
    c.s = c.s.wrapping_sub(1);

    // Push PCL on stack; decrement S
    c.step(b);
    b.write(0x100 + c.s as u16, (c.pc & 0xFF) as u8);
    c.s = c.s.wrapping_sub(1);

    // Copy low address byte to PCL; fetch high address byte to PCH
    c.step(b);
    let h = b.read(c.pc);
    c.pc = l as u16 | ((h as u16) << 8);
}

// JMP u16
pub fn _4C(c: &mut Context, b: &mut Bus) {
    // Fetch low address byte; increment PC
    c.step(b);
    let l = b.read(c.pc);
    c.pc = c.pc.wrapping_add(1);

    // Copy low address byte to PCL; fetch high address byte to PCH
    c.step(b);
    let h = b.read(c.pc);
    c.pc = l as u16 | ((h as u16) << 8);
}

// JMP (u16)
pub fn _6C(c: &mut Context, b: &mut Bus) {
    // Fetch pointer address low; increment PC
    c.step(b);
    let ptr_l = b.read(c.pc);
    c.pc = c.pc.wrapping_add(1);

    // Fetch pointer address high; increment PC
    c.step(b);
    let ptr_h = b.read(c.pc);
    c.pc = c.pc.wrapping_add(1);

    // Fetch low address
    c.step(b);
    let l = b.read(ptr_l as u16 | ((ptr_h as u16) << 8));

    // Fetch high address (note: page crossing is not handled)
    c.step(b);
    let h = b.read(ptr_l.wrapping_add(1) as u16 | ((ptr_h as u16) << 8));
    c.pc = l as u16 | ((h as u16) << 8);
}

// Return
// ================================================================================================

pub fn _60(c: &mut Context, b: &mut Bus) {
    // Increment S
    c.step(b);
    c.s = c.s.wrapping_add(1);

    // Pull PCL from stack; increment S
    c.step(b);
    c.pc = (c.pc & !0xFF) | (b.read(0x100 + c.s as u16) as u16);
    c.s = c.s.wrapping_add(1);

    // Pull PCH from stack
    c.step(b);
    c.pc = (c.pc & !0xFF00) | ((b.read(0x100 + c.s as u16) as u16) << 8);

    // Increment PC
    c.step(b);
    c.pc = c.pc.wrapping_add(1);
}

pub fn _40(c: &mut Context, b: &mut Bus) {
    // Increment S
    c.step(b);
    c.s = c.s.wrapping_add(1);

    // Pull P from stack; increment S
    c.step(b);
    c.p = cpu::Flags::from_bits_truncate(b.read(0x100 + c.s as u16));
    c.p.remove(cpu::BREAK);
    c.s = c.s.wrapping_add(1);

    // Pull PCL from stack; increment S
    c.step(b);
    c.pc = (c.pc & !0xFF) | (b.read(0x100 + c.s as u16) as u16);
    c.s = c.s.wrapping_add(1);

    // Pull PCH from stack
    c.step(b);
    c.pc = (c.pc & !0xFF00) | ((b.read(0x100 + c.s as u16) as u16) << 8);
}

// Branch
// ================================================================================================

// BCC nn [-------] {2...4}
pub fn _90(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::CARRY, false);
}

// BCS nn [-------] {2...4}
pub fn _B0(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::CARRY, true);
}

// BNE nn [-------] {2...4}
pub fn _D0(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::ZERO, false);
}

// BEQ nn [-------] {2...4}
pub fn _F0(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::ZERO, true);
}

// BPL nn [-------] {2...4}
pub fn _10(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::SIGN, false);
}

// BMI nn [-------] {2...4}
pub fn _30(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::SIGN, true);
}

// BVC nn [-------] {2...4}
pub fn _50(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::OVERFLOW, false);
}

// BVS nn [-------] {2...4}
pub fn _70(c: &mut Context, b: &mut Bus) {
    om::branch(c, b, cpu::OVERFLOW, true);
}
