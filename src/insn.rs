#[derive(Clone, Copy, Debug)]
pub struct Insn(pub u64);

impl Insn {
    pub fn from_bits(bits: u32) -> Self {
        Self(bits as u64)
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    }

    pub fn bits(&self) -> u64 {
        self.0
    }

    pub fn bit_range(&self, offset: u8, length: u8) -> u64 {
        (self.bits() >> offset) & ((1 << length) - 1)
    }

    pub fn bit_range_sign_extended(&self, offset: u8, length: u8) -> u64 {
        ((self.bits() as i64) << (64 - offset - length) >> (64 - length)) as u64
    }

    pub fn sign_extend(value: u64, length: u8) -> i64 {
        let sign_bit = 1u64 << (length - 1);
        if value & sign_bit != 0 {
            (value as i64) | !((1u64 << length) - 1) as i64
        } else {
            value as i64
        }
    }
}

impl Insn {
    pub fn rd(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn rt(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn rs1(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn rs2(&self) -> u64 {
        self.bit_range(20, 5)
    }
    pub fn rs3(&self) -> u64 {
        self.bit_range(27, 5)
    }
    pub fn aqrl(&self) -> u64 {
        self.bit_range(25, 2)
    }
    pub fn aq(&self) -> u64 {
        self.bit_range(26, 1)
    }
    pub fn rl(&self) -> u64 {
        self.bit_range(25, 1)
    }
    pub fn fm(&self) -> u64 {
        self.bit_range(28, 4)
    }
    pub fn pred(&self) -> u64 {
        self.bit_range(24, 4)
    }
    pub fn succ(&self) -> u64 {
        self.bit_range(20, 4)
    }
    pub fn rm(&self) -> u64 {
        self.bit_range(12, 3)
    }
    pub fn funct3(&self) -> u64 {
        self.bit_range(12, 3)
    }
    pub fn funct2(&self) -> u64 {
        self.bit_range(25, 2)
    }
    pub fn imm20(&self) -> u64 {
        self.bit_range(12, 20)
    }
    pub fn jimm20(&self) -> u64 {
        self.bit_range(12, 20)
    }
    pub fn imm12(&self) -> u64 {
        self.bit_range(20, 12)
    }
    pub fn csr(&self) -> u64 {
        self.bit_range(20, 12)
    }
    pub fn imm12hi(&self) -> u64 {
        self.bit_range(25, 7)
    }
    pub fn bimm12hi(&self) -> u64 {
        self.bit_range(25, 7)
    }
    pub fn imm12lo(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn bimm12lo(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn shamtq(&self) -> u64 {
        self.bit_range(20, 7)
    }
    pub fn shamtw(&self) -> u64 {
        self.bit_range(20, 5)
    }
    pub fn shamtw4(&self) -> u64 {
        self.bit_range(20, 4)
    }
    pub fn shamtd(&self) -> u64 {
        self.bit_range(20, 6)
    }
    pub fn bs(&self) -> u64 {
        self.bit_range(30, 2)
    }
    pub fn rnum(&self) -> u64 {
        self.bit_range(20, 4)
    }
    pub fn rc(&self) -> u64 {
        self.bit_range(25, 5)
    }
    pub fn imm2(&self) -> u64 {
        self.bit_range(20, 2)
    }
    pub fn imm3(&self) -> u64 {
        self.bit_range(20, 3)
    }
    pub fn imm4(&self) -> u64 {
        self.bit_range(20, 4)
    }
    pub fn imm5(&self) -> u64 {
        self.bit_range(20, 5)
    }
    pub fn imm6(&self) -> u64 {
        self.bit_range(20, 6)
    }
    pub fn zimm(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn opcode(&self) -> u64 {
        self.bit_range(0, 7)
    }
    pub fn funct7(&self) -> u64 {
        self.bit_range(25, 7)
    }
    pub fn vd(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn vs3(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn vs1(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn vs2(&self) -> u64 {
        self.bit_range(20, 5)
    }
    pub fn vm(&self) -> u64 {
        self.bit_range(25, 1)
    }
    pub fn wd(&self) -> u64 {
        self.bit_range(26, 1)
    }
    pub fn amoop(&self) -> u64 {
        self.bit_range(27, 5)
    }
    pub fn nf(&self) -> u64 {
        self.bit_range(29, 3)
    }
    pub fn simm5(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn zimm5(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn zimm10(&self) -> u64 {
        self.bit_range(20, 10)
    }
    pub fn zimm11(&self) -> u64 {
        self.bit_range(20, 11)
    }
    pub fn zimm6hi(&self) -> u64 {
        self.bit_range(26, 1)
    }
    pub fn zimm6lo(&self) -> u64 {
        self.bit_range(15, 5)
    }
    pub fn c_nzuimm10(&self) -> u64 {
        self.bit_range(5, 8)
    }
    pub fn c_uimm7lo(&self) -> u64 {
        self.bit_range(5, 2)
    }
    pub fn c_uimm7hi(&self) -> u64 {
        self.bit_range(10, 3)
    }
    pub fn c_uimm8lo(&self) -> u64 {
        self.bit_range(5, 2)
    }
    pub fn c_uimm8hi(&self) -> u64 {
        self.bit_range(10, 3)
    }
    pub fn c_uimm9lo(&self) -> u64 {
        self.bit_range(5, 2)
    }
    pub fn c_uimm9hi(&self) -> u64 {
        self.bit_range(10, 3)
    }
    pub fn c_nzimm6lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_nzimm6hi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_imm6lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_imm6hi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_nzimm10hi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_nzimm10lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_nzimm18hi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_nzimm18lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_imm12(&self) -> u64 {
        self.bit_range(2, 11)
    }
    pub fn c_bimm9lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_bimm9hi(&self) -> u64 {
        self.bit_range(10, 3)
    }
    pub fn c_nzuimm5(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_nzuimm6lo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_nzuimm6hi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_uimm8splo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_uimm8sphi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_uimm8sp_s(&self) -> u64 {
        self.bit_range(7, 6)
    }
    pub fn c_uimm10splo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_uimm10sphi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_uimm9splo(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_uimm9sphi(&self) -> u64 {
        self.bit_range(12, 1)
    }
    pub fn c_uimm10sp_s(&self) -> u64 {
        self.bit_range(7, 6)
    }
    pub fn c_uimm9sp_s(&self) -> u64 {
        self.bit_range(7, 6)
    }
    pub fn c_uimm2(&self) -> u64 {
        self.bit_range(5, 2)
    }
    pub fn c_uimm1(&self) -> u64 {
        self.bit_range(5, 1)
    }
    pub fn c_rlist(&self) -> u64 {
        self.bit_range(4, 4)
    }
    pub fn c_spimm(&self) -> u64 {
        self.bit_range(2, 2)
    }
    pub fn c_index(&self) -> u64 {
        self.bit_range(2, 8)
    }
    pub fn rs1_p(&self) -> u64 {
        self.bit_range(7, 3)
    }
    pub fn rs2_p(&self) -> u64 {
        self.bit_range(2, 3)
    }
    pub fn rd_p(&self) -> u64 {
        self.bit_range(2, 3)
    }
    pub fn rd_rs1_n0(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn rd_rs1_p(&self) -> u64 {
        self.bit_range(7, 3)
    }
    pub fn rd_rs1(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn rd_n2(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn rd_n0(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn rs1_n0(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn c_rs2_n0(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_rs1_n0(&self) -> u64 {
        self.bit_range(7, 5)
    }
    pub fn c_rs2(&self) -> u64 {
        self.bit_range(2, 5)
    }
    pub fn c_sreg1(&self) -> u64 {
        self.bit_range(7, 3)
    }
    pub fn c_sreg2(&self) -> u64 {
        self.bit_range(2, 3)
    }
}
