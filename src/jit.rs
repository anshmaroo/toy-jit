use cranelift::prelude::isa::aarch64::inst::Cond;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
use std::i64;
use std::mem::offset_of;

use crate::block::Block;
use crate::cpu::Cpu;
use crate::insn::Insn;
pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data description, which is to data objects what `ctx` is to functions.
    data_description: DataDescription,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }
}

enum ControlFlowType {
    Sequential(u64),
    Jump(Value),
    End(Value),
}
impl JIT {
    pub fn build_block(&mut self, rom: &[u32], start_pc: u64) -> Result<*const u8, String> {
        // push a pointer to the cpu as an input, new pc as output
        self.ctx
            .func
            .signature
            .params
            .push(AbiParam::new(types::I64));
        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(types::I64));

        // create a function builder
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        // create an entry block in the builder
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.append_block_params_for_function_params(entry_block);

        let mut i = start_pc;
        let mut end_pc;
        loop {
            match Self::translate(&mut builder, rom, i) {
                ControlFlowType::Sequential(new_pc_val) => i = new_pc_val,
                ControlFlowType::Jump(new_pc_val) | ControlFlowType::End(new_pc_val) => {
                    end_pc = new_pc_val;
                    break;
                }
            }
        }

        builder.ins().return_(&[end_pc]);
        builder.seal_all_blocks();
        builder.finalize();

        // declare the function
        let id = self
            .module
            .declare_anonymous_function(&self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        // define the function to jit
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        // Now that compilation is finished, we can clear out the context state.
        self.module.clear_context(&mut self.ctx);

        self.module.finalize_definitions().unwrap();

        let code = self.module.get_finalized_function(id);

        Ok(code)
    }

    fn translate(builder: &mut FunctionBuilder, rom: &[u32], pc: u64) -> ControlFlowType {
        let cpu_ptr = builder.block_params(builder.current_block().expect("err"))[0];
        if pc >> 2 < rom.len() as u64 {
            let insn = Insn::from_bits(rom[(pc >> 2) as usize]);
            let bits = insn.bits();
            let length = if (insn.bits() & 0b11 == 0b11) { 4 } else { 2 };
            let mut flow_type = ControlFlowType::Sequential(pc + length);

            let rd = insn.rd() as usize;
            let rs1 = insn.rs1() as usize;
            let rs2 = insn.rs2() as usize;
            let bimm12hi = insn.bimm12hi();
            let bimm12lo = insn.bimm12lo();

            let rs1_offset = offset_of!(Cpu, regs) + rs1 * 8;
            let rs2_offset = offset_of!(Cpu, regs) + rs2 * 8;
            let rd_offset = offset_of!(Cpu, regs) + rd * 8;

            let branch_offset = Insn::sign_extend(
                ((bimm12hi & 0x40) << 6)
                    | ((bimm12lo & 0x01) << 11)
                    | ((bimm12hi & 0x3F) << 5)
                    | (bimm12lo & 0x1E),
                13,
            );

            if bits & 0xfe00707f == 0x33 {
                // add
                let rs1_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs1_offset as i32);
                let rs2_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs2_offset as i32);
                let result = builder.ins().iadd(rs1_value, rs2_value);
                builder
                    .ins()
                    .store(MemFlags::new(), result, cpu_ptr, rd_offset as i32);
            } else if bits & 0xfe00707f == 0x7033 {
                // and
                let rs1_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs1_offset as i32);
                let rs2_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs2_offset as i32);
                let result = builder.ins().band(rs1_value, rs2_value);
                builder
                    .ins()
                    .store(MemFlags::new(), result, cpu_ptr, rd_offset as i32);
            } else if bits & 0xfe00707f == 0x1033 {
                // sll
                let rs1_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs1_offset as i32);
                let rs2_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs2_offset as i32);
                let result = builder.ins().ishl(rs1_value, rs2_value);
                builder
                    .ins()
                    .store(MemFlags::new(), result, cpu_ptr, rd_offset as i32);
            } else if bits & 0x707f == 0x1063 {
                // bne
                let rs1_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs1_offset as i32);
                let rs2_value =
                    builder
                        .ins()
                        .load(types::I64, MemFlags::new(), cpu_ptr, rs2_offset as i32);

                // perform comparison
                let condition = builder.ins().icmp(IntCC::NotEqual, rs1_value, rs2_value);

                // define blocks
                let taken_block = builder.create_block();
                let not_taken_block = builder.create_block();

                // taken or not taken path
                builder
                    .ins()
                    .brif(condition, taken_block, &[], not_taken_block, &[]);

                builder.switch_to_block(taken_block);
                builder.seal_block(taken_block);
                flow_type = ControlFlowType::Jump(
                    builder
                        .ins()
                        .iconst(types::I64, pc.wrapping_add_signed(branch_offset) as i64),
                );

                builder.switch_to_block(not_taken_block);
                builder.seal_block(not_taken_block);
                flow_type = ControlFlowType::Jump(
                    builder.ins().iconst(types::I64, pc.wrapping_add(4) as i64),
                );
            }

            flow_type
        } else {
            let new_pc = builder.ins().iconst(types::I64, pc as i64);
            return ControlFlowType::End(new_pc);
        }
    }
}
