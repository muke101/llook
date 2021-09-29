#include "llvm_cppwrapper.h"

char *get_name(LLVMValueRef inst){
    return get_name_wrapper(inst);
}
