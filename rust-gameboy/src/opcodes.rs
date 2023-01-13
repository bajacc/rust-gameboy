use crate::cpu::Cpu;
use crate::mmu::Mmu;

pub fn read_bc(cpu: &mut Cpu) -> u16 {
    return (cpu.c as u16) | ((cpu.b as u16) << 8);
}

pub fn write_bc(cpu: &mut Cpu, v: u16) {
    cpu.b = (v >> 8) as u8;
    cpu.c = (v & 0xff) as u8;
}
pub fn read_de(cpu: &mut Cpu) -> u16 {
    return (cpu.e as u16) | ((cpu.d as u16) << 8);
}

pub fn write_de(cpu: &mut Cpu, v: u16) {
    cpu.d = (v >> 8) as u8;
    cpu.e = (v & 0xff) as u8;
}
pub fn read_hl(cpu: &mut Cpu) -> u16 {
    return (cpu.l as u16) | ((cpu.h as u16) << 8);
}

pub fn write_hl(cpu: &mut Cpu, v: u16) {
    cpu.h = (v >> 8) as u8;
    cpu.l = (v & 0xff) as u8;
}

#[allow(unused_variables)]
pub fn execute_prefixed0x00(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x01(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x02(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x03(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x04(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x05(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x06(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v >> 7;
    v = (v << 1) | c;
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x07(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v >> 7;
    v = (v << 1) | c;
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x08(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x09(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x0f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v & 1;
    v = (v >> 1) | (c << 7);
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x10(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x11(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x12(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x13(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x14(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x15(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x16(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x17(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x18(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x19(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x1f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x20(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v >> 7;
    v <<= 1;
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x21(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v >> 7;
    v <<= 1;
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x22(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v >> 7;
    v <<= 1;
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x23(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v >> 7;
    v <<= 1;
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x24(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v >> 7;
    v <<= 1;
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x25(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v >> 7;
    v <<= 1;
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x26(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v >> 7;
    v <<= 1;
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x27(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v >> 7;
    v <<= 1;
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x28(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x29(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x2f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x30(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v = (v >> 4) | (v << 4);
    cpu.b = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x31(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v = (v >> 4) | (v << 4);
    cpu.c = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x32(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v = (v >> 4) | (v << 4);
    cpu.d = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x33(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v = (v >> 4) | (v << 4);
    cpu.e = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x34(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v = (v >> 4) | (v << 4);
    cpu.h = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x35(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v = (v >> 4) | (v << 4);
    cpu.l = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x36(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v = (v >> 4) | (v << 4);
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x37(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v = (v >> 4) | (v << 4);
    cpu.a = v;
    let z = v == 0;

    cpu.f = (0 << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x38(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    let c = v & 1;
    v >>= 1;
    cpu.b = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x39(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    let c = v & 1;
    v >>= 1;
    cpu.c = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    let c = v & 1;
    v >>= 1;
    cpu.d = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    let c = v & 1;
    v >>= 1;
    cpu.e = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    let c = v & 1;
    v >>= 1;
    cpu.h = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    let c = v & 1;
    v >>= 1;
    cpu.l = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    let c = v & 1;
    v >>= 1;
    mmu.write(read_hl(cpu), v);
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x3f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    let c = v & 1;
    v >>= 1;
    cpu.a = v;
    let z = v == 0;

    cpu.f = (c << 4) | (0 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x40(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x41(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x42(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x43(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x44(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x45(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x46(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x47(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 0)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x48(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x49(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x4f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 1)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x50(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x51(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x52(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x53(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x54(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x55(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x56(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x57(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 2)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x58(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x59(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x5f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 3)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x60(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x61(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x62(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x63(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x64(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x65(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x66(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x67(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 4)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x68(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x69(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x6f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 5)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x70(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x71(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x72(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x73(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x74(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x75(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x76(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x77(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 6)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x78(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    cpu.b = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x79(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    cpu.c = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    cpu.d = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    cpu.e = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    cpu.h = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    cpu.l = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    mmu.write(read_hl(cpu), v);
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x7f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    cpu.a = v;
    let z = (v & (1 << 7)) == 0;

    cpu.f = (((cpu.f >> 7) & 1) << 4) | (1 << 5) | (0 << 6) | ((z as u8) << 7);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x80(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 0);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x81(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 0);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x82(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 0);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x83(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 0);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x84(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 0);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x85(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 0);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x86(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 0);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x87(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 0);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x88(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 1);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x89(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 1);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 1);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 1);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 1);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 1);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 1);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x8f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 1);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x90(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 2);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x91(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 2);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x92(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 2);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x93(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 2);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x94(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 2);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x95(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 2);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x96(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 2);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x97(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 2);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x98(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 3);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x99(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 3);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 3);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9b(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 3);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9c(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 3);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9d(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 3);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 3);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0x9f(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 3);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 4);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 4);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 4);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 4);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 4);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 4);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 4);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 4);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 5);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xa9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 5);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xaa(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 5);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xab(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 5);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xac(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 5);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xad(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 5);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xae(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 5);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xaf(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 5);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 6);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 6);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 6);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 6);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 6);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 6);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 6);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 6);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v &= !(1 << 7);
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xb9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v &= !(1 << 7);
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xba(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v &= !(1 << 7);
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v &= !(1 << 7);
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbc(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v &= !(1 << 7);
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbd(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v &= !(1 << 7);
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbe(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v &= !(1 << 7);
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xbf(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v &= !(1 << 7);
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 0;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 0;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 0;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 0;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 0;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 0;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 0;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 0;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 1;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xc9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 1;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xca(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 1;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 1;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcc(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 1;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcd(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 1;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xce(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 1;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xcf(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 1;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 2;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 2;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 2;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 2;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 2;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 2;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 2;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 2;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 3;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xd9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 3;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xda(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 3;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 3;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdc(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 3;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdd(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 3;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xde(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 3;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xdf(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 3;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 4;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 4;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 4;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 4;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 4;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 4;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 4;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 4;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 5;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xe9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 5;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xea(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 5;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xeb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 5;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xec(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 5;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xed(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 5;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xee(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 5;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xef(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 5;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 6;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 6;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 6;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf3(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 6;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf4(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 6;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf5(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 6;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 6;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf7(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 6;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf8(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.b;
    v |= 1 << 7;
    cpu.b = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xf9(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.c;
    v |= 1 << 7;
    cpu.c = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfa(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.d;
    v |= 1 << 7;
    cpu.d = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.e;
    v |= 1 << 7;
    cpu.e = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfc(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.h;
    v |= 1 << 7;
    cpu.h = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfd(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.l;
    v |= 1 << 7;
    cpu.l = v;
}
#[allow(unused_variables)]
pub fn execute_prefixed0xfe(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = mmu.read(read_hl(cpu));
    v |= 1 << 7;
    mmu.write(read_hl(cpu), v);
}
#[allow(unused_variables)]
pub fn execute_prefixed0xff(cpu: &mut Cpu, mmu: &mut Mmu) {
    let mut v = cpu.a;
    v |= 1 << 7;
    cpu.a = v;
}
#[allow(unused_variables)]
pub fn execute_unprefixed0x00(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x01(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    write_bc(cpu, arg);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x02(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_bc(cpu), cpu.a);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x03(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x04(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x05(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x06(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x07(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x08(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write16(arg, cpu.sp);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x09(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(read_bc(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x0f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x10(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x11(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    write_de(cpu, arg);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x12(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_de(cpu), cpu.a);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x13(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x14(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x15(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x16(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x17(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x18(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x19(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(read_de(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x1f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x20(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x21(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    write_hl(cpu, arg);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x22(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.a);
    let v = read_hl(cpu);
    write_hl(cpu, v + 1);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x23(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x24(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x25(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x26(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x27(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x28(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x29(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(read_hl(cpu));
    let v = read_hl(cpu);
    write_hl(cpu, v + 1);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x2f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x30(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x31(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.sp = arg;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x32(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.a);
    let v = read_hl(cpu);
    write_hl(cpu, v - 1);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x33(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x34(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x35(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x36(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), arg as u8);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x37(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x38(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x39(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(read_hl(cpu));
    let v = read_hl(cpu);
    write_hl(cpu, v - 1);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = arg as u8;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x3f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x40(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x41(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x42(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x43(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x44(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x45(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x46(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x47(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.b = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x48(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x49(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x4f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x50(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x51(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x52(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x53(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x54(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x55(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x56(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x57(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.d = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x58(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x59(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x5f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.e = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x60(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x61(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x62(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x63(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x64(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x65(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x66(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x67(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.h = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x68(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x69(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x6f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.l = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x70(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.b);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x71(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.c);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x72(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.d);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x73(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.e);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x74(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.h);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x75(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.l);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x76(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x77(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(read_hl(cpu), cpu.a);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x78(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.b;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x79(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.d;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.e;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.h;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.l;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(read_hl(cpu));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x7f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x80(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x81(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x82(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x83(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x84(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x85(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x86(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x87(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x88(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x89(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x8f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x90(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x91(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x92(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x93(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x94(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x95(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x96(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x97(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x98(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x99(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9a(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9b(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9c(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9d(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9e(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0x9f(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xa9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xaa(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xab(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xac(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xad(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xae(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xaf(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xb9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xba(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xbb(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xbc(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xbd(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xbe(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xbf(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xc9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xca(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xcb(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xcc(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xcd(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xce(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xcf(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xd9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xda(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xdb(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xdc(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xdd(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xde(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xdf(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.c = cpu.a;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xe9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xea(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    mmu.write(arg, cpu.a);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xeb(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xec(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xed(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xee(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xef(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf0(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf1(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf2(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = cpu.c;
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf3(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf4(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf5(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf6(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf7(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf8(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    write_hl(cpu, cpu.sp + (arg as u8 as i8 as i16 as u16));
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xf9(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.sp = read_hl(cpu);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xfa(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    cpu.a = mmu.read(arg);
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xfb(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xfc(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xfd(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xfe(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");

    let z = 0; // todo
}

#[allow(unused_variables)]
pub fn execute_unprefixed0xff(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    panic!("not implemented");
}

pub fn execute_prefixed(cpu: &mut Cpu, mmu: &mut Mmu, opcode: u8) {
    match opcode {
        0x00 => execute_prefixed0x00(cpu, mmu),
        0x01 => execute_prefixed0x01(cpu, mmu),
        0x02 => execute_prefixed0x02(cpu, mmu),
        0x03 => execute_prefixed0x03(cpu, mmu),
        0x04 => execute_prefixed0x04(cpu, mmu),
        0x05 => execute_prefixed0x05(cpu, mmu),
        0x06 => execute_prefixed0x06(cpu, mmu),
        0x07 => execute_prefixed0x07(cpu, mmu),
        0x08 => execute_prefixed0x08(cpu, mmu),
        0x09 => execute_prefixed0x09(cpu, mmu),
        0x0A => execute_prefixed0x0a(cpu, mmu),
        0x0B => execute_prefixed0x0b(cpu, mmu),
        0x0C => execute_prefixed0x0c(cpu, mmu),
        0x0D => execute_prefixed0x0d(cpu, mmu),
        0x0E => execute_prefixed0x0e(cpu, mmu),
        0x0F => execute_prefixed0x0f(cpu, mmu),
        0x10 => execute_prefixed0x10(cpu, mmu),
        0x11 => execute_prefixed0x11(cpu, mmu),
        0x12 => execute_prefixed0x12(cpu, mmu),
        0x13 => execute_prefixed0x13(cpu, mmu),
        0x14 => execute_prefixed0x14(cpu, mmu),
        0x15 => execute_prefixed0x15(cpu, mmu),
        0x16 => execute_prefixed0x16(cpu, mmu),
        0x17 => execute_prefixed0x17(cpu, mmu),
        0x18 => execute_prefixed0x18(cpu, mmu),
        0x19 => execute_prefixed0x19(cpu, mmu),
        0x1A => execute_prefixed0x1a(cpu, mmu),
        0x1B => execute_prefixed0x1b(cpu, mmu),
        0x1C => execute_prefixed0x1c(cpu, mmu),
        0x1D => execute_prefixed0x1d(cpu, mmu),
        0x1E => execute_prefixed0x1e(cpu, mmu),
        0x1F => execute_prefixed0x1f(cpu, mmu),
        0x20 => execute_prefixed0x20(cpu, mmu),
        0x21 => execute_prefixed0x21(cpu, mmu),
        0x22 => execute_prefixed0x22(cpu, mmu),
        0x23 => execute_prefixed0x23(cpu, mmu),
        0x24 => execute_prefixed0x24(cpu, mmu),
        0x25 => execute_prefixed0x25(cpu, mmu),
        0x26 => execute_prefixed0x26(cpu, mmu),
        0x27 => execute_prefixed0x27(cpu, mmu),
        0x28 => execute_prefixed0x28(cpu, mmu),
        0x29 => execute_prefixed0x29(cpu, mmu),
        0x2A => execute_prefixed0x2a(cpu, mmu),
        0x2B => execute_prefixed0x2b(cpu, mmu),
        0x2C => execute_prefixed0x2c(cpu, mmu),
        0x2D => execute_prefixed0x2d(cpu, mmu),
        0x2E => execute_prefixed0x2e(cpu, mmu),
        0x2F => execute_prefixed0x2f(cpu, mmu),
        0x30 => execute_prefixed0x30(cpu, mmu),
        0x31 => execute_prefixed0x31(cpu, mmu),
        0x32 => execute_prefixed0x32(cpu, mmu),
        0x33 => execute_prefixed0x33(cpu, mmu),
        0x34 => execute_prefixed0x34(cpu, mmu),
        0x35 => execute_prefixed0x35(cpu, mmu),
        0x36 => execute_prefixed0x36(cpu, mmu),
        0x37 => execute_prefixed0x37(cpu, mmu),
        0x38 => execute_prefixed0x38(cpu, mmu),
        0x39 => execute_prefixed0x39(cpu, mmu),
        0x3A => execute_prefixed0x3a(cpu, mmu),
        0x3B => execute_prefixed0x3b(cpu, mmu),
        0x3C => execute_prefixed0x3c(cpu, mmu),
        0x3D => execute_prefixed0x3d(cpu, mmu),
        0x3E => execute_prefixed0x3e(cpu, mmu),
        0x3F => execute_prefixed0x3f(cpu, mmu),
        0x40 => execute_prefixed0x40(cpu, mmu),
        0x41 => execute_prefixed0x41(cpu, mmu),
        0x42 => execute_prefixed0x42(cpu, mmu),
        0x43 => execute_prefixed0x43(cpu, mmu),
        0x44 => execute_prefixed0x44(cpu, mmu),
        0x45 => execute_prefixed0x45(cpu, mmu),
        0x46 => execute_prefixed0x46(cpu, mmu),
        0x47 => execute_prefixed0x47(cpu, mmu),
        0x48 => execute_prefixed0x48(cpu, mmu),
        0x49 => execute_prefixed0x49(cpu, mmu),
        0x4A => execute_prefixed0x4a(cpu, mmu),
        0x4B => execute_prefixed0x4b(cpu, mmu),
        0x4C => execute_prefixed0x4c(cpu, mmu),
        0x4D => execute_prefixed0x4d(cpu, mmu),
        0x4E => execute_prefixed0x4e(cpu, mmu),
        0x4F => execute_prefixed0x4f(cpu, mmu),
        0x50 => execute_prefixed0x50(cpu, mmu),
        0x51 => execute_prefixed0x51(cpu, mmu),
        0x52 => execute_prefixed0x52(cpu, mmu),
        0x53 => execute_prefixed0x53(cpu, mmu),
        0x54 => execute_prefixed0x54(cpu, mmu),
        0x55 => execute_prefixed0x55(cpu, mmu),
        0x56 => execute_prefixed0x56(cpu, mmu),
        0x57 => execute_prefixed0x57(cpu, mmu),
        0x58 => execute_prefixed0x58(cpu, mmu),
        0x59 => execute_prefixed0x59(cpu, mmu),
        0x5A => execute_prefixed0x5a(cpu, mmu),
        0x5B => execute_prefixed0x5b(cpu, mmu),
        0x5C => execute_prefixed0x5c(cpu, mmu),
        0x5D => execute_prefixed0x5d(cpu, mmu),
        0x5E => execute_prefixed0x5e(cpu, mmu),
        0x5F => execute_prefixed0x5f(cpu, mmu),
        0x60 => execute_prefixed0x60(cpu, mmu),
        0x61 => execute_prefixed0x61(cpu, mmu),
        0x62 => execute_prefixed0x62(cpu, mmu),
        0x63 => execute_prefixed0x63(cpu, mmu),
        0x64 => execute_prefixed0x64(cpu, mmu),
        0x65 => execute_prefixed0x65(cpu, mmu),
        0x66 => execute_prefixed0x66(cpu, mmu),
        0x67 => execute_prefixed0x67(cpu, mmu),
        0x68 => execute_prefixed0x68(cpu, mmu),
        0x69 => execute_prefixed0x69(cpu, mmu),
        0x6A => execute_prefixed0x6a(cpu, mmu),
        0x6B => execute_prefixed0x6b(cpu, mmu),
        0x6C => execute_prefixed0x6c(cpu, mmu),
        0x6D => execute_prefixed0x6d(cpu, mmu),
        0x6E => execute_prefixed0x6e(cpu, mmu),
        0x6F => execute_prefixed0x6f(cpu, mmu),
        0x70 => execute_prefixed0x70(cpu, mmu),
        0x71 => execute_prefixed0x71(cpu, mmu),
        0x72 => execute_prefixed0x72(cpu, mmu),
        0x73 => execute_prefixed0x73(cpu, mmu),
        0x74 => execute_prefixed0x74(cpu, mmu),
        0x75 => execute_prefixed0x75(cpu, mmu),
        0x76 => execute_prefixed0x76(cpu, mmu),
        0x77 => execute_prefixed0x77(cpu, mmu),
        0x78 => execute_prefixed0x78(cpu, mmu),
        0x79 => execute_prefixed0x79(cpu, mmu),
        0x7A => execute_prefixed0x7a(cpu, mmu),
        0x7B => execute_prefixed0x7b(cpu, mmu),
        0x7C => execute_prefixed0x7c(cpu, mmu),
        0x7D => execute_prefixed0x7d(cpu, mmu),
        0x7E => execute_prefixed0x7e(cpu, mmu),
        0x7F => execute_prefixed0x7f(cpu, mmu),
        0x80 => execute_prefixed0x80(cpu, mmu),
        0x81 => execute_prefixed0x81(cpu, mmu),
        0x82 => execute_prefixed0x82(cpu, mmu),
        0x83 => execute_prefixed0x83(cpu, mmu),
        0x84 => execute_prefixed0x84(cpu, mmu),
        0x85 => execute_prefixed0x85(cpu, mmu),
        0x86 => execute_prefixed0x86(cpu, mmu),
        0x87 => execute_prefixed0x87(cpu, mmu),
        0x88 => execute_prefixed0x88(cpu, mmu),
        0x89 => execute_prefixed0x89(cpu, mmu),
        0x8A => execute_prefixed0x8a(cpu, mmu),
        0x8B => execute_prefixed0x8b(cpu, mmu),
        0x8C => execute_prefixed0x8c(cpu, mmu),
        0x8D => execute_prefixed0x8d(cpu, mmu),
        0x8E => execute_prefixed0x8e(cpu, mmu),
        0x8F => execute_prefixed0x8f(cpu, mmu),
        0x90 => execute_prefixed0x90(cpu, mmu),
        0x91 => execute_prefixed0x91(cpu, mmu),
        0x92 => execute_prefixed0x92(cpu, mmu),
        0x93 => execute_prefixed0x93(cpu, mmu),
        0x94 => execute_prefixed0x94(cpu, mmu),
        0x95 => execute_prefixed0x95(cpu, mmu),
        0x96 => execute_prefixed0x96(cpu, mmu),
        0x97 => execute_prefixed0x97(cpu, mmu),
        0x98 => execute_prefixed0x98(cpu, mmu),
        0x99 => execute_prefixed0x99(cpu, mmu),
        0x9A => execute_prefixed0x9a(cpu, mmu),
        0x9B => execute_prefixed0x9b(cpu, mmu),
        0x9C => execute_prefixed0x9c(cpu, mmu),
        0x9D => execute_prefixed0x9d(cpu, mmu),
        0x9E => execute_prefixed0x9e(cpu, mmu),
        0x9F => execute_prefixed0x9f(cpu, mmu),
        0xA0 => execute_prefixed0xa0(cpu, mmu),
        0xA1 => execute_prefixed0xa1(cpu, mmu),
        0xA2 => execute_prefixed0xa2(cpu, mmu),
        0xA3 => execute_prefixed0xa3(cpu, mmu),
        0xA4 => execute_prefixed0xa4(cpu, mmu),
        0xA5 => execute_prefixed0xa5(cpu, mmu),
        0xA6 => execute_prefixed0xa6(cpu, mmu),
        0xA7 => execute_prefixed0xa7(cpu, mmu),
        0xA8 => execute_prefixed0xa8(cpu, mmu),
        0xA9 => execute_prefixed0xa9(cpu, mmu),
        0xAA => execute_prefixed0xaa(cpu, mmu),
        0xAB => execute_prefixed0xab(cpu, mmu),
        0xAC => execute_prefixed0xac(cpu, mmu),
        0xAD => execute_prefixed0xad(cpu, mmu),
        0xAE => execute_prefixed0xae(cpu, mmu),
        0xAF => execute_prefixed0xaf(cpu, mmu),
        0xB0 => execute_prefixed0xb0(cpu, mmu),
        0xB1 => execute_prefixed0xb1(cpu, mmu),
        0xB2 => execute_prefixed0xb2(cpu, mmu),
        0xB3 => execute_prefixed0xb3(cpu, mmu),
        0xB4 => execute_prefixed0xb4(cpu, mmu),
        0xB5 => execute_prefixed0xb5(cpu, mmu),
        0xB6 => execute_prefixed0xb6(cpu, mmu),
        0xB7 => execute_prefixed0xb7(cpu, mmu),
        0xB8 => execute_prefixed0xb8(cpu, mmu),
        0xB9 => execute_prefixed0xb9(cpu, mmu),
        0xBA => execute_prefixed0xba(cpu, mmu),
        0xBB => execute_prefixed0xbb(cpu, mmu),
        0xBC => execute_prefixed0xbc(cpu, mmu),
        0xBD => execute_prefixed0xbd(cpu, mmu),
        0xBE => execute_prefixed0xbe(cpu, mmu),
        0xBF => execute_prefixed0xbf(cpu, mmu),
        0xC0 => execute_prefixed0xc0(cpu, mmu),
        0xC1 => execute_prefixed0xc1(cpu, mmu),
        0xC2 => execute_prefixed0xc2(cpu, mmu),
        0xC3 => execute_prefixed0xc3(cpu, mmu),
        0xC4 => execute_prefixed0xc4(cpu, mmu),
        0xC5 => execute_prefixed0xc5(cpu, mmu),
        0xC6 => execute_prefixed0xc6(cpu, mmu),
        0xC7 => execute_prefixed0xc7(cpu, mmu),
        0xC8 => execute_prefixed0xc8(cpu, mmu),
        0xC9 => execute_prefixed0xc9(cpu, mmu),
        0xCA => execute_prefixed0xca(cpu, mmu),
        0xCB => execute_prefixed0xcb(cpu, mmu),
        0xCC => execute_prefixed0xcc(cpu, mmu),
        0xCD => execute_prefixed0xcd(cpu, mmu),
        0xCE => execute_prefixed0xce(cpu, mmu),
        0xCF => execute_prefixed0xcf(cpu, mmu),
        0xD0 => execute_prefixed0xd0(cpu, mmu),
        0xD1 => execute_prefixed0xd1(cpu, mmu),
        0xD2 => execute_prefixed0xd2(cpu, mmu),
        0xD3 => execute_prefixed0xd3(cpu, mmu),
        0xD4 => execute_prefixed0xd4(cpu, mmu),
        0xD5 => execute_prefixed0xd5(cpu, mmu),
        0xD6 => execute_prefixed0xd6(cpu, mmu),
        0xD7 => execute_prefixed0xd7(cpu, mmu),
        0xD8 => execute_prefixed0xd8(cpu, mmu),
        0xD9 => execute_prefixed0xd9(cpu, mmu),
        0xDA => execute_prefixed0xda(cpu, mmu),
        0xDB => execute_prefixed0xdb(cpu, mmu),
        0xDC => execute_prefixed0xdc(cpu, mmu),
        0xDD => execute_prefixed0xdd(cpu, mmu),
        0xDE => execute_prefixed0xde(cpu, mmu),
        0xDF => execute_prefixed0xdf(cpu, mmu),
        0xE0 => execute_prefixed0xe0(cpu, mmu),
        0xE1 => execute_prefixed0xe1(cpu, mmu),
        0xE2 => execute_prefixed0xe2(cpu, mmu),
        0xE3 => execute_prefixed0xe3(cpu, mmu),
        0xE4 => execute_prefixed0xe4(cpu, mmu),
        0xE5 => execute_prefixed0xe5(cpu, mmu),
        0xE6 => execute_prefixed0xe6(cpu, mmu),
        0xE7 => execute_prefixed0xe7(cpu, mmu),
        0xE8 => execute_prefixed0xe8(cpu, mmu),
        0xE9 => execute_prefixed0xe9(cpu, mmu),
        0xEA => execute_prefixed0xea(cpu, mmu),
        0xEB => execute_prefixed0xeb(cpu, mmu),
        0xEC => execute_prefixed0xec(cpu, mmu),
        0xED => execute_prefixed0xed(cpu, mmu),
        0xEE => execute_prefixed0xee(cpu, mmu),
        0xEF => execute_prefixed0xef(cpu, mmu),
        0xF0 => execute_prefixed0xf0(cpu, mmu),
        0xF1 => execute_prefixed0xf1(cpu, mmu),
        0xF2 => execute_prefixed0xf2(cpu, mmu),
        0xF3 => execute_prefixed0xf3(cpu, mmu),
        0xF4 => execute_prefixed0xf4(cpu, mmu),
        0xF5 => execute_prefixed0xf5(cpu, mmu),
        0xF6 => execute_prefixed0xf6(cpu, mmu),
        0xF7 => execute_prefixed0xf7(cpu, mmu),
        0xF8 => execute_prefixed0xf8(cpu, mmu),
        0xF9 => execute_prefixed0xf9(cpu, mmu),
        0xFA => execute_prefixed0xfa(cpu, mmu),
        0xFB => execute_prefixed0xfb(cpu, mmu),
        0xFC => execute_prefixed0xfc(cpu, mmu),
        0xFD => execute_prefixed0xfd(cpu, mmu),
        0xFE => execute_prefixed0xfe(cpu, mmu),
        0xFF => execute_prefixed0xff(cpu, mmu),
    }
}

pub fn execute_unprefixed(cpu: &mut Cpu, mmu: &mut Mmu, opcode: u8, arg: u16) {
    match opcode {
        0x00 => execute_unprefixed0x00(cpu, mmu, arg),
        0x01 => execute_unprefixed0x01(cpu, mmu, arg),
        0x02 => execute_unprefixed0x02(cpu, mmu, arg),
        0x03 => execute_unprefixed0x03(cpu, mmu, arg),
        0x04 => execute_unprefixed0x04(cpu, mmu, arg),
        0x05 => execute_unprefixed0x05(cpu, mmu, arg),
        0x06 => execute_unprefixed0x06(cpu, mmu, arg),
        0x07 => execute_unprefixed0x07(cpu, mmu, arg),
        0x08 => execute_unprefixed0x08(cpu, mmu, arg),
        0x09 => execute_unprefixed0x09(cpu, mmu, arg),
        0x0A => execute_unprefixed0x0a(cpu, mmu, arg),
        0x0B => execute_unprefixed0x0b(cpu, mmu, arg),
        0x0C => execute_unprefixed0x0c(cpu, mmu, arg),
        0x0D => execute_unprefixed0x0d(cpu, mmu, arg),
        0x0E => execute_unprefixed0x0e(cpu, mmu, arg),
        0x0F => execute_unprefixed0x0f(cpu, mmu, arg),
        0x10 => execute_unprefixed0x10(cpu, mmu, arg),
        0x11 => execute_unprefixed0x11(cpu, mmu, arg),
        0x12 => execute_unprefixed0x12(cpu, mmu, arg),
        0x13 => execute_unprefixed0x13(cpu, mmu, arg),
        0x14 => execute_unprefixed0x14(cpu, mmu, arg),
        0x15 => execute_unprefixed0x15(cpu, mmu, arg),
        0x16 => execute_unprefixed0x16(cpu, mmu, arg),
        0x17 => execute_unprefixed0x17(cpu, mmu, arg),
        0x18 => execute_unprefixed0x18(cpu, mmu, arg),
        0x19 => execute_unprefixed0x19(cpu, mmu, arg),
        0x1A => execute_unprefixed0x1a(cpu, mmu, arg),
        0x1B => execute_unprefixed0x1b(cpu, mmu, arg),
        0x1C => execute_unprefixed0x1c(cpu, mmu, arg),
        0x1D => execute_unprefixed0x1d(cpu, mmu, arg),
        0x1E => execute_unprefixed0x1e(cpu, mmu, arg),
        0x1F => execute_unprefixed0x1f(cpu, mmu, arg),
        0x20 => execute_unprefixed0x20(cpu, mmu, arg),
        0x21 => execute_unprefixed0x21(cpu, mmu, arg),
        0x22 => execute_unprefixed0x22(cpu, mmu, arg),
        0x23 => execute_unprefixed0x23(cpu, mmu, arg),
        0x24 => execute_unprefixed0x24(cpu, mmu, arg),
        0x25 => execute_unprefixed0x25(cpu, mmu, arg),
        0x26 => execute_unprefixed0x26(cpu, mmu, arg),
        0x27 => execute_unprefixed0x27(cpu, mmu, arg),
        0x28 => execute_unprefixed0x28(cpu, mmu, arg),
        0x29 => execute_unprefixed0x29(cpu, mmu, arg),
        0x2A => execute_unprefixed0x2a(cpu, mmu, arg),
        0x2B => execute_unprefixed0x2b(cpu, mmu, arg),
        0x2C => execute_unprefixed0x2c(cpu, mmu, arg),
        0x2D => execute_unprefixed0x2d(cpu, mmu, arg),
        0x2E => execute_unprefixed0x2e(cpu, mmu, arg),
        0x2F => execute_unprefixed0x2f(cpu, mmu, arg),
        0x30 => execute_unprefixed0x30(cpu, mmu, arg),
        0x31 => execute_unprefixed0x31(cpu, mmu, arg),
        0x32 => execute_unprefixed0x32(cpu, mmu, arg),
        0x33 => execute_unprefixed0x33(cpu, mmu, arg),
        0x34 => execute_unprefixed0x34(cpu, mmu, arg),
        0x35 => execute_unprefixed0x35(cpu, mmu, arg),
        0x36 => execute_unprefixed0x36(cpu, mmu, arg),
        0x37 => execute_unprefixed0x37(cpu, mmu, arg),
        0x38 => execute_unprefixed0x38(cpu, mmu, arg),
        0x39 => execute_unprefixed0x39(cpu, mmu, arg),
        0x3A => execute_unprefixed0x3a(cpu, mmu, arg),
        0x3B => execute_unprefixed0x3b(cpu, mmu, arg),
        0x3C => execute_unprefixed0x3c(cpu, mmu, arg),
        0x3D => execute_unprefixed0x3d(cpu, mmu, arg),
        0x3E => execute_unprefixed0x3e(cpu, mmu, arg),
        0x3F => execute_unprefixed0x3f(cpu, mmu, arg),
        0x40 => execute_unprefixed0x40(cpu, mmu, arg),
        0x41 => execute_unprefixed0x41(cpu, mmu, arg),
        0x42 => execute_unprefixed0x42(cpu, mmu, arg),
        0x43 => execute_unprefixed0x43(cpu, mmu, arg),
        0x44 => execute_unprefixed0x44(cpu, mmu, arg),
        0x45 => execute_unprefixed0x45(cpu, mmu, arg),
        0x46 => execute_unprefixed0x46(cpu, mmu, arg),
        0x47 => execute_unprefixed0x47(cpu, mmu, arg),
        0x48 => execute_unprefixed0x48(cpu, mmu, arg),
        0x49 => execute_unprefixed0x49(cpu, mmu, arg),
        0x4A => execute_unprefixed0x4a(cpu, mmu, arg),
        0x4B => execute_unprefixed0x4b(cpu, mmu, arg),
        0x4C => execute_unprefixed0x4c(cpu, mmu, arg),
        0x4D => execute_unprefixed0x4d(cpu, mmu, arg),
        0x4E => execute_unprefixed0x4e(cpu, mmu, arg),
        0x4F => execute_unprefixed0x4f(cpu, mmu, arg),
        0x50 => execute_unprefixed0x50(cpu, mmu, arg),
        0x51 => execute_unprefixed0x51(cpu, mmu, arg),
        0x52 => execute_unprefixed0x52(cpu, mmu, arg),
        0x53 => execute_unprefixed0x53(cpu, mmu, arg),
        0x54 => execute_unprefixed0x54(cpu, mmu, arg),
        0x55 => execute_unprefixed0x55(cpu, mmu, arg),
        0x56 => execute_unprefixed0x56(cpu, mmu, arg),
        0x57 => execute_unprefixed0x57(cpu, mmu, arg),
        0x58 => execute_unprefixed0x58(cpu, mmu, arg),
        0x59 => execute_unprefixed0x59(cpu, mmu, arg),
        0x5A => execute_unprefixed0x5a(cpu, mmu, arg),
        0x5B => execute_unprefixed0x5b(cpu, mmu, arg),
        0x5C => execute_unprefixed0x5c(cpu, mmu, arg),
        0x5D => execute_unprefixed0x5d(cpu, mmu, arg),
        0x5E => execute_unprefixed0x5e(cpu, mmu, arg),
        0x5F => execute_unprefixed0x5f(cpu, mmu, arg),
        0x60 => execute_unprefixed0x60(cpu, mmu, arg),
        0x61 => execute_unprefixed0x61(cpu, mmu, arg),
        0x62 => execute_unprefixed0x62(cpu, mmu, arg),
        0x63 => execute_unprefixed0x63(cpu, mmu, arg),
        0x64 => execute_unprefixed0x64(cpu, mmu, arg),
        0x65 => execute_unprefixed0x65(cpu, mmu, arg),
        0x66 => execute_unprefixed0x66(cpu, mmu, arg),
        0x67 => execute_unprefixed0x67(cpu, mmu, arg),
        0x68 => execute_unprefixed0x68(cpu, mmu, arg),
        0x69 => execute_unprefixed0x69(cpu, mmu, arg),
        0x6A => execute_unprefixed0x6a(cpu, mmu, arg),
        0x6B => execute_unprefixed0x6b(cpu, mmu, arg),
        0x6C => execute_unprefixed0x6c(cpu, mmu, arg),
        0x6D => execute_unprefixed0x6d(cpu, mmu, arg),
        0x6E => execute_unprefixed0x6e(cpu, mmu, arg),
        0x6F => execute_unprefixed0x6f(cpu, mmu, arg),
        0x70 => execute_unprefixed0x70(cpu, mmu, arg),
        0x71 => execute_unprefixed0x71(cpu, mmu, arg),
        0x72 => execute_unprefixed0x72(cpu, mmu, arg),
        0x73 => execute_unprefixed0x73(cpu, mmu, arg),
        0x74 => execute_unprefixed0x74(cpu, mmu, arg),
        0x75 => execute_unprefixed0x75(cpu, mmu, arg),
        0x76 => execute_unprefixed0x76(cpu, mmu, arg),
        0x77 => execute_unprefixed0x77(cpu, mmu, arg),
        0x78 => execute_unprefixed0x78(cpu, mmu, arg),
        0x79 => execute_unprefixed0x79(cpu, mmu, arg),
        0x7A => execute_unprefixed0x7a(cpu, mmu, arg),
        0x7B => execute_unprefixed0x7b(cpu, mmu, arg),
        0x7C => execute_unprefixed0x7c(cpu, mmu, arg),
        0x7D => execute_unprefixed0x7d(cpu, mmu, arg),
        0x7E => execute_unprefixed0x7e(cpu, mmu, arg),
        0x7F => execute_unprefixed0x7f(cpu, mmu, arg),
        0x80 => execute_unprefixed0x80(cpu, mmu, arg),
        0x81 => execute_unprefixed0x81(cpu, mmu, arg),
        0x82 => execute_unprefixed0x82(cpu, mmu, arg),
        0x83 => execute_unprefixed0x83(cpu, mmu, arg),
        0x84 => execute_unprefixed0x84(cpu, mmu, arg),
        0x85 => execute_unprefixed0x85(cpu, mmu, arg),
        0x86 => execute_unprefixed0x86(cpu, mmu, arg),
        0x87 => execute_unprefixed0x87(cpu, mmu, arg),
        0x88 => execute_unprefixed0x88(cpu, mmu, arg),
        0x89 => execute_unprefixed0x89(cpu, mmu, arg),
        0x8A => execute_unprefixed0x8a(cpu, mmu, arg),
        0x8B => execute_unprefixed0x8b(cpu, mmu, arg),
        0x8C => execute_unprefixed0x8c(cpu, mmu, arg),
        0x8D => execute_unprefixed0x8d(cpu, mmu, arg),
        0x8E => execute_unprefixed0x8e(cpu, mmu, arg),
        0x8F => execute_unprefixed0x8f(cpu, mmu, arg),
        0x90 => execute_unprefixed0x90(cpu, mmu, arg),
        0x91 => execute_unprefixed0x91(cpu, mmu, arg),
        0x92 => execute_unprefixed0x92(cpu, mmu, arg),
        0x93 => execute_unprefixed0x93(cpu, mmu, arg),
        0x94 => execute_unprefixed0x94(cpu, mmu, arg),
        0x95 => execute_unprefixed0x95(cpu, mmu, arg),
        0x96 => execute_unprefixed0x96(cpu, mmu, arg),
        0x97 => execute_unprefixed0x97(cpu, mmu, arg),
        0x98 => execute_unprefixed0x98(cpu, mmu, arg),
        0x99 => execute_unprefixed0x99(cpu, mmu, arg),
        0x9A => execute_unprefixed0x9a(cpu, mmu, arg),
        0x9B => execute_unprefixed0x9b(cpu, mmu, arg),
        0x9C => execute_unprefixed0x9c(cpu, mmu, arg),
        0x9D => execute_unprefixed0x9d(cpu, mmu, arg),
        0x9E => execute_unprefixed0x9e(cpu, mmu, arg),
        0x9F => execute_unprefixed0x9f(cpu, mmu, arg),
        0xA0 => execute_unprefixed0xa0(cpu, mmu, arg),
        0xA1 => execute_unprefixed0xa1(cpu, mmu, arg),
        0xA2 => execute_unprefixed0xa2(cpu, mmu, arg),
        0xA3 => execute_unprefixed0xa3(cpu, mmu, arg),
        0xA4 => execute_unprefixed0xa4(cpu, mmu, arg),
        0xA5 => execute_unprefixed0xa5(cpu, mmu, arg),
        0xA6 => execute_unprefixed0xa6(cpu, mmu, arg),
        0xA7 => execute_unprefixed0xa7(cpu, mmu, arg),
        0xA8 => execute_unprefixed0xa8(cpu, mmu, arg),
        0xA9 => execute_unprefixed0xa9(cpu, mmu, arg),
        0xAA => execute_unprefixed0xaa(cpu, mmu, arg),
        0xAB => execute_unprefixed0xab(cpu, mmu, arg),
        0xAC => execute_unprefixed0xac(cpu, mmu, arg),
        0xAD => execute_unprefixed0xad(cpu, mmu, arg),
        0xAE => execute_unprefixed0xae(cpu, mmu, arg),
        0xAF => execute_unprefixed0xaf(cpu, mmu, arg),
        0xB0 => execute_unprefixed0xb0(cpu, mmu, arg),
        0xB1 => execute_unprefixed0xb1(cpu, mmu, arg),
        0xB2 => execute_unprefixed0xb2(cpu, mmu, arg),
        0xB3 => execute_unprefixed0xb3(cpu, mmu, arg),
        0xB4 => execute_unprefixed0xb4(cpu, mmu, arg),
        0xB5 => execute_unprefixed0xb5(cpu, mmu, arg),
        0xB6 => execute_unprefixed0xb6(cpu, mmu, arg),
        0xB7 => execute_unprefixed0xb7(cpu, mmu, arg),
        0xB8 => execute_unprefixed0xb8(cpu, mmu, arg),
        0xB9 => execute_unprefixed0xb9(cpu, mmu, arg),
        0xBA => execute_unprefixed0xba(cpu, mmu, arg),
        0xBB => execute_unprefixed0xbb(cpu, mmu, arg),
        0xBC => execute_unprefixed0xbc(cpu, mmu, arg),
        0xBD => execute_unprefixed0xbd(cpu, mmu, arg),
        0xBE => execute_unprefixed0xbe(cpu, mmu, arg),
        0xBF => execute_unprefixed0xbf(cpu, mmu, arg),
        0xC0 => execute_unprefixed0xc0(cpu, mmu, arg),
        0xC1 => execute_unprefixed0xc1(cpu, mmu, arg),
        0xC2 => execute_unprefixed0xc2(cpu, mmu, arg),
        0xC3 => execute_unprefixed0xc3(cpu, mmu, arg),
        0xC4 => execute_unprefixed0xc4(cpu, mmu, arg),
        0xC5 => execute_unprefixed0xc5(cpu, mmu, arg),
        0xC6 => execute_unprefixed0xc6(cpu, mmu, arg),
        0xC7 => execute_unprefixed0xc7(cpu, mmu, arg),
        0xC8 => execute_unprefixed0xc8(cpu, mmu, arg),
        0xC9 => execute_unprefixed0xc9(cpu, mmu, arg),
        0xCA => execute_unprefixed0xca(cpu, mmu, arg),
        0xCB => execute_unprefixed0xcb(cpu, mmu, arg),
        0xCC => execute_unprefixed0xcc(cpu, mmu, arg),
        0xCD => execute_unprefixed0xcd(cpu, mmu, arg),
        0xCE => execute_unprefixed0xce(cpu, mmu, arg),
        0xCF => execute_unprefixed0xcf(cpu, mmu, arg),
        0xD0 => execute_unprefixed0xd0(cpu, mmu, arg),
        0xD1 => execute_unprefixed0xd1(cpu, mmu, arg),
        0xD2 => execute_unprefixed0xd2(cpu, mmu, arg),
        0xD3 => execute_unprefixed0xd3(cpu, mmu, arg),
        0xD4 => execute_unprefixed0xd4(cpu, mmu, arg),
        0xD5 => execute_unprefixed0xd5(cpu, mmu, arg),
        0xD6 => execute_unprefixed0xd6(cpu, mmu, arg),
        0xD7 => execute_unprefixed0xd7(cpu, mmu, arg),
        0xD8 => execute_unprefixed0xd8(cpu, mmu, arg),
        0xD9 => execute_unprefixed0xd9(cpu, mmu, arg),
        0xDA => execute_unprefixed0xda(cpu, mmu, arg),
        0xDB => execute_unprefixed0xdb(cpu, mmu, arg),
        0xDC => execute_unprefixed0xdc(cpu, mmu, arg),
        0xDD => execute_unprefixed0xdd(cpu, mmu, arg),
        0xDE => execute_unprefixed0xde(cpu, mmu, arg),
        0xDF => execute_unprefixed0xdf(cpu, mmu, arg),
        0xE0 => execute_unprefixed0xe0(cpu, mmu, arg),
        0xE1 => execute_unprefixed0xe1(cpu, mmu, arg),
        0xE2 => execute_unprefixed0xe2(cpu, mmu, arg),
        0xE3 => execute_unprefixed0xe3(cpu, mmu, arg),
        0xE4 => execute_unprefixed0xe4(cpu, mmu, arg),
        0xE5 => execute_unprefixed0xe5(cpu, mmu, arg),
        0xE6 => execute_unprefixed0xe6(cpu, mmu, arg),
        0xE7 => execute_unprefixed0xe7(cpu, mmu, arg),
        0xE8 => execute_unprefixed0xe8(cpu, mmu, arg),
        0xE9 => execute_unprefixed0xe9(cpu, mmu, arg),
        0xEA => execute_unprefixed0xea(cpu, mmu, arg),
        0xEB => execute_unprefixed0xeb(cpu, mmu, arg),
        0xEC => execute_unprefixed0xec(cpu, mmu, arg),
        0xED => execute_unprefixed0xed(cpu, mmu, arg),
        0xEE => execute_unprefixed0xee(cpu, mmu, arg),
        0xEF => execute_unprefixed0xef(cpu, mmu, arg),
        0xF0 => execute_unprefixed0xf0(cpu, mmu, arg),
        0xF1 => execute_unprefixed0xf1(cpu, mmu, arg),
        0xF2 => execute_unprefixed0xf2(cpu, mmu, arg),
        0xF3 => execute_unprefixed0xf3(cpu, mmu, arg),
        0xF4 => execute_unprefixed0xf4(cpu, mmu, arg),
        0xF5 => execute_unprefixed0xf5(cpu, mmu, arg),
        0xF6 => execute_unprefixed0xf6(cpu, mmu, arg),
        0xF7 => execute_unprefixed0xf7(cpu, mmu, arg),
        0xF8 => execute_unprefixed0xf8(cpu, mmu, arg),
        0xF9 => execute_unprefixed0xf9(cpu, mmu, arg),
        0xFA => execute_unprefixed0xfa(cpu, mmu, arg),
        0xFB => execute_unprefixed0xfb(cpu, mmu, arg),
        0xFC => execute_unprefixed0xfc(cpu, mmu, arg),
        0xFD => execute_unprefixed0xfd(cpu, mmu, arg),
        0xFE => execute_unprefixed0xfe(cpu, mmu, arg),
        0xFF => execute_unprefixed0xff(cpu, mmu, arg),
    }
}
