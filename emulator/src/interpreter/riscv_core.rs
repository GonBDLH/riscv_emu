use crate::interpreter::bus::Bus;
use crate::interpreter::csr::*;
use crate::interpreter::extensions::rv32a::*;
use crate::interpreter::extensions::rv32c::*;
use crate::interpreter::extensions::rv32i::*;
use crate::interpreter::extensions::rv32m::*;
use crate::interpreter::extensions::rv32privileged::*;
use crate::interpreter::extensions::rv32zicrs::*;
use crate::interpreter::extensions::rv32zifencei::fence_i;
use crate::interpreter::virtual_memory::sv32::AccessType;
use crate::interpreter::virtual_memory::sv32::PhysicalAddress;

pub struct RVCore {
    // x0/zero -> Siempre 0
    pub registers: [u32; 32],
    pc: u32,

    pub control_and_status: ControlAndStatus,

    pub privilege_level: PrivilegeLevel,

    hart_id: u32,
    pub stalled: bool,

    pub new_pc: u32,
}

impl Default for RVCore {
    fn default() -> Self {
        Self {
            registers: [0u32; 32],
            pc: 0x80000000,
            control_and_status: ControlAndStatus::new(0),

            privilege_level: PrivilegeLevel::Machine,

            hart_id: 0,
            stalled: false,

            new_pc: 0
        }
    }
}

impl RVCore {
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn decode32(&self, instr: u32) -> Option<InstructionType> {
        let opcode = instr & 0x7F;

        match opcode {
            0b0000011 => self.try_decode_load_instr(instr).map(InstructionType::I),
            0b0010011 => self.try_decode_i_instr(instr).map(InstructionType::I),
            0b0010111 => self.try_decode_auipc_instr(instr).map(InstructionType::U),
            0b0100011 => self.try_decode_s_instr(instr).map(InstructionType::S),
            0b0101111 => self
                .try_decode_atomic_instr(instr)
                .map(InstructionType::Atomic),
            0b0110011 => self.try_decode_r_instr(instr).map(InstructionType::R),
            0b0110111 => self.try_decode_lui_instr(instr).map(InstructionType::U),
            0b1100011 => self.try_decode_b_instr(instr).map(InstructionType::B),
            0b1100111 => self.try_decode_jalr_instr(instr).map(InstructionType::I),
            0b1101111 => self.try_decode_j_instr(instr).map(InstructionType::J),
            0b1110011 => self.try_decode_system_instr(instr).map(InstructionType::I),
            0b0001111 => self.try_decode_fence_instr(instr).map(InstructionType::I),

            _ => None,
        }
    }

    pub fn decode16(&mut self, instr: u16) -> Option<InstructionType> {
        let opcode = instr & 0b11;

        match opcode {
            0b00 => self.try_decode_c0(instr),
            0b01 => self.try_decode_c1(instr),
            0b10 => self.try_decode_c2(instr),
            _ => unreachable!(),
        }
    }

    pub fn read_reg(&self, reg: u32) -> u32 {
        if reg < 32 {
            self.registers[reg as usize]
        } else {
            unreachable!("MAL REGISTRO")
        }
    }

    pub fn write_reg(&mut self, reg: u32, val: u32) {
        if reg > 0 && reg < 32 {
            self.registers[reg as usize] = val;
        } else if reg >= 32 {
            unreachable!("MAL REGISTRO")
        }
    }

    fn try_decode_r_instr(&self, instr: u32) -> Option<RInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let funct7 = (instr >> 25) & 0x7F;

