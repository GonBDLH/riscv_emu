use crate::interpreter::{bus::Bus, riscv_core::{AtomicInstruction, Exception, RVCore}};

pub fn lr_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let val = bus.read_word(address)?;
    core.write_reg(instr.rd, val);
    bus.reserve_address(core.read_hartid()?, address);
    
    Ok(())
}

pub fn sc_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let rs2_val = core.read_reg(instr.rs2);

    if bus.is_address_reserved(core.read_hartid()?, address) {
        bus.write_word(address, rs2_val)?;
        core.write_reg(instr.rd, 0);
    } else {
        core.write_reg(instr.rd, 1);
    }

    bus.invalidate_reserved_address(core.read_hartid()?);

    Ok(())
}

pub fn amoswap_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoadd_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp.wrapping_add(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoand_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp & core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoor_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp | core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoxor_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp ^ core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomax_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, (tmp as i32).max(core.read_reg(instr.rs2) as i32) as u32)?;

    core.write_reg(instr.rd, tmp);
    
    Ok(())
}

pub fn amomin_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, (tmp as i32).min(core.read_reg(instr.rs2) as i32) as u32)?;

    core.write_reg(instr.rd, tmp);
    
    Ok(())
}

pub fn amomaxu_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp.max(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);
    
    Ok(())
}

pub fn amominu_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1) as usize;

    let tmp = bus.read_word(address)?;

    bus.write_word(address, tmp.min(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);
    
    Ok(())
}
