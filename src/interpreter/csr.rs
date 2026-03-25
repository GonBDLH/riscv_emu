use crate::interpreter::riscv_core::{Exception, ExceptionType, PrivilegeLevel};
use bitfield::bitfield;

/*
 * MACHINE CRS
 */
// INFORMATION
const MVENDORID: usize = 0xF11;
const MARCHID: usize = 0xF12;
const MIMPID: usize = 0xF13;
pub const MHARTID: usize = 0xF14;
// TRAP SETUP
pub const MSTATUS: usize = 0x300;
const MSTATUS_MASK: u32 = 0x81FFFFEA;
const MISA: usize = 0x301;
const MISA_MASK_WRITE: u32 = 0b00000000000101000001000100000001;
pub const MEDELEG: usize = 0x302;
pub const MIDELEG: usize = 0x303;
const MIE: usize = 0x304;
const MIE_MASK: u32 = 0xFFFF2AAA;
pub const MTVEC: usize = 0x305;
const MCOUNTEREN: usize = 0x306;
pub const MSTATUSH: usize = 0x310;
const MSTATUSH_MASK: u32 = 0x6F0;
pub const MEDELEGH: usize = 0x312;
// TRAP HANDLING
const MSCRATCH: usize = 0x340;
pub const MEPC: usize = 0x341;
pub const MCAUSE: usize = 0x342;
pub const MTVAL: usize = 0x343;
const MIP: usize = 0x344;
const MIP_MASK: u32 = 0xFFFF2AAA;
const MTINST: usize = 0x34A;
const MTVAL2: usize = 0x34B;
// COUNTER/TIMERS
const MCYCLE: usize = 0xB00;
const MINSTRET: usize = 0xB02;
const MHPCOUNTER3: usize = 0xB03; // MAX 31 (-3)
const MCYCLEH: usize = 0xB80;
const MINSTRETH: usize = 0xB82;
const MHPCOUNTERH3: usize = 0xB83; // MAX 31 (-3)

// DEBUG
const TSELECT: usize = 0x7A0;

/*
 * SUPERVISOR
 */
pub const SSTATUS: usize = 0x100;
const SSTATUS_MASK: u32 = 0x818DE762;
const SIE: usize = 0x104;
const SIE_MASK: u32 = 0xFFFF2222;
pub const STVEC: usize = 0x105;
const SCOUNTEREN: usize = 0x106;
const SSCRATCH: usize = 0x140;
pub const SEPC: usize = 0x141;
pub const SCAUSE: usize = 0x142;
pub const STVAL: usize = 0x143;
pub const SIP: usize = 0x144;
const SIP_MASK: u32 = 0xFFFF2222;

pub const SATP: usize = 0x180;

pub struct ControlAndStatus {
    csrs: [u32; 4096],
    // mstatus: MStatus,
    // satp: Satp32,

    minstret_loaded: bool,
}

impl ControlAndStatus {
    pub fn new(hart_id: u32) -> Self {
        let mut csrs = [0u32; 4096];

        let mut misa = 0u32;
        misa |= 0b01 << 30; // 32 bits
        misa |= 1 << 18; // Supervisor ISA
        misa |= 1 << 12; // RV31M
        misa |= 1 << 8; // RV32I
        misa |= 1; // RV32A

        csrs[MISA] = misa;

        csrs[MHARTID] = hart_id;

        Self {
            csrs,
            minstret_loaded: false,
        }
    }