        match (funct3, funct7) {
            // ADD
            (0x0, 0x00) => Some(RInstruction::new(rs1, rs2, rd, add)),
            // MUL
            (0x0, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mul)),
            // SUB
            (0x0, 0x20) => Some(RInstruction::new(rs1, rs2, rd, sub)),
            // MULH
            (0x1, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulh)),
            // MULSU
            (0x2, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulhsu)),
            // MULSU
            (0x3, 0x01) => Some(RInstruction::new(rs1, rs2, rd, mulhu)),
            // XOR
            (0x4, 0x00) => Some(RInstruction::new(rs1, rs2, rd, xor)),
            // DIV
            (0x4, 0x01) => Some(RInstruction::new(rs1, rs2, rd, div)),
            // DIVU
            (0x5, 0x01) => Some(RInstruction::new(rs1, rs2, rd, divu)),
            // OR
            (0x6, 0x00) => Some(RInstruction::new(rs1, rs2, rd, or)),
            // REM
            (0x6, 0x01) => Some(RInstruction::new(rs1, rs2, rd, rem)),
            // AND
            (0x7, 0x00) => Some(RInstruction::new(rs1, rs2, rd, and)),
            // REMU
            (0x7, 0x01) => Some(RInstruction::new(rs1, rs2, rd, remu)),
            // SLL
            (0x1, 0x00) => Some(RInstruction::new(rs1, rs2, rd, sll)),
            // SRL
            (0x5, 0x00) => Some(RInstruction::new(rs1, rs2, rd, srl)),
            // SRA
            (0x5, 0x20) => Some(RInstruction::new(rs1, rs2, rd, sra)),
            // SLT
            (0x2, 0x00) => Some(RInstruction::new(rs1, rs2, rd, slt)),
            // SLTU
            (0x3, 0x00) => Some(RInstruction::new(rs1, rs2, rd, sltu)),

            _ => None,
        }
    }

    fn try_decode_i_instr(&self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm_val = get_i_imm_val(instr);

        match funct3 {
            // ADDI
            0x0 => Some(IInstruction::new(instr, rs1, imm_val, rd, addi)),
            // XORI
            0x4 => Some(IInstruction::new(instr, rs1, imm_val, rd, xori)),
            // ORI
            0x6 => Some(IInstruction::new(instr, rs1, imm_val, rd, ori)),
            // ANDI
            0x7 => Some(IInstruction::new(instr, rs1, imm_val, rd, andi)),
            // SLLI
            0x1 => {
                if (imm_val >> 5) & 0x7F == 0x00 {
                    Some(IInstruction::new(instr, rs1, imm_val & 0x1F, rd, slli))
                } else {
                    None
                }
            }
            0x5 => {
                if (imm_val >> 5) & 0x7F == 0x00 {
                    // SRLI
                    Some(IInstruction::new(instr, rs1, imm_val & 0x1F, rd, srli))
                } else if (imm_val >> 5) & 0x7F == 0x20 {
                    // SRAI
                    Some(IInstruction::new(instr, rs1, imm_val & 0x1F, rd, srai))
                } else {
                    None
                }
            }
            // SLTI
            0x2 => Some(IInstruction::new(instr, rs1, imm_val, rd, slti)),
            // SLTIU
            0x3 => Some(IInstruction::new(instr, rs1, imm_val, rd, sltui)),

            _ => None,
        }
    }

    fn try_decode_load_instr(&self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;

        let imm_val = get_i_imm_val(instr);

        match funct3 {
            0x0 => Some(IInstruction::new(instr, rs1, imm_val, rd, lb)),
            0x1 => Some(IInstruction::new(instr, rs1, imm_val, rd, lh)),
            0x2 => Some(IInstruction::new(instr, rs1, imm_val, rd, lw)),
            0x4 => Some(IInstruction::new(instr, rs1, imm_val, rd, lbu)),
            0x5 => Some(IInstruction::new(instr, rs1, imm_val, rd, lhu)),

            _ => None,
        }
    }

    fn try_decode_s_instr(&self, instr: u32) -> Option<SInstruction> {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;

        let imm_val = get_s_imm_val(instr);

        match funct3 {
            0x0 => Some(SInstruction::new(rs1, rs2, imm_val, sb)),
            0x1 => Some(SInstruction::new(rs1, rs2, imm_val, sh)),
            0x2 => Some(SInstruction::new(rs1, rs2, imm_val, sw)),

            _ => None,
        }
    }

    fn try_decode_b_instr(&self, instr: u32) -> Option<BInstruction> {
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;

        let imm_val = get_b_imm_val(instr);

        match funct3 {
            0x0 => Some(BInstruction::new(rs1, rs2, imm_val, beq)),
            0x1 => Some(BInstruction::new(rs1, rs2, imm_val, bne)),
            0x4 => Some(BInstruction::new(rs1, rs2, imm_val, blt)),
            0x5 => Some(BInstruction::new(rs1, rs2, imm_val, bge)),
            0x6 => Some(BInstruction::new(rs1, rs2, imm_val, bltu)),
            0x7 => Some(BInstruction::new(rs1, rs2, imm_val, bgeu)),

            _ => None,
        }
    }

    fn try_decode_j_instr(&self, instr: u32) -> Option<JInstruction> {
        let rd = (instr >> 7) & 0x1F;

        let imm_val = get_j_imm_val(instr);

        Some(JInstruction::new(imm_val, rd, jal))
    }

    fn try_decode_jalr_instr(&self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;

        let imm_val = get_i_imm_val(instr);

        if funct3 == 0x0 {
            Some(IInstruction::new(instr, rs1, imm_val, rd, jalr))
        } else {
            None
        }
    }

    fn try_decode_lui_instr(&self, instr: u32) -> Option<UInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let imm_val = get_u_imm_val(instr);

        Some(UInstruction::new(imm_val, rd, lui))
    }

    fn try_decode_auipc_instr(&self, instr: u32) -> Option<UInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let imm_val = get_u_imm_val(instr);

        Some(UInstruction::new(imm_val, rd, auipc))
    }

    fn try_decode_atomic_instr(&self, instr: u32) -> Option<AtomicInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let rs2 = (instr >> 20) & 0x1F;
        let constraint_bits = (instr >> 25) & 0b11;
        let funct5 = (instr >> 27) & 0x1F;

        match (funct5, funct3) {
            (0b00010, 0x2) => Some(AtomicInstruction::new(constraint_bits, rs1, rs2, rd, lr_w)),
            (0b00011, 0x2) => Some(AtomicInstruction::new(constraint_bits, rs1, rs2, rd, sc_w)),
            (0b00001, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoswap_w,
            )),
            (0b00000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoadd_w,
            )),
            (0b00100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoxor_w,
            )),
            (0b01100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoand_w,
            )),
            (0b01000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amoor_w,
            )),
            (0b10000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomin_w,
            )),
            (0b10100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomax_w,
            )),
            (0b11000, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amominu_w,
            )),
            (0b11100, 0x2) => Some(AtomicInstruction::new(
                constraint_bits,
                rs1,
                rs2,
                rd,
                amomaxu_w,
            )),
            _ => None,
        }
    }

    fn try_decode_system_instr(&self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let csr = get_csr(instr);

        match funct3 {
            // 0x1 => todo!("CSRRW"),
            0x0 => match (csr, rs1, rd) {
                (0, 0, 0) => Some(IInstruction::new(instr, rs1, csr, rd, ecall)),
                (1, 0, 0) => Some(IInstruction::new(instr, rs1, csr, rd, ebreak)),

                // Tecnicamente son instrucciones R, pero asi me simplifica la vida
                (0x102, 0, 0) => Some(IInstruction::new(instr, rs1, csr, rd, sret)),
                (0x105, 0, 0) => Some(IInstruction::new(instr, rs1, csr, rd, wfi)),
                (0x120..=0x13F, _, _) => Some(IInstruction::new(instr, rs1, csr, rd, sfence_vma)),
                (0x302, 0, 0) => Some(IInstruction::new(instr, rs1, csr, rd, mret)),
                _ => None,
            },
            0x1 => Some(IInstruction::new(instr, rs1, csr, rd, csrrw)),
            0x2 => Some(IInstruction::new(instr, rs1, csr, rd, csrrs)),
            0x3 => Some(IInstruction::new(instr, rs1, csr, rd, csrrc)),
            0x5 => Some(IInstruction::new(instr, rs1, csr, rd, csrrwi)),
            0x6 => Some(IInstruction::new(instr, rs1, csr, rd, csrrsi)),
            0x7 => Some(IInstruction::new(instr, rs1, csr, rd, csrrci)),

            _ => None,
        }
    }

    fn try_decode_fence_instr(&self, instr: u32) -> Option<IInstruction> {
        let rd = (instr >> 7) & 0x1F;
        let funct3 = (instr >> 12) & 0x7;
        let rs1 = (instr >> 15) & 0x1F;
        let imm_val = get_i_imm_val(instr);

        match funct3 {
            0b000 => Some(IInstruction::new(instr, rs1, imm_val, rd, fence)),
            0b001 => Some(IInstruction::new(instr, rs1, imm_val, rd, fence_i)),
            _ => None,
        }
    }

    fn try_decode_c0(&self, instr: u16) -> Option<InstructionType> {
        let funct3 = (instr >> 13) & 0b111;

        match funct3 {
            0b000 => {
                let imm = (instr >> 5) & 0b11111111;
                let rd = (instr >> 2) & 0b111;
                let op = instr & 0b11;

                if imm != 0 {
                    Some(InstructionType::CIW(CIWInstruction::new(
                        funct3, imm, rd, op, c_addi4spn,
                    )))
                } else {
                    None
                }
            }
            0b001 => todo!("C.FLD"),
            0b010 => {
                let imm12_10 = (instr >> 10) & 0b111;
                let rs1 = (instr >> 7) & 0b111;
                let imm6_5 = (instr >> 5) & 0b11;
                let rd = (instr >> 2) & 0b111;
                let op = instr & 0b11;

                Some(InstructionType::CL(CLInstruction::new(
                    funct3, imm12_10, rs1, imm6_5, rd, op, c_lw,
                )))
            }
            0b011 => todo!("C.FLW C.LD (RV32 RV64)"),
            0b100 => None,
            0b101 => todo!("C.FSD"),
            0b110 => {
                let imm12_10 = (instr >> 10) & 0b111;
                let rs1 = (instr >> 7) & 0b111;
                let imm6_5 = (instr >> 5) & 0b11;
                let rs2 = (instr >> 2) & 0b111;
                let op = instr & 0b11;

                Some(InstructionType::CS(CSInstruction::new(
                    funct3, imm12_10, rs1, imm6_5, rs2, op, c_sw,
                )))
            }
            0b111 => todo!("C.FSW C.SD (RV32 RV64)"),
            _ => unreachable!(),
        }
    }

    fn try_decode_c1(&self, instr: u16) -> Option<InstructionType> {
        let funct3 = (instr >> 13) & 0b111;

        match funct3 {
            0b000 => {
                let imm12 = (instr >> 12) & 0b1;
                let rd_rs1 = (instr >> 7) & 0b11111;
                let imm6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                if rd_rs1 == 0 {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd_rs1, op, c_nop,
                    )))
                } else {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd_rs1, op, c_addi,
                    )))
                }
            }
            0b001 => {
                // C.ADDIW RV64
                let jump_target = (instr >> 2) & 0x7FF;
                let op = instr & 0b11;

                Some(InstructionType::CJ(CJInstruction::new(
                    funct3,
                    jump_target,
                    op,
                    c_jal,
                )))
            }
            0b010 => {
                let imm12 = (instr >> 12) & 0b1;
                let rd_rs1 = (instr >> 7) & 0b11111;
                let imm6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                Some(InstructionType::CI(CIInstruction::new(
                    funct3, imm12, imm6_2, rd_rs1, op, c_li,
                )))
            }
            0b011 => {
                let imm12 = (instr >> 12) & 0b1;
                let rd_rs1 = (instr >> 7) & 0b11111;
                let imm6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                if imm12 == 0 && imm6_2 == 0 {
                    None
                } else if rd_rs1 == 2 {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd_rs1, op, c_addi16sp,
                    )))
                } else {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd_rs1, op, c_lui,
                    )))
                }
            }
            0b100 => {
                let offset12_10 = (instr >> 10) & 0b111;
                let rd_rs1 = (instr >> 7) & 0b111;
                let offset6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                let func1 = offset12_10 & 0b11;
                let funct2 = (offset6_2 >> 3) & 0b11;

                let shamt5 = (offset12_10 >> 2) & 0b1;

                match func1 {
                    0b00 => {
                        if shamt5 == 0 {
                            Some(InstructionType::CB(CBInstruction::new(
                                funct3,
                                offset12_10,
                                rd_rs1,
                                offset6_2,
                                op,
                                c_srli,
                            )))
                        } else {
                            None
                        }
                    }
                    0b01 => {
                        if shamt5 == 0 {
                            Some(InstructionType::CB(CBInstruction::new(
                                funct3,
                                offset12_10,
                                rd_rs1,
                                offset6_2,
                                op,
                                c_srai,
                            )))
                        } else {
                            None
                        }
                    }
                    0b10 => Some(InstructionType::CB(CBInstruction::new(
                        funct3,
                        offset12_10,
                        rd_rs1,
                        offset6_2,
                        op,
                        c_andi,
                    ))),
                    0b11 => {
                        if ((offset12_10 >> 2) & 0b1) == 0 {
                            let funct6 = (instr >> 10) & 0b111111;
                            let rs2 = (instr >> 2) & 0b111;

                            match funct2 {
                                0b00 => Some(InstructionType::CA(CAInstruction::new(
                                    funct6, rd_rs1, funct2, rs2, op, c_sub,
                                ))),
                                0b01 => Some(InstructionType::CA(CAInstruction::new(
                                    funct6, rd_rs1, funct2, rs2, op, c_xor,
                                ))),
                                0b10 => Some(InstructionType::CA(CAInstruction::new(
                                    funct6, rd_rs1, funct2, rs2, op, c_or,
                                ))),
                                0b11 => Some(InstructionType::CA(CAInstruction::new(
                                    funct6, rd_rs1, funct2, rs2, op, c_and,
                                ))),
                                _ => unreachable!(),
                            }
                        } else {
                            todo!("C.SUBW C.ADDW")
                        }
                    }
                    _ => unreachable!(),
                }
            }

            0b101 => {
                let jump_target = (instr >> 2) & 0x7FF;
                let op = instr & 0b11;

                Some(InstructionType::CJ(CJInstruction::new(
                    funct3,
                    jump_target,
                    op,
                    c_j,
                )))
            }
            0b110 => {
                let offset12_10 = (instr >> 10) & 0b111;
                let rd_rs1 = (instr >> 7) & 0b111;
                let offset6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                Some(InstructionType::CB(CBInstruction::new(
                    funct3,
                    offset12_10,
                    rd_rs1,
                    offset6_2,
                    op,
                    c_beqz,
                )))
            }
            0b111 => {
                let offset12_10 = (instr >> 10) & 0b111;
                let rd_rs1 = (instr >> 7) & 0b111;
                let offset6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                Some(InstructionType::CB(CBInstruction::new(
                    funct3,
                    offset12_10,
                    rd_rs1,
                    offset6_2,
                    op,
                    c_benz,
                )))
            }
            _ => unreachable!(),
        }
    }

    fn try_decode_c2(&self, instr: u16) -> Option<InstructionType> {
        let funct3 = (instr >> 13) & 0b111;

        match funct3 {
            0b000 => {
                let rd = (instr >> 7) & 0b11111;
                let imm12 = (instr >> 12) & 0b1;
                let imm6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                if imm12 == 0 {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd, op, c_slli,
                    )))
                } else {
                    None
                }
            }
            0b001 => todo!("C.FLDSP"),
            0b010 => {
                let rd = (instr >> 7) & 0b11111;
                let imm12 = (instr >> 12) & 0b1;
                let imm6_2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                if rd == 0 {
                    None
                } else {
                    Some(InstructionType::CI(CIInstruction::new(
                        funct3, imm12, imm6_2, rd, op, c_lwsp,
                    )))
                }
            }
            0b011 => todo!("C.FLWSP C.LDSP (F RV64)"),
            // 0b100 => todo!("C.JR C.MV C.EBREAK C.JALR C.ADD"),
            0b100 => {
                let funct4 = (instr >> 12) & 0b1111;
                let rd_rs1 = (instr >> 7) & 0b11111;
                let rs2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                if funct4 & 0b1 == 1 {
                    if rs2 == 0 && rd_rs1 == 0 {
                        Some(InstructionType::I(IInstruction::new(
                            instr as u32,
                            0,
                            0,
                            0,
                            ebreak,
                        )))
                    } else if rs2 == 0 {
                        Some(InstructionType::CR(CRInstruction::new(
                            funct4, rd_rs1, rs2, op, c_jalr,
                        )))
                    } else {
                        Some(InstructionType::CR(CRInstruction::new(
                            funct4, rd_rs1, rs2, op, c_add,
                        )))
                    }
                } else {
                    if rs2 == 0 {
                        Some(InstructionType::CR(CRInstruction::new(
                            funct4, rd_rs1, rs2, op, c_jr,
                        )))
                    } else {
                        Some(InstructionType::CR(CRInstruction::new(
                            funct4, rd_rs1, rs2, op, c_mv,
                        )))
                    }
                }
            }
            0b101 => todo!("C.FSDSP"),
            0b110 => {
                let imm = (instr >> 7) & 0b111111;
                let rs2 = (instr >> 2) & 0b11111;
                let op = instr & 0b11;

                Some(InstructionType::CSS(CSSInstruction::new(
                    funct3, imm, rs2, op, c_swsp,
                )))
            }
            0b111 => todo!("C.FSWSP C.SDSP (F RV64)"),
            _ => unreachable!(),
        }
    }

    pub fn get_hartid(&self) -> usize {
        self.hart_id as usize
    }

    pub fn get_c_ext_active(&self) -> bool {
        let misa = self.control_and_status.read_misa_unchecked();

        (misa & (1 << 2)) > 0
    }

    pub fn check_int_to_m(&self, int: InterruptType) -> bool {
        let mstatus = self.control_and_status.read_mstatus_unchecked();
        let mip = self.control_and_status.read_mip_unchecked();
        let mie = self.control_and_status.read_mie_unchecked();
        let mideleg = self.control_and_status.read_mideleg_unchecked();

        ((self.privilege_level == PrivilegeLevel::Machine && mstatus.get_mie())
            || (self.privilege_level as u32) < (PrivilegeLevel::Machine as u32))
            && (((mip & mie) & (1 << (int as u32))) > 0)
            && ((mideleg & (1 << int as u32)) == 0)
    }

    pub fn check_int_to_s(&self, int: InterruptType) -> bool {
        let sstatus = self.control_and_status.read_sstatus_unchecked();
        let sip = self.control_and_status.read_sie_unchecked();
        let sie = self.control_and_status.read_sip_unchecked();

        ((self.privilege_level == PrivilegeLevel::Supervisor && sstatus.get_sie())
            || (self.privilege_level as u32) < (PrivilegeLevel::Supervisor as u32))
            && (((sip & sie) & (1 << (int as u32))) > 0)
    }

    pub fn check_pmp(
        &self,
        phys_address: PhysicalAddress,
        priv_level: PrivilegeLevel,
        access_type: AccessType,
        access_length: u64,
    ) -> Result<PhysicalAddress, ExceptionType> {
        self.control_and_status
            .check_pmp(phys_address, priv_level, access_type, access_length)
    }

    pub fn update_pc(&mut self) {
        self.pc = self.new_pc;
    }

    pub fn inc_pc(&mut self, bytes: u32) {
        self.new_pc = self.pc.wrapping_add(bytes);
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.new_pc = new_pc;
    }
}

