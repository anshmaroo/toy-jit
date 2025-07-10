use crate::{cpu::Cpu, jit::JIT};

mod block;
mod cpu;
mod insn;
mod jit;

fn main() {
    let mut jit = JIT::default();
    let rom = [
        0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3,
        0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3,
        0x001100b3,
    ];
    let mut cpu = Cpu::default();
    cpu.regs[2] = 1;
    cpu.regs[0] = 0;

    while cpu.pc < rom.len() as u64 {
        let function = jit
            .build_block(&rom, cpu.pc, 5)
            .expect("could not translate block");

        // Cast and call the JIT-compiled function
        let func: extern "C" fn(*mut Cpu) -> u64 = unsafe { std::mem::transmute(function) };
        let new_pc = func(&mut cpu as *mut Cpu);

        cpu.pc = new_pc;

        println!("cpu state after running: {:?}", cpu);
    }
}
