#![allow(non_snake_case)]
use bus::Bus;
use cpu;
use super::Context;
use super::om;

// Set Flag
// ========

// 38 — SEC [1------] {2}
pub fn _38(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::CARRY);
}

// 78 — SEI [--1----] {2}
pub fn _78(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::IRQ_DISABLE);
}

// F8 — SED [---1---] {2}
pub fn _F8(c: &mut Context, _: &mut Bus) {
    c.p.insert(cpu::DECIMAL_MODE);
}

// Clear Flag
// ==========

// 18 — CLC [0------] {2}
pub fn _18(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::CARRY);
}

// 58 — CLI [--0----] {2}
pub fn _58(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::IRQ_DISABLE);
}

// B8 — CLV [-----0-] {2}
pub fn _B8(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::OVERFLOW);
}

// D8 — CLD [---0---] {2}
pub fn _D8(c: &mut Context, _: &mut Bus) {
    c.p.remove(cpu::DECIMAL_MODE);
}

// Transfer
// ========

// AA — TAX [-z----n] {2}
pub fn _AA(c: &mut Context, _: &mut Bus) {
    c.x = c.a;

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// A8 — TAY [-z----n] {2}
pub fn _A8(c: &mut Context, _: &mut Bus) {
    c.y = c.a;

    c.p.set(cpu::ZERO, c.y == 0);
    c.p.set(cpu::SIGN, c.y & 0x80 != 0);
}

// BA — TSX [-z----n] {2}
pub fn _BA(c: &mut Context, _: &mut Bus) {
    c.x = c.s;

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// 8A — TXA [-z----n] {2}
pub fn _8A(c: &mut Context, _: &mut Bus) {
    c.a = c.x;

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// 9A — TXS [-z----n] {2}
pub fn _9A(c: &mut Context, _: &mut Bus) {
    c.s = c.x;
}

// 98 — TYA [-z----n] {2}
pub fn _98(c: &mut Context, _: &mut Bus) {
    c.a = c.y;

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// Load
// ====

// A9 nn — LDA #nn [-z----n] {4}
pub fn _A9(c: &mut Context, b: &mut Bus) {
    c.a = om::read(c, b, om::Immediate);

    c.p.set(cpu::ZERO, c.a == 0);
    c.p.set(cpu::SIGN, c.a & 0x80 != 0);
}

// A2 nn — LDX #nn [-------] {4}
pub fn _A2(c: &mut Context, b: &mut Bus) {
    c.x = om::read(c, b, om::Immediate);

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// Store
// =====

// 8D nn nn — STA nnnn [-------] {4}
pub fn _8D(c: &mut Context, b: &mut Bus) {
    let r = c.a;
    om::write(c, b, om::Absolute, r);
}

// 8E nn nn — STX nnnn [-------] {4}
pub fn _8E(c: &mut Context, b: &mut Bus) {
    let r = c.x;
    om::write(c, b, om::Absolute, r);
}

// Increment
// =========

// E6 nn — INC nn [-z----n] {5}
pub fn _E6(c: &mut Context, b: &mut Bus) {
    om::modify(c, b, om::ZeroPage, |c, _, value| {
        let r = value.wrapping_add(1);

        c.p.set(cpu::ZERO, r == 0);
        c.p.set(cpu::SIGN, r & 0x80 != 0);

        r
    });
}

// E8 — INX [-z----n] {2}
pub fn _E8(c: &mut Context, _: &mut Bus) {
    c.x = c.x.wrapping_add(1);

    c.p.set(cpu::ZERO, c.x == 0);
    c.p.set(cpu::SIGN, c.x & 0x80 != 0);
}

// Jump
// ====

// 4C nn nn — JMP nnnn [-------] {3}
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