fn get_i_imm_val(instr: u32) -> u32 {
    let imm_31_11 = 0xFFFFF800 * ((instr >> 31) & 1);
    let imm_10_0 = (instr >> 20) & 0x7FF;

    imm_31_11 | imm_10_0
}

fn get_s_imm_val(instr: u32) -> u32 {
    let imm_31_11 = 0xFFFFF800 * ((instr >> 31) & 1);
    let imm_10_5 = ((instr >> 25) & 0x3F) << 5;
    let imm_4_0 = (instr >> 7) & 0x1F;

    imm_31_11 | imm_10_5 | imm_4_0
}

fn get_b_imm_val(instr: u32) -> u32 {
    let imm_31_12 = 0xFFFFF000 * (instr >> 31);
    // let imm_11 = (instr & (1 << 7)) << 4;
    // let imm_10_5 = (instr & (0b0111111 << 25)) >> 20;
    // let imm_4_1 = (instr & (0b1111 << 8)) >> 7;
    let imm_11 = ((instr >> 7) & 1) << 11;
    let imm_10_5 = ((instr >> 25) & 0x3F) << 5;
    let imm_4_1 = ((instr >> 8) & 0xF) << 1;

    imm_31_12 | imm_11 | imm_10_5 | imm_4_1
}

fn get_j_imm_val(instr: u32) -> u32 {
    let imm_31_20 = 0xFFF00000 * (instr >> 31);
    let imm_19_12 = instr & (0b11111111 << 12);
    let imm_11 = (instr & (1 << 20)) >> 9;
    let imm_10_5 = (instr & (0b111111 << 25)) >> 20;
    let imm_4_1 = (instr & (0b1111 << 21)) >> 20;

    imm_31_20 | imm_19_12 | imm_11 | imm_10_5 | imm_4_1
}

