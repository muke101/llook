#include "llvm_wrapper.h"

char *get_name(LLVMValueRef inst){
    return get_name_wrapper(inst);
}
