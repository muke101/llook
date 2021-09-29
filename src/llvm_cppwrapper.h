#include "llvm-c/Core.h"

#ifdef __cplusplus
extern "C"{
#endif
char *get_name(LLVMValueRef inst);

char *get_name_wrapper(LLVMValueRef inst);
#ifdef __cplusplus
}
#endif
