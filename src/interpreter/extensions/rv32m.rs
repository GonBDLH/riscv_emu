use crate::interpreter::riscv_core::{Exception, RInstruction, RVCore};

pub fn mul(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as i32;
    let rs2_val = core.read_reg(instr.rs2) as i32;

    let val = rs1_val.wrapping_mul(rs2_val);
    core.write_reg(instr.rd, val as u32);
    Ok(())
}

pub fn mulh(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as i32 as i64;
    let rs2_val = core.read_reg(instr.rs2) as i32 as i64;

    let val = rs1_val.wrapping_mul(rs2_val);
    core.write_reg(instr.rd, (val >> 32) as u32);
    Ok(())
}

pub fn mulhsu(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as i32 as i64;
    let rs2_val = core.read_reg(instr.rs2) as i64;

    let val = rs1_val.wrapping_mul(rs2_val);
    core.write_reg(instr.rd, (val >> 32) as u32);
    Ok(())
}

pub fn mulhu(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as u64;
    let rs2_val = core.read_reg(instr.rs2) as u64;

    let val = rs1_val.wrapping_mul(rs2_val);
    core.write_reg(instr.rd, (val >> 32) as u32);
    Ok(())
}

pub fn div(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as i32;
    let rs2_val = core.read_reg(instr.rs2) as i32;

    let val = if rs2_val == 0 {
        -1
    } else if rs1_val == i32::MIN && rs2_val == -1 {
        i32::MIN
    } else {
        rs1_val / rs2_val
    };

    core.write_reg(instr.rd, val as u32);
    Ok(())
}

pub fn divu(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    let val = if rs2_val != 0 {
        rs1_val / rs2_val
    } else {
        u32::MAX
    };
    
    core.write_reg(instr.rd, val);
    Ok(())
}

pub fn rem(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1) as i32;
    let rs2_val = core.read_reg(instr.rs2) as i32;

    let val = if rs2_val != 0 {
        rs1_val % rs2_val
    } else {
        rs1_val
    };
    
    core.write_reg(instr.rd, val as u32);
    Ok(())
}

pub fn remu(instr: &RInstruction, core: &mut RVCore) -> Result<(), Exception> {
    let rs1_val = core.read_reg(instr.rs1);
    let rs2_val = core.read_reg(instr.rs2);

    let val = if rs2_val != 0 {
        rs1_val % rs2_val
    } else {
        rs1_val
    };
    
    core.write_reg(instr.rd, val);
    Ok(())
}