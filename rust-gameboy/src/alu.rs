fn carry(a: i32, b: i32, c: i32, mask: i32) -> bool {
    return (a & mask) + (b & mask) + (c & mask) > mask;
}

fn borrow(a: i32, b: i32, c: i32, mask: i32) -> bool {
    return (a & mask) - (b & mask) - (c & mask) < 0;
}

pub fn add(_a: u8, _b: u8, _c: bool) -> (u8, bool, bool) {
    let (a, b, c) = (_a as i32, _b as i32, _c as i32);
    let r = a + b + c;
    let h = carry(a, b, c, 0xf);
    let newc = carry(a, b, c, 0xff);
    return ((r & 0xff) as u8, h, newc);
}

pub fn sub(_a: u8, _b: u8, _c: bool) -> (u8, bool, bool) {
    let (a, b, c) = (_a as i32, _b as i32, _c as i32);
    let r = a - b - c;
    let h = borrow(a, b, c, 0xf);
    let newc = borrow(a, b, c, 0xff);
    return ((r & 0xff) as u8, h, newc);
}

pub fn add16h(_a: u16, _b: u16) -> (u16, bool, bool) {
    let (a, b) = (_a as i32, _b as i32);
    let r = a + b;
    let h = carry(a, b, 0, 0xfff);
    let newc = carry(a, b, 0, 0xffff);
    return ((r & 0xffff) as u16, h, newc);
}

pub fn add16l(_a: u16, _b: u16) -> (u16, bool, bool) {
    let (a, b) = (_a as i32, _b as i32);
    let r = a + b;
    let h = carry(a, b, 0, 0xf);
    let newc = carry(a, b, 0, 0xff);
    return ((r & 0xffff) as u16, h, newc);
}

pub fn bcd_adjust(v: u8, n: bool, h: bool, c: bool) -> (u8, bool) {
    let fix_l = h | (!n & ((v & 0xf) > 0x9));
    let fix_h = c | (!n & (v > 0x99));
    let fix: u8 = if fix_h { 0x60 } else { 0 } + if fix_l { 0x6 } else { 0 };
    let va: u8 = if n {
        v.wrapping_sub(fix)
    } else {
        v.wrapping_add(fix)
    };
    return (va, fix_h);
}

#[test]
fn test_add() {
    assert_eq!(add(0x10, 0x20, false), (0x30, false, false));
    assert_eq!(add(0x10, 0x20, true), (0x31, false, false));
    assert_eq!(add(0xf0, 0x10, false), (0x0, false, true));
    assert_eq!(add(0xf0, 0x0f, true), (0x0, true, true));
}

#[test]
fn test_sub() {
    assert_eq!(sub(0x20, 0x10, false), (0x10, false, false));
    assert_eq!(sub(0x20, 0x10, true), (0x0f, true, false));
    assert_eq!(sub(0x10, 0x20, false), ((-0x10) as i8 as u8, false, true));
}
