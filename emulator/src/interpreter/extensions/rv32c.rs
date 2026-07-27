use crate::interpreter::{
    bus::Bus,
    extensions::{sign_extend16to32, sign_extend32to32},
    riscv_core::{
        CAInstruction, CBInstruction, CIInstruction, CIWInstruction, CJInstruction, CLInstruction,
        CRInstruction, CSInstruction, CSSInstruction, Exception, RVCore, WithErrVal,
    },
    virtual_memory::sv32::{AccessType, translate_address},
};

pub fn c_lwsp(instr: &CIInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let offset4_2 = (instr.imm6_2 >> 2) & 0b111;
    let offset7_6 = instr.imm6_2 & 0b11;
    let offset = (offset7_6 << 6) | (instr.imm12 << 5) | (offset4_2 << 2);

    let address = core.read_reg(2).wrapping_add(offset as u32);
    let phys_address = translate_address(core, bus, address, AccessType::Load, 4)?;

    let val = bus.read_aligned_word(&phys_address).with_err_val(address)?;

    core.write_reg(instr.rd_rs1 as u32, val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_swsp(instr: &CSSInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let offset5_2 = (instr.imm >> 2) & 0b1111;
    let offset7_6 = instr.imm & 0b11;
    let offset = (offset7_6 << 6) | (offset5_2 << 2);

    let address = core.read_reg(2).wrapping_add(offset as u32);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    bus.write_aligned_word(&phys_address, core.read_reg(instr.rs2 as u32))
        .with_err_val(address)?;

    core.inc_pc(2);

    Ok(())
}

pub fn c_lw(instr: &CLInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let offset5_3 = instr.imm12_10;
    let offset2 = (instr.imm6_5 >> 1) & 0b1;
    let offset6 = instr.imm6_5 & 0b1;
    let offset = (offset6 << 6) | (offset5_3 << 3) | (offset2 << 2);

    let address = core
        .read_reg(instr.rs1_p as u32 + 8)
        .wrapping_add(offset as u32);
    let phys_address = translate_address(core, bus, address, AccessType::Load, 4)?;

    let val = bus.read_aligned_word(&phys_address).with_err_val(address)?;

    core.write_reg(instr.rd_p as u32 + 8, val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_sw(instr: &CSInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let offset5_3 = instr.imm12_10;
    let offset2 = (instr.imm6_5 >> 1) & 0b1;
    let offset6 = instr.imm6_5 & 0b1;
    let offset = (offset6 << 6) | (offset5_3 << 3) | (offset2 << 2);

    let address = core
        .read_reg(instr.rs1_p as u32 + 8)
        .wrapping_add(offset as u32);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo, 4)?;

    bus.write_aligned_word(&phys_address, core.read_reg(instr.rs2_p as u32 + 8))
        .with_err_val(address)?;

    core.inc_pc(2);

    Ok(())
}

pub fn c_j(instr: &CJInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let offset11 = (instr.jump_target >> 10) & 0b1;
    let offset4 = (instr.jump_target >> 9) & 0b1;
    let offset9_8 = (instr.jump_target >> 7) & 0b11;
    let offset10 = (instr.jump_target >> 6) & 0b1;
    let offset6 = (instr.jump_target >> 5) & 0b1;
    let offset7 = (instr.jump_target >> 4) & 0b1;
    let offset3_1 = (instr.jump_target >> 1) & 0b111;
    let offset5 = instr.jump_target & 0b1;
    let jump_target = (offset11 << 11)
        | (offset10 << 10)
        | (offset9_8 << 8)
        | (offset7 << 7)
        | (offset6 << 6)
        | (offset5 << 5)
        | (offset4 << 4)
        | (offset3_1 << 1);

    let sign_extended = sign_extend16to32(jump_target, 12);
    let val = core.pc.wrapping_add(sign_extended);

    core.set_pc(val);

    Ok(())
}

pub fn c_jal(instr: &CJInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let offset11 = (instr.jump_target >> 10) & 0b1;
    let offset4 = (instr.jump_target >> 9) & 0b1;
    let offset9_8 = (instr.jump_target >> 7) & 0b11;
    let offset10 = (instr.jump_target >> 6) & 0b1;
    let offset6 = (instr.jump_target >> 5) & 0b1;
    let offset7 = (instr.jump_target >> 4) & 0b1;
    let offset3_1 = (instr.jump_target >> 1) & 0b111;
    let offset5 = instr.jump_target & 0b1;
    let jump_target = (offset11 << 11)
        | (offset10 << 10)
        | (offset9_8 << 8)
        | (offset7 << 7)
        | (offset6 << 6)
        | (offset5 << 5)
        | (offset4 << 4)
        | (offset3_1 << 1);

    let sign_extended = sign_extend16to32(jump_target, 12);
    let val = core.pc.wrapping_add(sign_extended);

    core.write_reg(1, core.pc.wrapping_add(2));

    core.set_pc(val);

    Ok(())
}

pub fn c_jr(instr: &CRInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rd_rs1 as u32) & !0b1;

    core.set_pc(address);

    Ok(())
}

pub fn c_jalr(instr: &CRInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let val = core.read_reg(instr.rd_rs1 as u32) & !0b1;
    let prev_pc = core.pc;

    core.write_reg(1, prev_pc.wrapping_add(2));

    core.set_pc(val);

    Ok(())
}

pub fn c_beqz(instr: &CBInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let offset8 = (instr.offset12_10 >> 2) & 0b1;
    let offset4_3 = instr.offset12_10 & 0b11;
    let offset7_6 = (instr.offset6_2 >> 3) & 0b11;
    let offset2_1 = (instr.offset6_2 >> 1) & 0b11;
    let offset5 = instr.offset6_2 & 0b1;
    let jump_target =
        (offset8 << 8) | (offset7_6 << 6) | (offset5 << 5) | (offset4_3 << 3) | (offset2_1 << 1);

    let sign_extended = sign_extend16to32(jump_target, 9);
    let val = core.pc.wrapping_add(sign_extended);

    if core.read_reg(instr.rd_rs1_p as u32 + 8) == 0 {
        core.set_pc(val);
    } else {
        core.inc_pc(2);
    }

    Ok(())
}

pub fn c_benz(instr: &CBInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let offset8 = (instr.offset12_10 >> 2) & 0b1;
    let offset4_3 = instr.offset12_10 & 0b11;
    let offset7_6 = (instr.offset6_2 >> 3) & 0b11;
    let offset2_1 = (instr.offset6_2 >> 1) & 0b11;
    let offset5 = instr.offset6_2 & 0b1;
    let jump_target =
        (offset8 << 8) | (offset7_6 << 6) | (offset5 << 5) | (offset4_3 << 3) | (offset2_1 << 1);

    let sign_extended = sign_extend16to32(jump_target, 9);
    let val = core.pc.wrapping_add(sign_extended);

    if core.read_reg(instr.rd_rs1_p as u32 + 8) != 0 {
        core.set_pc(val);
    } else {
        core.inc_pc(2);
    }

    Ok(())
}

pub fn c_li(instr: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let imm = (instr.imm12 << 5) | (instr.imm6_2);
    let sign_extended = sign_extend16to32(imm, 6);

    core.write_reg(instr.rd_rs1 as u32, sign_extended);

    core.inc_pc(2);

    Ok(())
}

pub fn c_lui(instr: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let imm = (instr.imm12 << 5) | (instr.imm6_2);
    // let sign_extended = sign_extend16to32(imm, 6);
    let val = (imm as u32) << 12;
    let sign_extended = sign_extend32to32(val, 18);

    core.write_reg(instr.rd_rs1 as u32, sign_extended);

    core.inc_pc(2);

    Ok(())
}

pub fn c_addi(instr: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let imm = (instr.imm12 << 5) | (instr.imm6_2);
    let sign_extended = sign_extend16to32(imm, 6);

    let val = core
        .read_reg(instr.rd_rs1 as u32)
        .wrapping_add(sign_extended);

    core.write_reg(instr.rd_rs1 as u32, val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_nop(_: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    core.inc_pc(2);

    Ok(())
}

pub fn c_addi16sp(instr: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let nzimm9 = instr.imm12;
    let nzimm4 = (instr.imm6_2 >> 4) & 0b1;
    let nzimm6 = (instr.imm6_2 >> 3) & 0b1;
    let nzimm8_7 = (instr.imm6_2 >> 1) & 0b11;
    let nzimm5 = instr.imm6_2 & 0b1;
    let nzimm = (nzimm9 << 9) | (nzimm8_7 << 7) | (nzimm6 << 6) | (nzimm5 << 5) | (nzimm4 << 4);

    let sign_extended = sign_extend16to32(nzimm, 10);

    let val = core.read_reg(2);
    core.write_reg(2, val.wrapping_add(sign_extended));

    core.inc_pc(2);

    Ok(())
}

pub fn c_addi4spn(instr: &CIWInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let nzuimm5_4 = (instr.imm >> 6) & 0b11;
    let nzuimm9_6 = (instr.imm >> 2) & 0b1111;
    let nzuimm2 = (instr.imm >> 1) & 0b1;
    let nzuimm3 = instr.imm & 0b1;
    let nzuimm = ((nzuimm9_6 << 6) | (nzuimm5_4 << 4) | (nzuimm3 << 3) | (nzuimm2 << 2)) as u32;

    core.write_reg(instr.rd_p as u32 + 8, core.read_reg(2).wrapping_add(nzuimm));

    core.inc_pc(2);

    Ok(())
}

pub fn c_slli(instr: &CIInstruction, _: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let shamt = (instr.imm12 << 5) | instr.imm6_2;

    let val = core.read_reg(instr.rd_rs1 as u32);
    core.write_reg(instr.rd_rs1 as u32, val.wrapping_shl(shamt as u32));

    core.inc_pc(2);

    Ok(())
}

pub fn c_srli(instr: &CBInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let shamt5 = (instr.offset12_10 >> 2) & 0b1;
    let shamt = (shamt5 << 5) | instr.offset6_2;

    let val = core.read_reg(instr.rd_rs1_p as u32 + 8);
    core.write_reg(instr.rd_rs1_p as u32 + 8, val.wrapping_shr(shamt as u32));

    core.inc_pc(2);

    Ok(())
}

pub fn c_srai(instr: &CBInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let shamt5 = (instr.offset12_10 >> 2) & 0b1;
    let shamt = (shamt5 << 5) | instr.offset6_2;

    let val = (core.read_reg(instr.rd_rs1_p as u32 + 8) as i32).wrapping_shr(shamt as u32) as u32;

    core.write_reg(instr.rd_rs1_p as u32 + 8, val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_andi(instr: &CBInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let imm5 = (instr.offset12_10 >> 2) & 0b1;
    let imm = (imm5 << 5) | (instr.offset6_2);
    let sign_extended = sign_extend16to32(imm, 6);

    let val = core.read_reg(instr.rd_rs1_p as u32 + 8);

    core.write_reg(instr.rd_rs1_p as u32 + 8, val & sign_extended);

    core.inc_pc(2);

    Ok(())
}

pub fn c_mv(instr: &CRInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let val = core.read_reg(instr.rs2 as u32);
    core.write_reg(instr.rd_rs1 as u32, val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_add(instr: &CRInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let val = core.read_reg(instr.rs2 as u32);
    let prev = core.read_reg(instr.rd_rs1 as u32);

    core.write_reg(instr.rd_rs1 as u32, prev.wrapping_add(val));

    core.inc_pc(2);

    Ok(())
}

pub fn c_and(instr: &CAInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let prev = core.read_reg(instr.rd_rs1_p as u32 + 8);
    let val = core.read_reg(instr.rs2_p as u32 + 8);

    core.write_reg(instr.rd_rs1_p as u32 + 8, prev & val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_or(instr: &CAInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let prev = core.read_reg(instr.rd_rs1_p as u32 + 8);
    let val = core.read_reg(instr.rs2_p as u32 + 8);

    core.write_reg(instr.rd_rs1_p as u32 + 8, prev | val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_xor(instr: &CAInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let prev = core.read_reg(instr.rd_rs1_p as u32 + 8);
    let val = core.read_reg(instr.rs2_p as u32 + 8);

    core.write_reg(instr.rd_rs1_p as u32 + 8, prev ^ val);

    core.inc_pc(2);

    Ok(())
}

pub fn c_sub(instr: &CAInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let prev = core.read_reg(instr.rd_rs1_p as u32 + 8);
    let val = core.read_reg(instr.rs2_p as u32 + 8);

    core.write_reg(instr.rd_rs1_p as u32 + 8, prev.wrapping_sub(val));

    core.inc_pc(2);

    Ok(())
}
