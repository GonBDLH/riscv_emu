pub mod rv32a;
pub mod rv32i;
pub mod rv32m;
pub mod rv32c;
pub mod rv32privileged;
pub mod rv32zicrs;
pub mod rv32zifencei;

fn sign_extend16to32(val: u16, bits: u32) -> u32 {
    let mask = (1u32 << bits) - 1;
    let val = (val as u32) & mask;
    let sign_bit = 1u32 << (bits - 1);

    if val & sign_bit != 0 {
        val | (!mask)
    } else {
        val
    }
}

fn sign_extend32to32(val: u32, bits: u32) -> u32 {
    let mask = (1u32 << bits) - 1;
    let val = val & mask;
    let sign_bit = 1u32 << (bits - 1);

    if val & sign_bit != 0 {
        val | (!mask)
    } else {
        val
    }
}