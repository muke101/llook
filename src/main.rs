use std::borrow::Borrow;
use std::fs::read_to_string;
use std::process::Command;
use std::path::Path;
use inkwell::module::Module;
use inkwell::context::Context;
use inkwell::values::*;
use inkwell::basic_block::BasicBlock;
use std::collections::HashMap;
use regex::Regex;
use either::Either;

struct TextData<'a> {
    line_number: u32,
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
    Command::new("llvm-as").arg("test.ll");
    let p = Path::new("./test.bc");
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
    let path = Path::new("./test.ll");
    let raw = read_to_string(path).unwrap();
    return raw.lines()
              .map(String::from)
              .collect::<Vec<_>>();
}

fn parse_block_name(line: &str) -> &str {
        let len = line.len();
        let a = 0;
        let b = len-3;
        match line.find(" ")    {
            Some(_) => &line[a+1..b-1],
            None => &line[a..b],
        }
}

fn parse_inst<'a, 'b>(i: u32, next_inst: &mut Option<InstructionValue<'b>>, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> (Option<InstructionValue<'b>>, bool)   {
    let inst = next_inst.unwrap();
    //let ssa_name = inst.get_name();
    let ssa_name = "hello";
    ir_map.insert(LLVMValue::Instruction(inst),
                  TextData { line_number: i, name: ssa_name });
    *next_inst = inst.get_next_instruction();
    if next_inst == &None {
        return (*next_inst, false);
    }
    return (*next_inst, true);
}

fn parse_block<'a, 'b>(i: u32, line: &'a str, current_blocks: Vec<BasicBlock<'b>>, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> Option<InstructionValue<'b>>   {
    let ssa_name = parse_block_name(line);
    for block in current_blocks.iter()  {
        if ssa_name == block.get_name().to_str().unwrap() {
            ir_map.insert(LLVMValue::BasicBlock(*block),
                          TextData { line_number: i as u32, name: ssa_name });
            return block.get_first_instruction();
        }
    }
    return None;
}

fn parse_func<'a, 'b>(i: u32, line: &'a str, func_name: &Regex, module: &'b Module, ir_map: &mut HashMap<LLVMValue<'b>, TextData<'a>>) -> Vec<BasicBlock<'b>>    {
    let ssa_name = func_name.find(line).unwrap();
    let ssa_name = &line[ssa_name.start()..ssa_name.end()-1]; //TODO: might not have to subtract 1 on end
    let func = module.get_function(ssa_name).unwrap();
    ir_map.insert(LLVMValue::Function(func),
                  TextData { line_number: i as u32, name: ssa_name });
    return func.get_basic_blocks();
}

fn test2(foo: &Vec<u32>)    {
    println!("{}", foo[0]);
}

fn test(a: u32, b: u32)   {
    let mut foo: Vec<u32> = Vec::new();
    for i in a..b  {
        if i == 10   {
            foo[0] = 1 as u32;
        }
        else {
            test2(&foo);
        }
    }
}

//TODO: make these functions methods of ir_map, impl get_name

fn parse_ir<'a, 'b>(lines: &'a Vec<String>, module: &'b Module) -> HashMap<LLVMValue<'b>, TextData<'a>>   {
    let mut ir_map: HashMap<LLVMValue, TextData> = HashMap::new();
    let block = Regex::new(r"^.:\n$").unwrap();
    let func = Regex::new(r"^define ").unwrap();
    let func_name = Regex::new(r#".*@(".*"|.*)\("#).unwrap();
    let mut current_blocks: Vec<BasicBlock> = Vec::new();
    let mut in_block = false;
    let mut next_inst: Option<InstructionValue> = None;

    for (i, line) in lines.iter().enumerate()   {
        let line = line.trim_start_matches(|c: char| c.is_whitespace());
        if in_block {
            //tuple unpacking for assignment is still unstable :(
            let ret = parse_inst(i as u32, &mut next_inst, &mut ir_map);
            next_inst = ret.0;
            in_block = ret.1;
        }
        else if block.is_match(line) {
            in_block = true;
            next_inst = parse_block(i as u32, line, current_blocks.clone(), &mut ir_map);
        }
        else if func.is_match(line) {
            current_blocks = parse_func(i as u32, line, &func_name, module, &mut ir_map);
        }
    }

    return ir_map;
}

fn main() {
    let ctx = Context::create();
    let module = get_test_ir(&ctx);
}
