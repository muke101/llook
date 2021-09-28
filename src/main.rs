use std::process::Command;
use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::fs::{File, read_to_string};
use std::ptr::null;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::values::*;
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

fn parse_block_name(line: &str) -> &str {
        let len = line.len();
        let mut a = 0;
        let mut b = len-3;
        match line.find(" ")    {
            Some(_) => {a += 1; b -= 1;},
            None => (),
        };
        return &line[a..b];
}

struct TextData<'a> {
    line_number: u32,
    name: &'a str,
}

#[derive(PartialEq, Eq, Hash)]
enum LLVMValue<'a>  {
    ArrayValue(ArrayValue<'a>),
    IntValue(IntValue<'a>),
    FloatValue(FloatValue<'a>),
    PointerValue(PointerValue<'a>),
    StructValue(StructValue<'a>),
    VectorValue(VectorValue<'a>),
    BasicBlockValue(BasicBlock<'a>),
    FunctionValue(FunctionValue<'a>),
}

fn get_line_numbers<'a, 'b>(lines: &'a Vec<String>, module: &'b Module) -> HashMap<LLVMValue<'b>, TextData<'a>>   {
    let mut ir_map: HashMap<LLVMValue, TextData> = HashMap::new();
    let block = Regex::new(r"^.:\n$").unwrap();
    let func = Regex::new(r"^define ").unwrap();
    let func_name = Regex::new(r#".*@(".*"|.*)\("#).unwrap();
    let mut current_blocks: Vec<BasicBlock>;

    for (i, line) in lines.iter().enumerate()   {
        let line = line.trim_start_matches(|c: char| c.is_whitespace());
        if block.is_match(line) {
            let ssa_name = parse_block_name(line);
            for block in current_blocks.iter()  {
                if ssa_name == block.get_name().to_str().unwrap() {
                    ir_map.insert(LLVMValue::BasicBlockValue(*block),
                                  TextData { line_number: i as u32, name: ssa_name, });
                }
            }
        }
        else if func.is_match(line) {
            let ssa_name = func_name.find(line).unwrap();
            let ssa_name = &line[ssa_name.start()..ssa_name.end()-1]; //TODO: might not have to subtract 1 on end
            let func = module.get_function(ssa_name).unwrap();
            ir_map.insert(LLVMValue::FunctionValue(func),
                          TextData { line_number: i as u32, name: ssa_name });
            current_blocks = func.get_basic_blocks();
        }
    }

    return ir_map;
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
