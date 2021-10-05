#include <llvm-c/Core.h>
#include <llvm/IR/Instruction.h>

#ifdef __cplusplus
extern "C"{
#endif
const char *get_name(LLVMValueRef inst);
#ifdef __cplusplus
}
#endif
