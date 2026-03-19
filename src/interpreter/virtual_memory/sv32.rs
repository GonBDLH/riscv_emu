use crate::interpreter::{
    bus::Bus,
    riscv_core::{Exception, PrivilegeLevel, RVCore},
};

const PAGESIZE: u32 = 2u32.pow(12);
const LEVELS: u32 = 2;
const PTESIZE: u32 = 4;

#[derive(PartialEq)]
pub enum AccessType {
    Load,
    Store,
    Execute
}

impl AccessType {
    fn get_page_fault_exception(&self) -> Exception {
        match self {
            AccessType::Load => Exception::LoadPageFault,
            AccessType::Store => Exception::StoreAmoPageFault,
            AccessType::Execute => Exception::InstructionPageFault
        }
    }

    fn get_access_fault_exception(&self) -> Exception {
        match self {
            AccessType::Load => Exception::LoadAccessFault,
            AccessType::Store => Exception::StoreAmoAccessFault,
            AccessType::Execute => Exception::InstructionAccessFault
        }
    }
}

pub fn translate_address(
    core: &mut RVCore,
    bus: &Bus,
    virt_address: usize,
    access_type: AccessType
) -> Result<usize, Exception> {
    // TODO

    if core.privilege_level == PrivilegeLevel::Machine {
        // CREO

        return Ok(virt_address);
    }

    let satp = core.control_and_status.get_satp_ref_unchecked();
    let mstatus = core.control_and_status.get_mstatus_ref_unchecked();

    let mut a = satp.get_ppn() * PAGESIZE;
    let mut i = LEVELS as i32 - 1;

    while i >= 0 {
        let va_vpn = virt_address as u32 >> (12 * (i + 1));
        let address = a + va_vpn * PTESIZE;
        let pte = bus
            .read_word(address as usize)?;

        let pte_v = pte & 1;
        let pte_r = (pte >> 1) & 1;
        let pte_w = (pte >> 2) & 1;
        let pte_x = (pte >> 3) & 1;
        let pte_u = (pte >> 4) & 1;
        let pte_g = (pte >> 5) & 1;
        let pte_a = (pte >> 6) & 1;
        let pte_d = (pte >> 7) & 1;

        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            return Err(access_type.get_page_fault_exception());
        }

        let pte_ppn = pte >> 10;

        if pte_r == 1 || pte_x == 1 {
            // TODO STEP 5
            if i > 0 && ((pte_ppn >> (10 * (i - 1))) & 1) != 0 {
                return Err(access_type.get_page_fault_exception());
            }
            
            if core.privilege_level == PrivilegeLevel::User && pte_u == 0 {
                return Err(access_type.get_page_fault_exception());
            }

            if !mstatus.get_mxr() && pte_r == 0 {
                return Err(access_type.get_page_fault_exception());
            }

            if mstatus.get_mxr() && (pte_r == 0 && pte_w == 0) {
                return Err(access_type.get_page_fault_exception());
            }

            match (&access_type, pte_r, pte_w, pte_x) {
                (AccessType::Load, 0, _, _) => return Err(access_type.get_page_fault_exception()),
                (AccessType::Store, _, 0, _) => return Err(access_type.get_page_fault_exception()),
                (AccessType::Execute, _, _, 0) => return Err(access_type.get_page_fault_exception()),
                _ => ()
            }

            // TODO PASO 9
            if (pte_a == 0) || access_type == AccessType::Store && pte_d == 0 {
                // let new_pte = 
            }

        } else {
            a = pte_ppn * PAGESIZE;
            i -= 1;
        }
    }

    // TODO
    Err(access_type.get_access_fault_exception())
}
