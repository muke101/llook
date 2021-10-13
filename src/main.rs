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
use regex::Regex;
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
enum LLVMValue<'ctx>  {
    Basic(BasicValueEnum<'ctx>),
    Instruction(InstructionValue<'ctx>),
    BasicBlock(BasicBlock<'ctx>),
    Function(FunctionValue<'ctx>),
}

impl<'ctx> LLVMValue<'ctx> {
    fn get_first_use(&self) -> Option<BasicValueUse>    {
        match self  {
            LLVMValue::Basic(val) => val.get_first_use(),
            LLVMValue::Instruction(val) => val.get_first_use(),
            LLVMValue::BasicBlock(val) => val.get_first_use(),
            LLVMValue::Function(_) => panic!("function uses unimplemented")
        }
    }

    fn get_next_use(&self, last_use: BasicValueUse<'ctx>) -> Option<BasicValueUse<'ctx>>    {
        match self  {
            LLVMValue::Basic(_) => last_use.get_next_use(),
            LLVMValue::Instruction(_) => last_use.get_next_use(),
            LLVMValue::BasicBlock(_) => last_use.get_next_use(),
            LLVMValue::Function(_) => panic!("function uses unimplemented")
        }
    }
}

//TODO: get working with functions
fn get_uses<'inp, 'a, 'ctx>(val: &'ctx LLVMValue, ir_map: &'a HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Vec<usize>   {
    let mut current_use = val.get_first_use();
    let mut line_numbers: Vec<usize> = Vec::new();

    while current_use != None   {
        let user = current_use.unwrap().get_user();
        let user_inst = match user  {
            AnyValueEnum::FloatValue(v) => v.as_instruction(),
            AnyValueEnum::IntValue(v) => v.as_instruction(),
            AnyValueEnum::PointerValue(v) => v.as_instruction(),
            AnyValueEnum::StructValue(v) => v.as_instruction(),
            AnyValueEnum::ArrayValue(v) => v.as_instruction(),
            AnyValueEnum::VectorValue(v) => v.as_instruction(),
            AnyValueEnum::PhiValue(v) => Some(v.as_instruction()),
            AnyValueEnum::InstructionValue(v) => Some(v),
            AnyValueEnum::FunctionValue(_) => None
        };
        if user_inst == None    {
            panic!("user is not an instruction");
        }
        let user_inst = LLVMValue::Instruction(user_inst.unwrap());
        line_numbers.push(ir_map.get(&user_inst).unwrap().line_number);
        current_use = current_use.unwrap().get_next_use();
    }

    return line_numbers;
}

//TODO: get working with phi nodes, function names and function args
fn get_defs<'inp, 'a, 'ctx>(inst: &'ctx InstructionValue, ir_map: &'a HashMap<LLVMValue<'ctx>, TextData<'inp>>) -> Vec<&'a TextData<'inp>>   {
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
          .get_previous_instruction()
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
    let line_numbers = get_uses(&LLVMValue::Instruction(inst), &ir_map);
    line_numbers.iter().for_each(|n| println!("{}", n));

}
