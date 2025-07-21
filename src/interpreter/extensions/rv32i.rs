use crate::interpreter::{
    bus::Bus,
    riscv_core::{
        BInstruction, Exception, IInstruction, JInstruction, RInstruction, RVCore, SInstruction,
        UInstruction,
    },
};

pub fn add(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val.wrapping_add(rs2_val));
    Ok(())
}

pub fn sub(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val.wrapping_sub(rs2_val));
    Ok(())
}

pub fn xor(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val ^ rs2_val);
    Ok(())
}

pub fn or(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val | rs2_val);
    Ok(())
}

pub fn and(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val & rs2_val);
    Ok(())
}

pub fn sll(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val.wrapping_shl(rs2_val));
    Ok(())
}

pub fn srl(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    core.write_reg(instr.rd, rs1_val.wrapping_shr(rs2_val));
    Ok(())
}

pub fn sra(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let val = (rs1_val as i32).wrapping_shr(rs2_val) as u32;

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn slt(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let val = if (rs1_val as i32) < (rs2_val as i32) {
        1
    } else {
        0
    };

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn sltu(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let val = if rs1_val < rs2_val { 1 } else { 0 };

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn addi(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val.wrapping_add(instr.imm));
    Ok(())
}

pub fn xori(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val ^ instr.imm);
    Ok(())
}

pub fn ori(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val | instr.imm);
    Ok(())
}

pub fn andi(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val & instr.imm);
    Ok(())
}

pub fn slli(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val.wrapping_shl(instr.imm));
    Ok(())
}

pub fn srli(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);

    core.write_reg(instr.rd, rs1_val.wrapping_shr(instr.imm));
    Ok(())
}

pub fn srai(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let val = (rs1_val as i32).wrapping_shr(instr.imm) as u32;

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn slti(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let val = if (rs1_val as i32) < (instr.imm as i32) {
        1
    } else {
        0
    };

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn sltui(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let val = if rs1_val < instr.imm { 1 } else { 0 };

    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn lb(instr: &IInstruction, bus: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let addres = rs1_val.wrapping_add(instr.imm);
    let mem_val = bus.read_byte(addres as usize)?;
    let extend_val = (mem_val & 0b10000000 > 0) as u8 * 0xFF;

    let val = u32::from_le_bytes([mem_val, extend_val, extend_val, extend_val]);
    core.write_reg(instr.rd, val);

    Ok(())
}

pub fn lh(instr: &IInstruction, bus: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let addres = rs1_val.wrapping_add(instr.imm);

    let val_1 = bus.read_byte(addres as usize)?;
    let val_2 = bus.read_byte(addres.wrapping_add(1) as usize)?;

    let extend_val = (val_2 & 0b10000000 > 0) as u8 * 0xFF;

    let val = u32::from_le_bytes([val_1, val_2, extend_val, extend_val]);
    core.write_reg(instr.rd, val);

    Ok(())
}

pub fn lw(instr: &IInstruction, bus: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let addres = rs1_val.wrapping_add(instr.imm);

    let val_1 = bus.read_byte(addres as usize)?;
    let val_2 = bus.read_byte(addres.wrapping_add(1) as usize)?;
    let val_3 = bus.read_byte(addres.wrapping_add(2) as usize)?;
    let val_4 = bus.read_byte(addres.wrapping_add(3) as usize)?;

    let val = u32::from_le_bytes([val_1, val_2, val_3, val_4]);
    core.write_reg(instr.rd, val);

    Ok(())
}

pub fn lbu(instr: &IInstruction, bus: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let addres = rs1_val.wrapping_add(instr.imm);

    let val = bus.read_byte(addres as usize)? as u32;
    core.write_reg(instr.rd, val);

    Ok(())
}

pub fn lhu(instr: &IInstruction, bus: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let addres = rs1_val.wrapping_add(instr.imm);

    let val_1 = bus.read_byte(addres as usize)?;
    let val_2 = bus.read_byte(addres.wrapping_add(1) as usize)?;

    let val = u32::from_le_bytes([val_1, val_2, 0x00, 0x00]);
    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn sb(instr: &SInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let addres = rs1_val.wrapping_add(instr.imm);

    bus.write_byte(addres as usize, rs2_val as u8)
}

pub fn sh(instr: &SInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let addres = rs1_val.wrapping_add(instr.imm);

    bus.write_byte(addres as usize, rs2_val as u8)?;
    bus.write_byte(addres.wrapping_add(1) as usize, (rs2_val >> 8) as u8)
}

pub fn sw(instr: &SInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    let addres = rs1_val.wrapping_add(instr.imm);

    bus.write_byte(addres as usize, rs2_val as u8)?;
    bus.write_byte(addres.wrapping_add(1) as usize, (rs2_val >> 8) as u8)?;
    bus.write_byte(addres.wrapping_add(2) as usize, (rs2_val >> 16) as u8)?;
    bus.write_byte(addres.wrapping_add(3) as usize, (rs2_val >> 24) as u8)
}

pub fn beq(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if rs1_val == rs2_val {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn bne(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if rs1_val != rs2_val {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn blt(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if (rs1_val as i32) < (rs2_val as i32) {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn bge(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if (rs1_val as i32) >= (rs2_val as i32) {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn bltu(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if rs1_val < rs2_val {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn bgeu(instr: &BInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);
    if rs1_val >= rs2_val {
        let new_pc = core.pc.wrapping_add(instr.imm);

        core.pc = new_pc.wrapping_sub(4);
    }

    Ok(())
}

pub fn jal(instr: &JInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let new_pc = core.pc.wrapping_add(instr.imm);
    if new_pc % 4 != 0 {
        return Err(Exception::InstructionAddressMisaligned);
    }

    core.write_reg(instr.rd, core.pc.wrapping_add(4));
    core.pc = new_pc.wrapping_sub(4);

    Ok(())
}

pub fn jalr(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let new_pc = rs1_val.wrapping_add(instr.imm) & 0xFFFFFFFE;
    if new_pc % 4 != 0 {
        return Err(Exception::InstructionAddressMisaligned);
    }

    core.write_reg(instr.rd, core.pc.wrapping_add(4));
    core.pc = new_pc.wrapping_sub(4);

    Ok(())
}

pub fn lui(instr: &UInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let val = instr.imm;
    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn auipc(instr: &UInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let val = core.pc.wrapping_add(instr.imm);
    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn fence(_: &IInstruction, _: &Bus, _core: &mut RVCore) -> Result<(), Exception> {
    // TODO
    Ok(())
}
