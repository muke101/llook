#include "llvm_cppwrapper.h"
using namespace llvm;

extern "C"{
    const char *get_name(LLVMValueRef inst){
        return unwrap(inst)->getName().data();
    }
}
