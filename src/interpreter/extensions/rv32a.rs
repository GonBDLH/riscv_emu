use crate::interpreter::{
    bus::Bus,
    riscv_core::{AtomicInstruction, Exception, RVCore}, virtual_memory::sv32::{AccessType, translate_address},
};

pub fn lr_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::Load)?;

    let val = bus.read_aligned_word(&phys_address)?;
    core.write_reg(instr.rd, val);
    bus.reserve_address(core.get_hartid(), phys_address.0 as usize);

    Ok(())
}

pub fn sc_w(instr: &AtomicInstruction, bus: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let rs2_val = core.read_reg(instr.rs2);

    if bus.is_address_reserved(core.get_hartid(), phys_address.0 as usize) {
        bus.write_aligned_word(&phys_address, rs2_val)?;
        core.write_reg(instr.rd, 0);
    } else {
        core.write_reg(instr.rd, 1);
    }

    bus.invalidate_reserved_address(core.get_hartid(), phys_address.0 as usize);

    Ok(())
}

pub fn amoswap_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoadd_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp.wrapping_add(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoand_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp & core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoor_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp | core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amoxor_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp ^ core.read_reg(instr.rs2))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomax_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(
        &phys_address,
        (tmp as i32).max(core.read_reg(instr.rs2) as i32) as u32,
    )?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomin_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(
        &phys_address,
        (tmp as i32).min(core.read_reg(instr.rs2) as i32) as u32,
    )?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amomaxu_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp.max(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}

pub fn amominu_w(
    instr: &AtomicInstruction,
    bus: &mut Bus,
    core: &mut RVCore,
) -> Result<(), Exception> {
    let address = core.read_reg(instr.rs1);
    let phys_address = translate_address(core, bus, address, AccessType::StoreAmo)?;

    let tmp = bus.read_aligned_word(&phys_address)?;

    bus.write_aligned_word(&phys_address, tmp.min(core.read_reg(instr.rs2)))?;

    core.write_reg(instr.rd, tmp);

    Ok(())
}
