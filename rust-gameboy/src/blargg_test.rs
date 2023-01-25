use crate::gb::GameBoy;
use rstest::rstest;
use std::path::Path;

#[rstest]
#[case("01-special.gb")]
#[case("01-special.gb")]
#[case("02-interrupts.gb")]
#[case("03-op sp,hl.gb")]
#[case("04-op r,imm.gb")]
#[case("05-op rp.gb")]
#[case("06-ld r,r.gb")]
#[case("07-jr,jp,call,ret,rst.gb")]
#[case("08-misc instrs.gb")]
#[case("09-op r,r.gb")]
#[case("10-bit ops.gb")]
#[case("11-op a,(hl).gb")]
#[case("instr_timing.gb")]
fn test_blargg(#[case] path_name: &str) {
    assert!(blargg_pass(path_name));
}

fn blargg_pass(path_name: &str) -> bool {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let blargg_path = path.join("blarggs").join(path_name);
    let mut gb = GameBoy::from_path(blargg_path.to_path_buf());
    let mut output = String::new();

    gb.cpu.pc = 0x100;
    gb.mmu.disable_boot_rom = true;

    for _ in 0..20000000 {
        gb.cycle();
        if let Some(c) = gb.mmu.extract_serial_data() {
            output.push(c as char);
            if output.ends_with("Passed") {
                return true;
            }
        }
    }
    return false;
}
