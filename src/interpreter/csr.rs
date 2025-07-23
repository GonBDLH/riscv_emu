use crate::interpreter::riscv_core::{Exception, PrivilegeLevel};
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
pub const MEDELEG: usize = 0x302;
pub const MIDELEG: usize = 0x303;
const MIE: usize = 0x304;
const MIE_MASK: u32 = 0xFFFF2AAA;
pub const MTVEC: usize = 0x305;
const MCOUNTEREN: usize = 0x306;
pub const MSTATUSH: usize = 0x310;
const MSTATUSH_MASK: u32 = 0x6F0;
const MEDELEGH: usize = 0x312;
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
const SSTATUS: usize = 0x100;
const SSTATUS_MASK: u32 = 0x818DE762;
const SIE: usize = 0x104;
const SIE_MASK: u32 = 0xFFFF2222;
const SIP: usize = 0x144;
const SIP_MASK: u32 = 0xFFFF2222;
const STVEC: usize = 0x105;
const SCOUNTEREN: usize = 0x106;

pub struct ControlAndStatus {
    csrs: [u32; 4096],
    mstatus: MStatus,

    minstret_loaded: bool
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

        let mut mstatus = MStatus(0);
        mstatus.set_mpp(0b11);

        csrs[MHARTID] = hart_id;

        Self { csrs, mstatus, minstret_loaded: false }
    }

    pub fn read_csr(&self, csr: usize, priv_level: PrivilegeLevel) -> Result<u32, Exception> {
        let csr_priv = (csr >> 8) & 0b11;

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::IllegalInstruction(0));
        }

        // let val = self.csrs[csr as usize];
        let val = match csr {
            MSTATUS => self.mstatus.0,
            MSTATUSH => self.csrs[MSTATUSH] & MSTATUSH_MASK,
            MIP => self.csrs[MIP] & MIP_MASK,
            MIE => self.csrs[MIE] & MIE_MASK,
            MHARTID => self.read_hartid(),
            TSELECT => u32::MAX,    // TODO Cambiar si se incluye el modo debug

            SSTATUS => self.csrs[MSTATUS] & SSTATUS_MASK,
            SIP => self.csrs[MIP] & SIP_MASK,
            SIE => self.csrs[MIE] & SIE_MASK,
            _ => self.csrs[csr],
        };

        Ok(val)
    }

    // ATENCION SOLO USAR EN TRAPS
    pub fn read_csr_unchecked(&self, csr: usize) -> u32 {
        match csr {
            MSTATUS => self.mstatus.0,
            MSTATUSH => self.csrs[MSTATUSH] & MSTATUSH_MASK,
            MIP => self.csrs[MIP] & MIP_MASK,
            MIE => self.csrs[MIE] & MIE_MASK,

            SSTATUS => self.csrs[MSTATUS] & SSTATUS_MASK,
            SIP => self.csrs[MIP] & SIP_MASK,
            SIE => self.csrs[MIE] & SIE_MASK,
            _ => self.csrs[csr],
        }
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
            return Err(Exception::IllegalInstruction(0));
        }

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::IllegalInstruction(0));
        }

        match csr {
            MSTATUS => self.mstatus.0 = val,
            MINSTRET | MINSTRETH => {
                self.minstret_loaded = true;
                self.csrs[csr] = val;
            },
            MEPC => self.csrs[MEPC] = val & 0xFFFFFFFC,
            _ => self.csrs[csr] = val,
        }

        Ok(())
    }

    pub fn write_csr_unchecked(&mut self, csr: usize, val: u32) {
        match csr {
            MSTATUS => self.mstatus.0 = val,
            _ => self.csrs[csr] = val,
        }
    }

    pub fn get_mstatus_ref(&self, priv_level: PrivilegeLevel) -> Result<&MStatus, Exception> {
        let csr_priv = (MSTATUS >> 8) & 0b11;

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::IllegalInstruction(0));
        }

        Ok(&self.mstatus)
    }

    pub fn get_mstatus_mut_ref(
        &mut self,
        priv_level: PrivilegeLevel,
    ) -> Result<&mut MStatus, Exception> {
        let csr_priv = (MSTATUS >> 8) & 0b11;

        if csr_priv > priv_level as usize {
            // BAD PRIVILEGE LEVEL, RAISE EXCEPTION
            return Err(Exception::IllegalInstruction(0));
        }

        Ok(&mut self.mstatus)
    }

    pub fn read_hartid(&self) -> u32 {
        self.csrs[MHARTID]
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
