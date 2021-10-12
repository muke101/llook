; ModuleID = 'ir-opt.bc'
source_filename = "hot_code"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; Function Attrs: hot nofree norecurse nounwind willreturn
define i8 @test() {
entry:
%0 = call i8 @test2(i8 1)
ret i8 %0
}

define i8 @test2(i8 %0) {
entry:
%1 = add i8 %0, 1
ret i8 %1
}