fn get_u_imm_val(instr: u32) -> u32 {
    instr & 0xFFFFF000
}

fn get_csr(instr: u32) -> u32 {
    (instr & 0xFFF00000) >> 20
}
pub enum InstructionType {
    R(RInstruction),
    I(IInstruction),
    S(SInstruction),
    B(BInstruction),
    J(JInstruction),
    U(UInstruction),
    Atomic(AtomicInstruction),
    CR(CRInstruction),
    CI(CIInstruction),
    CSS(CSSInstruction),
    CIW(CIWInstruction),
    CL(CLInstruction),
    CS(CSInstruction),
    CA(CAInstruction),
    CB(CBInstruction),
    CJ(CJInstruction),
}

impl InstructionType {
    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        match self {
            Self::R(instr) => instr.execute(core),
            Self::I(instr) => instr.execute(mmu, core),
            Self::S(instr) => instr.execute(mmu, core),
            Self::B(instr) => instr.execute(core),
            Self::J(instr) => instr.execute(core),
            Self::U(instr) => instr.execute(core),
            Self::Atomic(instr) => instr.execute(mmu, core),
            Self::CR(instr) => instr.execute(core),
            Self::CI(instr) => instr.execute(mmu, core),
            Self::CSS(instr) => instr.execute(mmu, core),
            Self::CIW(instr) => instr.execute(mmu, core),
            Self::CL(instr) => instr.execute(mmu, core),
            Self::CS(instr) => instr.execute(mmu, core),
            Self::CA(instr) => instr.execute(core),
            Self::CB(instr) => instr.execute(core),
            Self::CJ(instr) => instr.execute(core),
        }
    }

    pub fn get_width(&self) -> u32 {
        match self {
            Self::R(_) => 4,
            Self::I(_) => 4,
            Self::S(_) => 4,
            Self::B(_) => 4,
            Self::J(_) => 4,
            Self::U(_) => 4,
            Self::Atomic(_) => 4,
            Self::CR(_) => 2,
            Self::CI(_) => 2,
            Self::CSS(_) => 2,
            Self::CIW(_) => 2,
            Self::CL(_) => 2,
            Self::CS(_) => 2,
            Self::CA(_) => 2,
            Self::CB(_) => 2,
            Self::CJ(_) => 2,
        }
    }
}

