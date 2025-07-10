#[derive(Default, Debug)]
pub struct Cpu {
    pub pc: u64,
    pub regs: [u64; 32]
}