    pub fn read_csr(&self, csr: usize, priv_level: PrivilegeLevel) -> Result<u32, Exception> {
        let csr_priv = (csr >> 8) & 0b11;

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::new(ExceptionType::IllegalInstruction, 0));
        }

        // let val = self.csrs[csr as usize];
        let val = match csr {
            // MSTATUS => self.mstatus.0,
            MSTATUS => self.csrs[MSTATUS] & MSTATUS_MASK,
            MSTATUSH => self.csrs[MSTATUSH] & MSTATUSH_MASK,
            MIP => self.csrs[MIP] & MIP_MASK,
            MIE => self.csrs[MIE] & MIE_MASK,
            TSELECT => u32::MAX, // TODO Cambiar si se incluye el modo debug

            SSTATUS => self.csrs[MSTATUS] & SSTATUS_MASK,
            SIP => self.csrs[MIP] & SIP_MASK,
            SIE => self.csrs[MIE] & SIE_MASK,

            SATP => {
                let mstatus = self.read_mstatus_unchecked();

                if mstatus.get_tvm() {
                    return Err(Exception::new(ExceptionType::IllegalInstruction, 0));
                }

                self.csrs[SATP]
            }

            _ => self.csrs[csr],
        };

        Ok(val)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_mstatus_unchecked(&self) -> MStatus {
        MStatus(self.csrs[MSTATUS] & MSTATUS_MASK)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_sstatus_unchecked(&self) -> SStatus {
        SStatus(self.csrs[MSTATUS] & SSTATUS_MASK)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_satp_unchecked(&self) -> Satp32 {
        Satp32(self.csrs[SATP])
    }

    pub fn read_mstatus(&self, priv_level: PrivilegeLevel) -> Result<MStatus, Exception> {
        let csr = self.read_csr(MSTATUS, priv_level)?;

        Ok(MStatus(csr))
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_sstatus(&self, priv_level: PrivilegeLevel) -> Result<SStatus, Exception> {
        let csr = self.read_csr(SSTATUS, priv_level)?;

        Ok(SStatus(csr))
    }

    pub fn write_csr(
        &mut self,
        csr: usize,
        priv_level: PrivilegeLevel,
        val: u32,
    ) -> Result<(), Exception> {
        let csr_rw = (csr >> 10) & 0b11;
        let csr_priv = (csr >> 8) & 0b11;

        if csr_rw == 0b11 {
            // READ-ONLY, RAISE EXCEPTION
            return Err(Exception::new(ExceptionType::IllegalInstruction, 0));
        }

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::new(ExceptionType::IllegalInstruction, 0));
        }

        match csr {
            MSTATUS => self.csrs[MSTATUS] = MSTATUS_MASK & val,
            MINSTRET | MINSTRETH => {
                self.minstret_loaded = true;
                self.csrs[csr] = val;
            }
            MEPC => self.csrs[MEPC] = val & 0xFFFFFFFC,
            MISA => self.csrs[MISA] = (self.csrs[MISA] & !MISA_MASK_WRITE) | (val & MISA_MASK_WRITE),

            SSTATUS => self.csrs[MSTATUS] = (self.csrs[MSTATUS] & !SSTATUS_MASK) | (val & SSTATUS_MASK),

            SATP => {
                let mstatus = self.read_mstatus_unchecked();

                if mstatus.get_tvm() {
                    return Err(Exception::new(ExceptionType::IllegalInstruction, 0));
                }

                self.csrs[SATP] = val;
            }

            _ => self.csrs[csr] = val,
        }

        Ok(())
    }

    pub fn increment_minstret(&mut self) {
        if self.minstret_loaded {
            self.minstret_loaded = false;
            return;
        }

        let minstret = self.csrs[MINSTRET];
        let minstreth = self.csrs[MINSTRETH];

        let minsret_64 = (minstret as u64) + ((minstreth as u64) << 32);
        let new_minstret = minsret_64.wrapping_add(1);

        self.csrs[MINSTRET] = new_minstret as u32;
        self.csrs[MINSTRETH] = (new_minstret >> 32) as u32;
    }
}

bitfield! {
    pub struct MStatus(u32);
    _, _: 0; // WPRI 0
    pub get_sie, set_sie: 1;
    _, _: 2; // WPRI 2
    pub get_mie, set_mie: 3;
    _, _: 4; // WPRI 4
    pub get_spie, set_spie: 5;
    pub get_ube, set_ube: 6;
    pub get_mpie, set_mpie: 7;
    pub get_spp, set_spp: 8;
    pub get_vs, set_vs: 10, 9;
    pub get_mpp, set_mpp: 12, 11;
    pub get_fs, set_fs: 14, 13;
    pub get_xs, set_xs: 16, 15;
    pub get_mprv, set_mprv: 17;
    pub get_sum, set_sum: 18;
    pub get_mxr, set_mxr: 19;
    pub get_tvm, set_tvm: 20;
    pub get_tw, set_tw: 21;
    pub get_tsr, set_tsr: 22;
    pub get_spelp, set_spelp: 23;
    pub get_sdt, set_sdt: 24;
    _, _: 30, 25; // WPRI 25-30
    pub get_sd, set_sd: 31;
}

bitfield! {
    pub struct Satp32(u32);
    u32;
    pub get_ppn, set_ppn: 21, 0;
    pub get_asid, set_asid: 30, 22;
    pub get_mode, set_mode: 31
}

bitfield! {
    pub struct SStatus(u32);
    _, _: 0; // WPRI 0
    pub get_sie, set_sie: 1;
    _, _: 4, 2; // WPRI 2
    pub get_spie, set_spie: 5;
    pub get_ube, set_ube: 6;
    _, _: 7; // WPRI 3
    pub get_spp, set_spp: 8;
    pub get_vs, set_vs: 10, 9;
    _, _: 12, 11; // WPRI 4
    pub get_fs, set_fs: 14, 13;
    pub get_xs, set_xs: 16, 15;
    _, _: 17;
    pub get_sum, set_sum: 18;
    pub get_mxr, set_mxr: 19;
    pub _, _: 22, 20;
    pub get_spelp, set_spelp: 23;
    pub get_sdt, set_sdt: 24;
    _, _: 30, 25; // WPRI 25-30
    pub get_sd, set_sd: 31;
}
