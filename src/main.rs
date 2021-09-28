use std::process::Command;
use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::{File, read_to_string};
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, InstructionValue};
use inkwell::basic_block::BasicBlock;
use either::Either;
use std::collections::HashMap;
use regex::Regex;

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

fn get_defs<'a>(inst: &'a InstructionValue) -> Vec<Either<BasicValueEnum<'a>, BasicBlock<'a>>>   {
    return (0..inst.get_num_operands()).map(|i| inst.get_operand(i)
                                            .unwrap())
                                       .collect::<Vec<_>>();
}

fn get_lines() -> Vec<String>  {
    let path = Path::new("./test.ll");
    let raw = read_to_string(path).unwrap();
    return raw.lines()
              .map(String::from)
              .collect::<Vec<_>>();
}

fn map_block_line_numbers(lines: &Vec<String>) -> HashMap<&str, u32>  {
    let mut block_lines: HashMap<&str, u32> = HashMap::new();

    let block = Regex::new(r"^.:\n$").unwrap();
    for (i, line) in lines.iter().enumerate()   {
        if block.is_match(line) {
            let len = line.len();
            let mut a = 0;
            let mut b = len-3;
            match line.find(" ")    {
                Some(_) => {a += 1; b -= 1;},
                None => (),
            };
            block_lines.insert(&line[a..b], i as u32);
        }
    }

    return block_lines;
}

/*
 * iterate over lines, map function and basic block names to line numbers
 * map names to objects
 * iterate through lines, for each basic block find it's object
 * iterate through instructions of basic block object
 * map instructions to line numbers after basic block sequentially
 * save their names
 */

fn main() {
    let ctx = Context::create();
    let module = get_test_ir(&ctx);
}
