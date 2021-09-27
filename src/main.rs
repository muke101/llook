use std::process::Command;
use std::path::Path;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, InstructionValue};
use inkwell::basic_block::BasicBlock;
use either::Either;

fn get_test_ir(ctx: &Context) -> Module  {
    Command::new("llvm-as").arg("test.ll");
    let p = Path::new("./test.bc");
    return Module::parse_bitcode_from_path(p, ctx).unwrap();
}

fn get_first_inst<'a>(module: &'a Module) -> InstructionValue<'a> {
    let func = module.get_first_function().unwrap();
    let entry = func.get_basic_blocks()[0];
    return entry.get_first_instruction().unwrap();
}

//FIXME: does this mess with ownership?
fn get_defs<'a>(inst: &'a InstructionValue) -> Vec<Either<BasicValueEnum<'a>, BasicBlock<'a>>>   {
    return (0..inst.get_num_operands())
            .map(|i| inst.get_operand(i)
                 .unwrap())
            .collect::<Vec<_>>();
}

/*iterate over lines,
 * map ssa names to line numbers,
 * iterate over functions,
 * basic blocks and instructions,
 * get their ssa names,
 * remap lines to objects*/

fn main() {
    let ctx = Context::create();
    let module = get_test_ir(&ctx);
}
