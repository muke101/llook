#include "llvm_wrapper.h"

const char *get_name(LLVMValueRef inst){
    return get_name_wrapper(inst);
}
