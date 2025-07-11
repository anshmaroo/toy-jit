use crate::{cpu::Cpu, jit::JIT};

mod block;
mod cpu;
mod insn;
mod jit;

fn main() {
    let mut jit = JIT::default();
    let rom = [
        0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3, 0x001100b3,
        0x001100b3, 0x001100b3, 0x001100b3, 0xfca09ce3, 0x003080b3, 0xfe009ee3
    ];
    let mut cpu = Cpu::default();
    cpu.regs[2] = 1;
    cpu.regs[10] = 30;
    cpu.regs[3] = u64::MAX;
    

    while cpu.pc < (rom.len() * 4) as u64 {
        if let Some(code) = jit.block_cache.get(&cpu.pc) {
            let function = *code;
            // Cast and call the JIT-compiled function
            let func: extern "C" fn(*mut Cpu) -> u64 = unsafe { std::mem::transmute(function) };
            let new_pc = func(&mut cpu as *mut Cpu);
            cpu.pc = new_pc;
        } else {
            println!("creating block at pc = {:#16x}", cpu.pc);
            jit.build_block(&rom, cpu.pc);
        }
    }
    println!("cpu state: {:?}", cpu);
}
