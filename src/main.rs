use core::fmt;
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

struct TextData<'inp> {
    line_number: usize,
    name: &'inp str,
}

impl fmt::Display for TextData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        let name = match self.name.len()  {
            0 => "<nameless instruction>",
            _ => self.name,
        };
        write!(f, "{}, {}", self.line_number, name)
    }
}

#[derive(PartialEq, Eq, Hash)]
enum LLVMValue<'inp>  {
    Basic(BasicValueEnum<'inp>),
    Instruction(InstructionValue<'inp>),
    BasicBlock(BasicBlock<'inp>),
    Function(FunctionValue<'inp>),
}

// fn get_preds<'inp, 'ctx, 'c>(inst: &'ctx InstructionValue, ir_map: &'c HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Vec<&'c TextData<'inp>>   {

// }

fn get_defs<'inp, 'b, 'ctx>(inst: &'ctx InstructionValue, ir_map: &'b HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Vec<&'b TextData<'inp>>   {
    (0..inst.get_num_operands())
        .filter(|n|
                match inst.get_operand(*n).unwrap()  {
                    Either::Left(op) =>
                        match op.as_instruction_value() {
                            Some(_) => true,
                            None => false
                        },
                    Either::Right(_) => true
                }
        )
        .map(|n|
             ir_map.get(
                 &match inst.get_operand(n).unwrap() {
                     Either::Left(op) => LLVMValue::Instruction(op.as_instruction_value().unwrap()),
                     Either::Right(op) => LLVMValue::BasicBlock(op)
                 }
             ).unwrap()
        )
        .collect::<Vec<_>>()
}

extern "C"  { fn get_name(isnt: InstructionValue) -> *const c_char; }

fn parse_inst<'inp, 'ctx>(i: usize, next_inst: &mut Option<InstructionValue<'ctx>>, ir_map: &mut HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> (Option<InstructionValue<'ctx>>, bool)   {
    let inst = next_inst.unwrap();
    //FIXME: pass LLVMValueRef
    //ssa_name owned by context
    let ssa_name = unsafe { CStr::from_ptr(get_name(inst)).to_str().unwrap() };
    ir_map.insert(LLVMValue::Instruction(inst),
                  TextData { line_number: i, name: ssa_name });
    *next_inst = inst.get_next_instruction();
    if next_inst == &None {
        return (*next_inst, false);
    }
    return (*next_inst, true);
}

fn parse_block<'inp, 'ctx>(i: usize, ssa_name: &'inp str, current_blocks: &mut Vec<BasicBlock<'ctx>>, ir_map: &mut HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Option<InstructionValue<'ctx>>   {
    for block in current_blocks.iter() {
        if ssa_name == block.get_name().to_str().unwrap() {
            ir_map.insert(LLVMValue::BasicBlock(*block),
                          TextData { line_number: i, name: ssa_name });
            return block.get_first_instruction();
        }
    }
    panic!("Block name not found in module!");
}

fn parse_func<'inp, 'ctx>(i: usize, ssa_name: &'inp str, module: &'ctx Module, ir_map: &mut HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Vec<BasicBlock<'ctx>>    {
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

fn parse_ir<'inp, 'ctx>(lines: &'inp Vec<String>, module: &'ctx Module) -> HashMap<LLVMValue<'ctx>, TextData<'inp>>   {
    let mut ir_map: HashMap<LLVMValue, TextData> = HashMap::new();
    let block = Regex::new(r#"^(".*"|(\w|\.)*):"#).unwrap();
    let func = Regex::new(r"^define ").unwrap();
    let func_name = Regex::new(r#"@(".*"|\w*)\("#).unwrap();
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
            next_inst = parse_block(i, ssa_name, &mut current_blocks, &mut ir_map);
        }
        else if func.is_match(line)  {
            let name_bounds = func_name.find(line).unwrap();
            let a = name_bounds.start()+1;
            let b = name_bounds.end()-1;
            let ssa_name = strip_quotes(a, b, line);
            current_blocks = parse_func(i, ssa_name, module, &mut ir_map);
        }
    }

    return ir_map;
}

fn get_test_inst<'ctx>(module: &Module<'ctx>) -> InstructionValue<'ctx>   {
    module.get_first_function()
          .unwrap()
          .get_first_basic_block()
          .unwrap()
          .get_last_instruction()
          .unwrap()
}

fn get_test_ir<'ctx>(ctx: &'ctx Context, ir_name: &str) -> Module<'ctx>  {
    let ll_path = format!("/home/muke/Programming/llook/test_ir/{}.ll", ir_name);
    let bc_path = format!("/home/muke/Programming/llook/test_ir/{}.bc", ir_name);
    Command::new("llvm-as").arg(ll_path);
    let p = Path::new(&bc_path);
    return Module::parse_bitcode_from_path(p, ctx).unwrap();
}

fn get_test_lines(ir_name: &str) -> Vec<String>  {
    let ll_path = format!("/home/muke/Programming/llook/test_ir/{}.ll", ir_name);
    let path = Path::new(&ll_path);
    let raw = read_to_string(path).unwrap();
    return raw.lines()
              .map(String::from)
              .collect::<Vec<_>>();
}

fn main() {
    let ctx = Context::create(); //'ctx lifetime
    let ir_name = "test_basic";
    let module = get_test_ir(&ctx, ir_name);
    let lines = get_test_lines(ir_name); //'inp lifetime
    let ir_map = parse_ir(&lines, &module);
    let inst = get_test_inst(&module);
    let text_data = ir_map.get(&LLVMValue::Instruction(inst)).unwrap();
    println!("instruction: {}", text_data);
    let text_data = get_defs(&inst, &ir_map);
    for data in text_data.iter()    {
        println!("{}", data);
    }
}
