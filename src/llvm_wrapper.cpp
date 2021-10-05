#include "llvm_wrapper.h"
using namespace llvm;

extern "C"{
    const char *get_name(LLVMValueRef inst){
        return unwrap(inst)->getName().data();
    }
}
