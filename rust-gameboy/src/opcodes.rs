use crate::cpu::Cpu;
use crate::mmu::Mmu;

#[allow(unused_variables)]
pub fn execute_prefixed0x00(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b >> 7;
    cpu.b = (cpu.b << 1) | c;
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x01(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c >> 7;
    cpu.c = (cpu.c << 1) | c;
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x02(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d >> 7;
    cpu.d = (cpu.d << 1) | c;
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x03(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e >> 7;
    cpu.e = (cpu.e << 1) | c;
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x04(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h >> 7;
    cpu.h = (cpu.h << 1) | c;
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x05(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l >> 7;
    cpu.l = (cpu.l << 1) | c;
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x06(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v >> 7;
    v = (v << 1) | c;
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x07(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a >> 7;
    cpu.a = (cpu.a << 1) | c;
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x08(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b & 1;
    cpu.b = (cpu.b >> 1) | (c << 7);
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x09(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c & 1;
    cpu.c = (cpu.c >> 1) | (c << 7);
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d & 1;
    cpu.d = (cpu.d >> 1) | (c << 7);
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e & 1;
    cpu.e = (cpu.e >> 1) | (c << 7);
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h & 1;
    cpu.h = (cpu.h >> 1) | (c << 7);
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l & 1;
    cpu.l = (cpu.l >> 1) | (c << 7);
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v & 1;
    v = (v >> 1) | (c << 7);
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a & 1;
    cpu.a = (cpu.a >> 1) | (c << 7);
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x10(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b >> 7;
    cpu.b = (cpu.b << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x11(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c >> 7;
    cpu.c = (cpu.c << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x12(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d >> 7;
    cpu.d = (cpu.d << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x13(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e >> 7;
    cpu.e = (cpu.e << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x14(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h >> 7;
    cpu.h = (cpu.h << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x15(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l >> 7;
    cpu.l = (cpu.l << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x16(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x17(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a >> 7;
    cpu.a = (cpu.a << 1) | ((cpu.f >> 4) & 1);
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x18(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b & 1;
    cpu.b = (cpu.b >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x19(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c & 1;
    cpu.c = (cpu.c >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d & 1;
    cpu.d = (cpu.d >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e & 1;
    cpu.e = (cpu.e >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h & 1;
    cpu.h = (cpu.h >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l & 1;
    cpu.l = (cpu.l >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a & 1;
    cpu.a = (cpu.a >> 1) | ((cpu.f << 3) & 0x80);
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x20(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b >> 7;
    cpu.b <<= 1;
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x21(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c >> 7;
    cpu.c <<= 1;
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x22(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d >> 7;
    cpu.d <<= 1;
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x23(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e >> 7;
    cpu.e <<= 1;
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x24(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h >> 7;
    cpu.h <<= 1;
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x25(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l >> 7;
    cpu.l <<= 1;
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x26(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v >> 7;
    v <<= 1;
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x27(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a >> 7;
    cpu.a <<= 1;
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x28(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b & 1;
    cpu.b = (cpu.b >> 1) | (cpu.b & 0x80);
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x29(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c & 1;
    cpu.c = (cpu.c >> 1) | (cpu.c & 0x80);
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d & 1;
    cpu.d = (cpu.d >> 1) | (cpu.d & 0x80);
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e & 1;
    cpu.e = (cpu.e >> 1) | (cpu.e & 0x80);
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h & 1;
    cpu.h = (cpu.h >> 1) | (cpu.h & 0x80);
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l & 1;
    cpu.l = (cpu.l >> 1) | (cpu.l & 0x80);
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a & 1;
    cpu.a = (cpu.a >> 1) | (cpu.a & 0x80);
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x30(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= (cpu.b >> 4) | (cpu.b << 4);
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x31(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= (cpu.c >> 4) | (cpu.c << 4);
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x32(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= (cpu.d >> 4) | (cpu.d << 4);
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x33(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= (cpu.e >> 4) | (cpu.e << 4);
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x34(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= (cpu.h >> 4) | (cpu.h << 4);
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x35(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= (cpu.l >> 4) | (cpu.l << 4);
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x36(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= (v >> 4) | (v << 4);
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x37(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= (cpu.a >> 4) | (cpu.a << 4);
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x38(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.b & 1;
    cpu.b >>= 1;
    let z = cpu.b == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x39(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.c & 1;
    cpu.c >>= 1;
    let z = cpu.c == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.d & 1;
    cpu.d >>= 1;
    let z = cpu.d == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.e & 1;
    cpu.e >>= 1;
    let z = cpu.e == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.h & 1;
    cpu.h >>= 1;
    let z = cpu.h == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.l & 1;
    cpu.l >>= 1;
    let z = cpu.l == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let c = v & 1;
    v >>= 1;
    let z = v == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let c = cpu.a & 1;
    cpu.a >>= 1;
    let z = cpu.a == 0;

    let mut f = 0;
    f <<= 1;
    f |= c;
    f <<= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x40(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x41(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x42(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x43(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x44(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x45(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x46(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 0)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x47(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 0)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x48(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x49(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 1)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 1)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x50(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x51(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x52(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x53(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x54(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x55(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x56(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 2)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x57(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 2)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x58(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x59(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 3)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 3)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x60(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x61(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x62(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x63(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x64(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x65(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x66(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 4)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x67(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 4)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x68(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x69(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 5)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 5)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x70(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x71(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x72(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x73(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x74(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x75(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x76(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 6)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x77(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 6)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x78(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.b & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x79(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.c & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.d & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.e & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.h & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.l & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    let z = (v & (1 << 7)) == 0;

    mmu.write(hl, v);

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let z = (cpu.a & (1 << 7)) == 0;

    let mut f = 0;
    f <<= 1;
    f |= (cpu.f >> 7) & 1;
    f <<= 1;
    f |= 1;
    f <<= 1;
    f <<= 1;
    f |= z as u8;
    cpu.f = f << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x80(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x81(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x82(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x83(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x84(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x85(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x86(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 0);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x87(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 0);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x88(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x89(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8a(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8b(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8c(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8d(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 1);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8f(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 1);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x90(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x91(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x92(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x93(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x94(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x95(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x96(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 2);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x97(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 2);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x98(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x99(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9a(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9b(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9c(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9d(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 3);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9f(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 3);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 4);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 4);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xaa(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xab(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xac(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xad(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xae(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 5);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xaf(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 5);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 6);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 6);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xba(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbb(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbc(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbd(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbe(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v &= !(1 << 7);

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbf(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a &= !(1 << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 0;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 0;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xca(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcb(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcc(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcd(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xce(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 1;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcf(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 1;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 2;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 2;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xda(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdb(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdc(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdd(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xde(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 3;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdf(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 3;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 4;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 4;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xea(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xeb(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xec(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xed(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xee(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 5;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xef(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 5;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf0(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf1(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf2(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf4(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 6;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 6;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf8(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfa(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfb(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfc(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfd(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l |= 1 << 7;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfe(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);

    v |= 1 << 7;

    mmu.write(hl, v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xff(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a |= 1 << 7;
}
