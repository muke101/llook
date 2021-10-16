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

//FIXME: name can be derived from objects with both 'inp and 'ctx lifetimes actually
struct TextData<'inp> {
    line_number: usize,
    name: &'inp str
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
}

extern "C"  { fn get_name(isnt: InstructionValue) -> *const c_char; }

struct IR<'inp, 'ctx>   {
    lines: &'inp Vec<String>,
    ir_map: HashMap<LLVMValue<'ctx>, TextData<'inp>>
}

impl<'inp, 'ctx> IR<'inp, 'ctx> {

    fn new(input_module: *const Module<'ctx>, input_lines: &'inp Vec<String>) -> Self {
        let mut ir = IR { lines: input_lines, ir_map: HashMap::new()};
        ir.parse_ir(input_module);
        return ir;
    }

    //TODO: get working with functions
    fn get_uses(&self, val: &'ctx LLVMValue) -> Vec<usize>   {
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
            line_numbers.push(self.ir_map.get(&user_inst).unwrap().line_number);
            current_use = current_use.unwrap().get_next_use();
        }

        return line_numbers;
    }

    //TODO: get working with function names and function args
    fn get_defs(&self, inst: &'ctx InstructionValue) -> Vec<&TextData<'inp>>   {
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
                self.ir_map.get(
                    &match inst.get_operand(n).unwrap() {
                        Either::Left(op) => LLVMValue::Instruction(op.as_instruction_value().unwrap()),
                        Either::Right(op) => LLVMValue::BasicBlock(op)
                    }
                ).unwrap()
            )
            .collect::<Vec<_>>()
    }

    //TODO: once you have access to value refs implement basic blocks
    // also consider just moving this into a trait to get rid of the either enum if it doesn't need self
    fn get_preds(self, val: &Either<PhiValue<'ctx>, BasicBlock<'ctx>>) -> Vec<BasicBlock<'ctx>> {
        match val   {
            Either::Left(phi) => (0..phi.count_incoming()).map(|i| phi.get_incoming(i).unwrap().1).collect::<Vec<_>>(),
            Either::Right(block) => panic!("unimplemented")
        }
    }

    fn parse_inst(&mut self, i: usize, next_inst: &mut Option<InstructionValue<'ctx>>) -> (Option<InstructionValue<'ctx>>, bool)   {
        let inst = next_inst.unwrap();
        //FIXME: pass LLVMValueRef
        //ssa_name owned by context
        let ssa_name = unsafe { CStr::from_ptr(get_name(inst)).to_str().unwrap() };
        self.ir_map.insert(LLVMValue::Instruction(inst),
                    TextData { line_number: i, name: ssa_name });
        *next_inst = inst.get_next_instruction();
        if next_inst == &None {
            return (*next_inst, false);
        }
        return (*next_inst, true);
    }

    fn parse_block(&mut self, i: usize, ssa_name: &'inp str, current_blocks: &mut Vec<BasicBlock<'ctx>>) -> Option<InstructionValue<'ctx>>   {
        for block in current_blocks.iter() {
            if ssa_name == block.get_name().to_str().unwrap() {
                self.ir_map.insert(LLVMValue::BasicBlock(*block),
                                   TextData { line_number: i, name: ssa_name });
                return block.get_first_instruction();
            }
        }
        panic!("Block name not found in module!");
    }

    fn parse_func(&mut self, module: *const Module<'ctx>, i: usize, ssa_name: &'inp str) -> Vec<BasicBlock<'ctx>>    {
        let func = unsafe { module.as_ref().unwrap().get_function(ssa_name).unwrap() };
        self.ir_map.insert(LLVMValue::Function(func),
                           TextData { line_number: i, name: ssa_name });
        return func.get_basic_blocks();
    }

    fn parse_ir(&mut self, module: *const Module<'ctx>)   {
        let block = Regex::new(r#"^(".*"|(\w|\.)*):"#).unwrap();
        let func = Regex::new(r"^define ").unwrap();
        let func_name = Regex::new(r#"@(".*"|\w*)\("#).unwrap();
        let mut current_blocks: Vec<BasicBlock> = Vec::new();
        let mut in_block = false;
        let mut next_inst: Option<InstructionValue> = None;

        for (mut i, line) in self.lines.iter().enumerate()   {
            i += 1; //enumerate starts at 0 but line numbers start at 1
            let line = line.trim_start_matches(|c: char| c.is_whitespace());
            if in_block {
                //tuple unpacking for assignment is still unstable :(
                let ret = self.parse_inst(i, &mut next_inst);
                next_inst = ret.0;
                in_block = ret.1;
            }
            else if let Some(name_bounds) = block.find(line)    {
                in_block = true;
                let a = name_bounds.start();
                let b = name_bounds.end()-1;
                let ssa_name = strip_quotes(a, b, line);
                next_inst = self.parse_block(i, ssa_name, &mut current_blocks);
            }
            else if func.is_match(line)  {
                let name_bounds = func_name.find(line).unwrap();
                let a = name_bounds.start()+1;
                let b = name_bounds.end()-1;
                let ssa_name = strip_quotes(a, b, line);
                current_blocks = self.parse_func(module, i, ssa_name);
            }
        }
    }
}

fn strip_quotes(a: usize, b: usize, line: &str) -> &str   {
    let mut ssa_name = &line[a..b];
    if let Some(_) = ssa_name.find("\"") {
        ssa_name = &line[a+1..b-1];
    }
    return ssa_name;
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

fn get_test_block<'ctx>(module: &Module<'ctx>) -> BasicBlock<'ctx>   {
    module.get_first_function()
          .unwrap()
          .get_first_basic_block()
          .unwrap()
          .get_next_basic_block()
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
    let ir = IR::new(&module as *const Module, &lines);
    let inst = get_test_inst(&module);
    let basic_block = get_test_block(&module);
    let text_data = ir.ir_map.get(&LLVMValue::Instruction(inst)).unwrap();
    println!("instruction: {}", text_data);
    let text_data = ir.get_defs(&inst);
    for data in text_data.iter()    {
        println!("{}", data);
    }
    let inst_value = LLVMValue::Instruction(inst);
    let block_value = LLVMValue::BasicBlock(basic_block);
    let line_numbers = ir.get_uses(&inst_value);
    line_numbers.iter().for_each(|n| println!("{}", n));
    let line_numbers = ir.get_uses(&block_value);
    line_numbers.iter().for_each(|n| println!("{}", n));
}
