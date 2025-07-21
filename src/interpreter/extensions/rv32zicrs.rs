use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, IInstruction, RVCore},
};

pub fn csrrw(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    if instr.rd != 0 {
        let old_csr = core
            .control_and_status
            .read_csr(csr, core.privilege_level)
            .map_err(|_| Exception::IllegalInstruction(instr.data))?;
        core.write_reg(instr.rd, old_csr);
    }

    if instr.rs1 != 0 {
        let rs1_val = core.read_reg(instr.rs1);
        core.control_and_status
            .write_csr(csr, core.privilege_level, rs1_val)?;
    }

    Ok(())
}

pub fn csrrs(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    let old_csr = core
        .control_and_status
        .read_csr(csr, core.privilege_level)
        .map_err(|_| Exception::IllegalInstruction(instr.data))?;
    core.write_reg(instr.rd, old_csr);

    if instr.rs1 != 0 {
        let rs1_val = core.read_reg(instr.rs1);
        let new_csr = old_csr | rs1_val;
        core.control_and_status
            .write_csr(csr, core.privilege_level, new_csr)?;
    }

    Ok(())
}

pub fn csrrc(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    let old_csr = core
        .control_and_status
        .read_csr(csr, core.privilege_level)
        .map_err(|_| Exception::IllegalInstruction(instr.data))?;
    core.write_reg(instr.rd, old_csr);

    if instr.rs1 != 0 {
        let rs1_val = core.read_reg(instr.rs1);
        let new_csr = old_csr & !rs1_val;
        core.control_and_status
            .write_csr(csr, core.privilege_level, new_csr)?;
    }

    Ok(())
}

pub fn csrrwi(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    let imm_val = instr.rs1;

    if instr.rd != 0 {
        let old_csr = core
            .control_and_status
            .read_csr(csr, core.privilege_level)
            .map_err(|_| Exception::IllegalInstruction(instr.data))?;
        core.write_reg(instr.rd, old_csr);
    }

    core.control_and_status
        .write_csr(csr, core.privilege_level, imm_val)?;

    Ok(())
}

pub fn csrrsi(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    let imm_val = instr.rs1;

    let old_csr = core
        .control_and_status
        .read_csr(csr, core.privilege_level)
        .map_err(|_| Exception::IllegalInstruction(instr.data))?;
    core.write_reg(instr.rd, old_csr);

    if imm_val != 0 {
        let new_csr = old_csr | imm_val;
        core.control_and_status
            .write_csr(csr, core.privilege_level, new_csr)?;
    }

    Ok(())
}

pub fn csrrci(instr: &IInstruction, _: &Bus, core: &mut RVCore) -> Result<(), Exception> {
    let csr = instr.imm as usize;
    let imm_val = instr.rs1;

    let old_csr = core
        .control_and_status
        .read_csr(csr, core.privilege_level)
        .map_err(|_| Exception::IllegalInstruction(instr.data))?;
    core.write_reg(instr.rd, old_csr);

    if imm_val != 0 {
        let new_csr = old_csr & !imm_val;
        core.control_and_status
            .write_csr(csr, core.privilege_level, new_csr)?;
    }

    Ok(())
}
