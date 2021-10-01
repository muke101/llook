use std::ffi::CStr;
use std::fs::read_to_string;
use std::os::raw::c_char;
use std::process::Command;
use std::path::Path;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::values::*;
use inkwell::basic_block::BasicBlock;
use std::collections::HashMap;
use regex::{Regex};
use either::Either;

struct TextData<'a> {
    line_number: usize,
    name: &'a str,
}

#[derive(PartialEq, Eq, Hash)]
enum LLVMValue<'a>  {
    Basic(BasicValueEnum<'a>),
    Instruction(InstructionValue<'a>),
    BasicBlock(BasicBlock<'a>),
    Function(FunctionValue<'a>),
}

fn get_test_ir(ctx: &Context) -> Module  {
    Command::new("llvm-as").arg("/home/muke/Programming/llook/test.ll");
    let p = Path::new("/home/muke/Programming/llook/test.bc");
    return Module::parse_bitcode_from_path(p, ctx).unwrap();
}

//TODO: investigate what happens when an operand is a function
fn get_defs<'a>(inst: &'a InstructionValue) -> Vec<LLVMValue<'a>>   {
    return (0..inst.get_num_operands()).map(|i|
                                            match inst.get_operand(i).unwrap()  {
                                                Either::Left(op) => LLVMValue::Basic(op),
                                                Either::Right(op) => LLVMValue::BasicBlock(op),
                                            })
                                       .collect::<Vec<_>>();
}

fn get_lines() -> Vec<String>  {
    let path = Path::new("/home/muke/Programming/llook/test.ll");
    let raw = read_to_string(path).unwrap();
    return raw.lines()
              .map(String::from)
              .collect::<Vec<_>>();
}

extern "C"  { fn get_name(isnt: InstructionValue) -> *const c_char; }

fn parse_inst<'a, 'b>(i: usize, next_inst: &mut Option<InstructionValue<'b>>, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> (Option<InstructionValue<'b>>, bool)   {
    let inst = next_inst.unwrap();
    //TODO: free ssa_name somewhere
    let ssa_name = unsafe { CStr::from_ptr(get_name(inst)).to_str().unwrap() };
    ir_map.insert(LLVMValue::Instruction(inst),
                  TextData { line_number: i, name: ssa_name });
    *next_inst = inst.get_next_instruction();
    if next_inst == &None {
        return (*next_inst, false);
    }
    return (*next_inst, true);
}

fn parse_block<'a, 'b>(i: usize, ssa_name: &'a str, current_blocks: Vec<BasicBlock<'b>>, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> Option<InstructionValue<'b>>   {
    for block in current_blocks.iter() {
        if ssa_name == block.get_name().to_str().unwrap() {
            ir_map.insert(LLVMValue::BasicBlock(*block),
                          TextData { line_number: i, name: ssa_name });
            return block.get_first_instruction();
        }
    }
    panic!("Block name not found in module!");
}

fn parse_func<'a, 'b>(i: usize, ssa_name: &'a str, module: &'b Module, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> Vec<BasicBlock<'b>>    {
    let func = module.get_function(ssa_name).unwrap();
    ir_map.insert(LLVMValue::Function(func),
                  TextData { line_number: i, name: ssa_name });
    return func.get_basic_blocks();
}

fn strip_quotes(a: usize, b: usize, line: &str) -> &str   {
    let mut ssa_name = &line[a..b];
    if let Some(_) = ssa_name.find("\"") {
        ssa_name = &line[a+1..b-1];
    }
    return ssa_name;
}

fn parse_ir<'a, 'b>(lines: &'a Vec<String>, module: &'b Module) -> HashMap<LLVMValue<'b>, TextData<'a>>   {
    let mut ir_map: HashMap<LLVMValue, TextData> = HashMap::new();
    let block = Regex::new(r#"^(".*"|\w*):"#).unwrap();
    let func = Regex::new(r#"@(".*"|\w*)\("#).unwrap();
    let mut current_blocks: Vec<BasicBlock> = Vec::new();
    let mut in_block = false;
    let mut next_inst: Option<InstructionValue> = None;

    for (mut i, line) in lines.iter().enumerate()   {
        i += 1; //enumerate starts at 0 but line numbers start at 1
        let line = line.trim_start_matches(|c: char| c.is_whitespace());
        if in_block {
            //tuple unpacking for assignment is still unstable :(
            let ret = parse_inst(i, &mut next_inst, &mut ir_map);
            next_inst = ret.0;
            in_block = ret.1;
        }
        else if let Some(name_bounds) = block.find(line)    {
            in_block = true;
            let a = name_bounds.start();
            let b = name_bounds.end()-1;
            let ssa_name = strip_quotes(a, b, line);
            next_inst = parse_block(i, ssa_name, current_blocks.clone(), &mut ir_map);
        }
        else if let Some(name_bounds) = func.find(line) {
            let a = name_bounds.start()+1;
            let b = name_bounds.end()-1;
            let ssa_name = strip_quotes(a, b, line);
            current_blocks = parse_func(i, ssa_name, module, &mut ir_map);
        }
    }

    return ir_map;
}

fn main() {
    let ctx = Context::create();
    let module = get_test_ir(&ctx);
    let lines = get_lines();
    let ir_map = parse_ir(&lines, &module);
    for (_, v) in &ir_map{
        println!("{}, {}", v.line_number, v.name);
        break;
    }
}