pub struct RInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl RInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        rd: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            rd,
            function,
        }
    }

    fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct IInstruction {
    pub rs1: u32,
    pub imm: u32,
    pub rd: u32,

    pub data: u32,

    function: fn(&IInstruction, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl IInstruction {
    pub fn new(
        data: u32,
        rs1: u32,
        imm: u32,
        rd: u32,
        function: fn(&IInstruction, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            data,
            rs1,
            imm,
            rd,
            function,
        }
    }

    fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct SInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl SInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        imm: u32,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            imm,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct BInstruction {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl BInstruction {
    pub fn new(
        rs1: u32,
        rs2: u32,
        imm: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            rs1,
            rs2,
            imm,
            function,
        }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct JInstruction {
    pub imm: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl JInstruction {
    pub fn new(
        imm: u32,
        rd: u32,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self { imm, rd, function }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct UInstruction {
    pub imm: u32,
    pub rd: u32,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl UInstruction {
    pub fn new(
        imm: u32,
        rd: u32,

        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self { imm, rd, function }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct AtomicInstruction {
    pub _constraint_bits: u32,
    pub rs2: u32,
    pub rs1: u32,
    pub rd: u32,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl AtomicInstruction {
    pub fn new(
        constraint_bits: u32,
        rs1: u32,
        rs2: u32,
        rd: u32,

        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            _constraint_bits: constraint_bits,
            rs2,
            rs1,
            rd,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        // (self.function)(self.rs1, self.rs2, self.rd, self.mmu, self.core)
        (self.function)(self, mmu, core)
    }
}

pub struct CRInstruction {
    pub funct4: u16,
    pub rd_rs1: u16,
    pub rs2: u16,
    pub op: u16,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl CRInstruction {
    pub fn new(
        funct4: u16,
        rd_rs1: u16,
        rs2: u16,
        op: u16,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct4,
            rd_rs1,
            rs2,
            op,
            function,
        }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct CIInstruction {
    pub funct3: u16,
    pub imm12: u16,
    pub imm6_2: u16,
    pub rd_rs1: u16,
    pub op: u16,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl CIInstruction {
    pub fn new(
        funct3: u16,
        imm12: u16,
        imm6_2: u16,
        rd_rs1: u16,
        op: u16,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            imm12,
            imm6_2,
            rd_rs1,
            op,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct CSSInstruction {
    pub funct3: u16,
    pub imm: u16,
    pub rs2: u16,
    pub op: u16,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl CSSInstruction {
    pub fn new(
        funct3: u16,
        imm: u16,
        rs2: u16,
        op: u16,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            imm,
            rs2,
            op,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct CIWInstruction {
    pub funct3: u16,
    pub imm: u16,
    pub rd_p: u16,
    pub op: u16,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl CIWInstruction {
    pub fn new(
        funct3: u16,
        imm: u16,
        rd_p: u16,
        op: u16,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            imm,
            rd_p,
            op,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct CLInstruction {
    pub funct3: u16,
    pub imm12_10: u16,
    pub rs1_p: u16,
    pub imm6_5: u16,
    pub rd_p: u16,
    pub op: u16,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl CLInstruction {
    pub fn new(
        funct3: u16,
        imm12_10: u16,
        rs1_p: u16,
        imm6_5: u16,
        rd_p: u16,
        op: u16,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            imm12_10,
            rs1_p,
            imm6_5,
            rd_p,
            op,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct CSInstruction {
    pub funct3: u16,
    pub imm12_10: u16,
    pub rs1_p: u16,
    pub imm6_5: u16,
    pub rs2_p: u16,
    pub op: u16,

    function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
}

impl CSInstruction {
    pub fn new(
        funct3: u16,
        imm12_10: u16,
        rs1_p: u16,
        imm6_5: u16,
        rs2_p: u16,
        op: u16,
        function: fn(&Self, &mut Bus, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            imm12_10,
            rs1_p,
            imm6_5,
            rs2_p,
            op,
            function,
        }
    }

    pub fn execute(&self, mmu: &mut Bus, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, mmu, core)
    }
}

pub struct CAInstruction {
    pub funct6: u16,
    pub rd_rs1_p: u16,
    pub funct2: u16,
    pub rs2_p: u16,
    pub op: u16,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl CAInstruction {
    pub fn new(
        funct6: u16,
        rd_rs1_p: u16,
        funct2: u16,
        rs2_p: u16,
        op: u16,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct6,
            rd_rs1_p,
            funct2,
            rs2_p,
            op,
            function,
        }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct CBInstruction {
    pub funct3: u16,
    pub offset12_10: u16,
    pub rd_rs1_p: u16,
    pub offset6_2: u16,
    pub op: u16,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl CBInstruction {
    pub fn new(
        funct3: u16,
        offset12_10: u16,
        rd_rs1_p: u16,
        offset6_2: u16,
        op: u16,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            offset12_10,
            rd_rs1_p,
            offset6_2,
            op,
            function,
        }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

pub struct CJInstruction {
    pub funct3: u16,
    pub jump_target: u16,
    pub op: u16,

    function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
}

impl CJInstruction {
    pub fn new(
        funct3: u16,
        jump_target: u16,
        op: u16,
        function: fn(&Self, &mut RVCore) -> Result<(), Exception>,
    ) -> Self {
        Self {
            funct3,
            jump_target,
            op,
            function,
        }
    }

    pub fn execute(&self, core: &mut RVCore) -> Result<(), Exception> {
        (self.function)(self, core)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PrivilegeLevel {
    User = 0x00,
    Supervisor = 0x01,
    #[default]
    Machine = 0x03,
}

impl PrivilegeLevel {
    pub fn new(val: u32) -> Self {
        match val {
            0x00 => Self::User,
            0x01 => Self::Supervisor,
            0x03 => Self::Machine,
            _ => Self::Machine, // TODO
        }
    }
}

pub trait Trap {
    fn get_cause(&self) -> u32;
    fn get_val(&self) -> u32;
    fn handle(&self, core: &mut RVCore);
    fn is_int(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExceptionType {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAmoAddressMisaligned = 6,
    StoreAmoAccessFault = 7,
    EnviromentCallFromUMode = 8,
    EnviromentCallFromSMode = 9,
    EnviromentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StoreAmoPageFault = 15,
    DoubleTrap = 16,
    SoftwareCheck = 18,
    HardwareError = 19,

    #[cfg(feature = "hitf")]
    HitfSyscall = 24,
    #[cfg(any(feature = "hitf", feature = "semihosting"))]
    ExitException = 25,
}

#[derive(Clone, Copy, Debug)]
pub struct Exception {
    pub exc_type: ExceptionType,
    pub val: u32,
}

impl Trap for Exception {
    fn get_cause(&self) -> u32 {
        self.exc_type as u32
    }

    fn get_val(&self) -> u32 {
        self.val
    }

    fn handle(&self, core: &mut RVCore) {
        let prev_priv_level = core.privilege_level;
        let cause = self.get_cause();

        let delegated = {
            let medelegl = core.control_and_status.read_medeleg_unchecked();
            let medelegh = core.control_and_status.read_medelegh_unchecked();
            let medeleg = ((medelegh as u64) << 32) | (medelegl as u64);

            ((medeleg >> cause) & 1u64) != 0
        };
        let handle_machine = !delegated || (prev_priv_level == PrivilegeLevel::Machine);

        if handle_machine {
            handle_machine_trap(self, core, cause);
        } else {
            handle_supervisor_trap(self, core, cause);
        }
    }

    fn is_int(&self) -> bool {
        false
    }
}

impl Exception {
    pub fn new(exc_type: ExceptionType, val: u32) -> Self {
        Self { exc_type, val }
    }
}

pub trait WithErrVal<T> {
    fn with_err_val(self, val: u32) -> Result<T, Exception>;
}

impl<T> WithErrVal<T> for Result<T, Exception> {
    fn with_err_val(self, val: u32) -> Result<T, Exception> {
        self.map_err(|mut e| {
            if e.val == 0 {
                e.val = val;
            }
            e
        })
    }
}

impl<T> WithErrVal<T> for Result<T, ExceptionType> {
    fn with_err_val(self, val: u32) -> Result<T, Exception> {
        self.map_err(|exc_type| Exception::new(exc_type, val))
    }
}

#[derive(Debug)]
pub struct Interrupt {
    int_type: InterruptType,
    val: u32,
    delegated_to_s: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum InterruptType {
    SupervisorSwInt = 1,
    MachineSwInt = 3,
    SupervisorTimerInt = 5,
    MachineTimerInt = 7,
    SupervisorExternalInt = 9,
    MachineExternalInt = 11,
    CounterOverflowInt = 13,
}

impl Trap for Interrupt {
    fn get_cause(&self) -> u32 {
        (1 << 31) | self.int_type as u32
    }

    fn get_val(&self) -> u32 {
        self.val
    }

    fn handle(&self, core: &mut RVCore) {
        let cause = self.get_cause();

        if self.delegated_to_s {
            handle_supervisor_trap(self, core, cause);
        } else {
            handle_machine_trap(self, core, cause);
        }
    }

    fn is_int(&self) -> bool {
        true
    }
}

impl Interrupt {
    pub fn new(int_type: InterruptType, delegated_to_s: bool) -> Self {
        Self {
            int_type,
            val: 0,
            delegated_to_s,
        }
    }
}
fn handle_machine_trap(trap: &impl Trap, core: &mut RVCore, cause: u32) {
    let prev_priv_level: PrivilegeLevel = core.privilege_level;

    let tval = trap.get_val();

    core.privilege_level = PrivilegeLevel::Machine;

    core.control_and_status
        .write_csr(ControlAndStatus::MEPC, core.privilege_level, core.pc)
        .unwrap();
    core.control_and_status
        .write_csr(ControlAndStatus::MCAUSE, core.privilege_level, cause)
        .unwrap();
    core.control_and_status
        .write_csr(ControlAndStatus::MTVAL, core.privilege_level, tval)
        .unwrap();

    let mut mstatus = core.control_and_status.read_mstatus_unchecked();
    mstatus.set_mpp(prev_priv_level as u32);
    mstatus.set_mpie(mstatus.get_mie());
    mstatus.set_mie(false);
    core.control_and_status
        .write_csr(ControlAndStatus::MSTATUS, core.privilege_level, mstatus.0)
        .unwrap();

    let mtvec = core.control_and_status.read_mtvec_unchecked();
    let base = mtvec & 0xFFFFFFFC;

    if trap.is_int() {
        let cause = cause & !(1 << 31);

        if mtvec & 0b11 == 1 {
            core.set_pc(base + 4 * cause);
        } else {
            core.set_pc(base);
        }
    } else {
        core.set_pc(base);
    }
}

fn handle_supervisor_trap(trap: &impl Trap, core: &mut RVCore, cause: u32) {
    let prev_priv_level = core.privilege_level;

    let tval = trap.get_val();

    core.privilege_level = PrivilegeLevel::Supervisor;

    core.control_and_status
        .write_csr(ControlAndStatus::SEPC, core.privilege_level, core.pc)
        .unwrap();
    core.control_and_status
        .write_csr(ControlAndStatus::SCAUSE, core.privilege_level, cause)
        .unwrap();
    core.control_and_status
        .write_csr(ControlAndStatus::STVAL, core.privilege_level, tval)
        .unwrap();

    let mut sstatus = core.control_and_status.read_sstatus_unchecked();
    sstatus.set_spp(prev_priv_level == PrivilegeLevel::Supervisor);
    sstatus.set_spie(sstatus.get_sie());
    sstatus.set_sie(false);
    core.control_and_status
        .write_csr(ControlAndStatus::SSTATUS, core.privilege_level, sstatus.0)
        .unwrap();

    let stvec = core.control_and_status.read_stvec_unchecked();
    let base = stvec & 0xFFFFFFFC;

    if trap.is_int() {
        let cause = cause & !(1 << 31);

        if stvec & 0b11 == 1 {
            core.set_pc(base + 4 * cause);
        } else {
            core.set_pc(base);
        }
    } else {
        core.set_pc(base);
    }
}
