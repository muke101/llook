; ModuleID = 'ir-opt.bc'
source_filename = "hot_code"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

; Function Attrs: hot nofree norecurse nounwind willreturn
define { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* @"(jitdriver: no get_printable_location)0"({ i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* noalias nocapture returned %0, i8* nocapture readnone %1) #0 {
entry:
  %struct_elem_ptr = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 1
  %struct_elem = load i64, i64* %struct_elem_ptr, align 8
  %struct_elem_ptr1 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 2
  %struct_elem2 = load i64, i64* %struct_elem_ptr1, align 8
  %struct_elem_ptr3 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 3
  %struct_elem4 = load i64, i64* %struct_elem_ptr3, align 8
  %struct_elem_ptr5 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 4
  %struct_elem6 = load i64, i64* %struct_elem_ptr5, align 8
  %struct_elem_ptr7 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 5
  %struct_elem8 = load i64, i64* %struct_elem_ptr7, align 8
  %struct_elem_ptr9 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 6
  %struct_elem10 = load i64, i64* %struct_elem_ptr9, align 8
  %struct_elem_ptr11 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 7
  %struct_elem12 = load i64, i64* %struct_elem_ptr11, align 8
  %struct_elem_ptr13 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 8
  %struct_elem14 = load i64, i64* %struct_elem_ptr13, align 8
  %arg15 = bitcast i64 %struct_elem14 to double
  %int_sub_res = add i64 %struct_elem2, -1
  %int_sub_res27 = add i64 %struct_elem10, -1
  %float_mul_res = fmul double %arg15, 2.500000e-01
  %struct = inttoptr i64 %struct_elem12 to { i64, i8* }*
  %struct_elem_ptr44 = getelementptr inbounds { i64, i8* }, { i64, i8* }* %struct, i64 0, i32 1
  %2 = bitcast i8** %struct_elem_ptr44 to { i64, [0 x i8*] }**
  %float_sub_res = fsub double 1.000000e+00, %arg15
  br label %loop_header.outer

loop_header.outer:                                ; preds = %loop_header.outer.backedge, %entry
  %phi_2.ph = phi i64 [ %struct_elem4, %entry ], [ %phi_2.ph.be, %loop_header.outer.backedge ]
  %phi_3.ph = phi i64 [ %struct_elem6, %entry ], [ %phi_3.lcssa, %loop_header.outer.backedge ]
  %phi_4.ph = phi i64 [ %struct_elem8, %entry ], [ %phi_4.ph.be, %loop_header.outer.backedge ]
  br label %loop_header

loop_header:                                      ; preds = %bridge210, %loop_header.outer
  %phi_3 = phi i64 [ %int_add_res225, %bridge210 ], [ %phi_3.ph, %loop_header.outer ]
  %phi_4 = phi i64 [ 1, %bridge210 ], [ %phi_4.ph, %loop_header.outer ]
  %int_cmp_res = icmp slt i64 %phi_4, %int_sub_res
  br i1 %int_cmp_res, label %resume, label %bridge210

resume:                                           ; preds = %loop_header
  %phi_3.lcssa = phi i64 [ %phi_3, %loop_header ]
  %phi_4.lcssa = phi i64 [ %phi_4, %loop_header ]
  %int_cmp_res28 = icmp slt i64 %phi_2.ph, %int_sub_res27
  br i1 %int_cmp_res28, label %resume30, label %resume180

resume30:                                         ; preds = %resume
  %struct_elem451 = load { i64, [0 x i8*] }*, { i64, [0 x i8*] }** %2, align 8
  %int_sub_res48 = add i64 %phi_4.lcssa, -1
  %int_cmp_res49 = icmp sgt i64 %int_sub_res48, -1
  br i1 %int_cmp_res49, label %resume51, label %bailout50, !prof !0

bailout50:                                        ; preds = %resume30
  %phi_3.lcssa.lcssa46 = phi i64 [ %phi_3.lcssa, %resume30 ]
  %phi_4.lcssa.lcssa42 = phi i64 [ %phi_4.lcssa, %resume30 ]
  %struct_elem_ptr54 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 1
  store i64 3, i64* %struct_elem_ptr54, align 8
  store i64 %int_sub_res, i64* %struct_elem_ptr, align 8
  store i64 %struct_elem10, i64* %struct_elem_ptr1, align 8
  store i64 %phi_4.lcssa.lcssa42, i64* %struct_elem_ptr3, align 8
  store i64 %phi_3.lcssa.lcssa46, i64* %struct_elem_ptr5, align 8
  store i64 %phi_2.ph, i64* %struct_elem_ptr7, align 8
  store i64 %struct_elem2, i64* %struct_elem_ptr9, align 8
  store i64 %struct_elem, i64* %struct_elem_ptr11, align 8
  store i64 %struct_elem12, i64* %struct_elem_ptr13, align 8
  %struct_elem_ptr63 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 9
  store i64 %struct_elem14, i64* %struct_elem_ptr63, align 8
  ret { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0

resume51:                                         ; preds = %resume30
  %sext = shl i64 %phi_2.ph, 32
  %3 = ashr exact i64 %sext, 32
  %struct_elem_ptr46 = getelementptr inbounds { i64, [0 x i8*] }, { i64, [0 x i8*] }* %struct_elem451, i64 0, i32 1, i64 %3
  %4 = bitcast i8** %struct_elem_ptr46 to { i64, i8* }**
  %struct_elem472 = load { i64, i8* }*, { i64, i8* }** %4, align 8
  %struct_elem_ptr65 = getelementptr inbounds { i64, i8* }, { i64, i8* }* %struct_elem472, i64 0, i32 1
  %5 = bitcast i8** %struct_elem_ptr65 to { i64, [0 x double] }**
  %struct_elem663 = load { i64, [0 x double] }*, { i64, [0 x double] }** %5, align 8
  %int_sub_res82 = add i64 %phi_2.ph, -1
  %int_cmp_res83 = icmp sgt i64 %int_sub_res82, -1
  %index79 = shl i64 %phi_4.lcssa, 32
  %6 = ashr exact i64 %index79, 32
  br i1 %int_cmp_res83, label %resume85, label %bailout84, !prof !0

bailout84:                                        ; preds = %resume51
  %phi_3.lcssa.lcssa47 = phi i64 [ %phi_3.lcssa, %resume51 ]
  %phi_4.lcssa.lcssa43 = phi i64 [ %phi_4.lcssa, %resume51 ]
  %struct_elem_ptr89 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 1
  store i64 4, i64* %struct_elem_ptr89, align 8
  store i64 %int_sub_res, i64* %struct_elem_ptr, align 8
  store i64 %struct_elem10, i64* %struct_elem_ptr1, align 8
  store i64 %phi_4.lcssa.lcssa43, i64* %struct_elem_ptr3, align 8
  store i64 %phi_3.lcssa.lcssa47, i64* %struct_elem_ptr5, align 8
  store i64 %phi_2.ph, i64* %struct_elem_ptr7, align 8
  store i64 %struct_elem2, i64* %struct_elem_ptr9, align 8
  store i64 %struct_elem, i64* %struct_elem_ptr11, align 8
  store i64 %struct_elem12, i64* %struct_elem_ptr13, align 8
  %struct_elem_ptr98 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 9
  store i64 %struct_elem14, i64* %struct_elem_ptr98, align 8
  ret { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0

resume85:                                         ; preds = %resume51
  %sext4 = shl i64 %int_sub_res48, 32
  %7 = ashr exact i64 %sext4, 32
  %struct_elem_ptr69 = getelementptr inbounds { i64, [0 x double] }, { i64, [0 x double] }* %struct_elem663, i64 0, i32 1, i64 %7
  %struct_elem70 = load double, double* %struct_elem_ptr69, align 8
  %sext5 = add i64 %index79, 4294967296
  %8 = ashr exact i64 %sext5, 32
  %struct_elem_ptr80 = getelementptr inbounds { i64, [0 x double] }, { i64, [0 x double] }* %struct_elem663, i64 0, i32 1, i64 %8
  %struct_elem81 = load double, double* %struct_elem_ptr80, align 8
  %float_add_res = fadd double %struct_elem70, %struct_elem81
  %sext6 = shl i64 %int_sub_res82, 32
  %9 = ashr exact i64 %sext6, 32
  %struct_elem_ptr101 = getelementptr inbounds { i64, [0 x i8*] }, { i64, [0 x i8*] }* %struct_elem451, i64 0, i32 1, i64 %9
  %10 = bitcast i8** %struct_elem_ptr101 to { i64, i8* }**
  %struct_elem1027 = load { i64, i8* }*, { i64, i8* }** %10, align 8
  %struct_elem_ptr104 = getelementptr inbounds { i64, i8* }, { i64, i8* }* %struct_elem1027, i64 0, i32 1
  %11 = bitcast i8** %struct_elem_ptr104 to { i64, [0 x double] }**
  %struct_elem1058 = load { i64, [0 x double] }*, { i64, [0 x double] }** %11, align 8
  %struct_elem_ptr108 = getelementptr inbounds { i64, [0 x double] }, { i64, [0 x double] }* %struct_elem1058, i64 0, i32 1, i64 %6
  %struct_elem109 = load double, double* %struct_elem_ptr108, align 8
  %float_add_res110 = fadd double %float_add_res, %struct_elem109
  %int_add_res111 = add nsw i64 %phi_2.ph, 1
  %sext10 = shl i64 %int_add_res111, 32
  %12 = ashr exact i64 %sext10, 32
  %struct_elem_ptr114 = getelementptr inbounds { i64, [0 x i8*] }, { i64, [0 x i8*] }* %struct_elem451, i64 0, i32 1, i64 %12
  %13 = bitcast i8** %struct_elem_ptr114 to { i64, i8* }**
  %struct_elem11511 = load { i64, i8* }*, { i64, i8* }** %13, align 8
  %struct_elem_ptr117 = getelementptr inbounds { i64, i8* }, { i64, i8* }* %struct_elem11511, i64 0, i32 1
  %14 = bitcast i8** %struct_elem_ptr117 to { i64, [0 x double] }**
  %struct_elem11812 = load { i64, [0 x double] }*, { i64, [0 x double] }** %14, align 8
  %struct_elem_ptr121 = getelementptr inbounds { i64, [0 x double] }, { i64, [0 x double] }* %struct_elem11812, i64 0, i32 1, i64 %6
  %struct_elem122 = load double, double* %struct_elem_ptr121, align 8
  %float_add_res123 = fadd double %float_add_res110, %struct_elem122
  %float_mul_res124 = fmul double %float_mul_res, %float_add_res123
  %struct_elem_ptr134 = getelementptr inbounds { i64, [0 x double] }, { i64, [0 x double] }* %struct_elem663, i64 0, i32 1, i64 %6
  %struct_elem135 = load double, double* %struct_elem_ptr134, align 8
  %float_mul_res136 = fmul double %float_sub_res, %struct_elem135
  %float_add_res137 = fadd double %float_mul_res124, %float_mul_res136
  store double %float_add_res137, double* %struct_elem_ptr134, align 8
  %int_cmp_res148 = icmp slt i64 %phi_3.lcssa, %struct_elem
  br i1 %int_cmp_res148, label %loop_header.outer.backedge, label %bailout149, !prof !0

loop_header.outer.backedge:                       ; preds = %resume180, %resume85
  %phi_2.ph.be = phi i64 [ 1, %resume180 ], [ %int_add_res111, %resume85 ]
  %phi_4.ph.be = phi i64 [ %int_add_res194, %resume180 ], [ %phi_4.lcssa, %resume85 ]
  br label %loop_header.outer

bailout149:                                       ; preds = %resume85
  %phi_3.lcssa.lcssa48 = phi i64 [ %phi_3.lcssa, %resume85 ]
  %phi_4.lcssa.lcssa44 = phi i64 [ %phi_4.lcssa, %resume85 ]
  %struct_elem_ptr153 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 1
  store i64 5, i64* %struct_elem_ptr153, align 8
  store i64 %struct_elem10, i64* %struct_elem_ptr, align 8
  store i64 %phi_4.lcssa.lcssa44, i64* %struct_elem_ptr1, align 8
  store i64 %phi_3.lcssa.lcssa48, i64* %struct_elem_ptr3, align 8
  store i64 %int_add_res111, i64* %struct_elem_ptr5, align 8
  store i64 %struct_elem2, i64* %struct_elem_ptr7, align 8
  store i64 %struct_elem, i64* %struct_elem_ptr9, align 8
  store i64 %struct_elem12, i64* %struct_elem_ptr11, align 8
  store i64 %struct_elem14, i64* %struct_elem_ptr13, align 8
  ret { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0

resume180:                                        ; preds = %resume
  %int_add_res194 = add nsw i64 %phi_4.lcssa, 1
  %int_cmp_res195 = icmp slt i64 %phi_3.lcssa, %struct_elem
  br i1 %int_cmp_res195, label %loop_header.outer.backedge, label %bailout196, !prof !0

bailout196:                                       ; preds = %resume180
  %phi_3.lcssa.lcssa = phi i64 [ %phi_3.lcssa, %resume180 ]
  %phi_4.lcssa.lcssa = phi i64 [ %phi_4.lcssa, %resume180 ]
  %struct_elem_ptr200 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 1
  store i64 8, i64* %struct_elem_ptr200, align 8
  store i64 %int_sub_res, i64* %struct_elem_ptr, align 8
  store i64 %struct_elem10, i64* %struct_elem_ptr1, align 8
  store i64 %phi_4.lcssa.lcssa, i64* %struct_elem_ptr3, align 8
  store i64 %phi_3.lcssa.lcssa, i64* %struct_elem_ptr5, align 8
  store i64 %phi_2.ph, i64* %struct_elem_ptr7, align 8
  store i64 %struct_elem2, i64* %struct_elem_ptr9, align 8
  store i64 %struct_elem, i64* %struct_elem_ptr11, align 8
  store i64 %struct_elem12, i64* %struct_elem_ptr13, align 8
  %struct_elem_ptr209 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 9
  store i64 %struct_elem14, i64* %struct_elem_ptr209, align 8
  ret { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0

bridge210:                                        ; preds = %loop_header
  %int_add_res225 = add i64 %phi_3, 1
  %int_cmp_res226 = icmp slt i64 %int_add_res225, %struct_elem
  br i1 %int_cmp_res226, label %loop_header, label %bailout227, !prof !0

bailout227:                                       ; preds = %bridge210
  %phi_4.lcssa41 = phi i64 [ %phi_4, %bridge210 ]
  %15 = add i64 %phi_3.ph, 1
  %16 = icmp sgt i64 %struct_elem, %15
  %smax.le = select i1 %16, i64 %struct_elem, i64 %15
  %17 = add i64 %smax.le, -1
  %struct_elem_ptr231 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 1
  store i64 10, i64* %struct_elem_ptr231, align 8
  store i64 %int_sub_res, i64* %struct_elem_ptr, align 8
  store i64 %struct_elem10, i64* %struct_elem_ptr1, align 8
  store i64 %phi_4.lcssa41, i64* %struct_elem_ptr3, align 8
  store i64 %17, i64* %struct_elem_ptr5, align 8
  store i64 %phi_2.ph, i64* %struct_elem_ptr7, align 8
  store i64 %struct_elem2, i64* %struct_elem_ptr9, align 8
  store i64 %struct_elem, i64* %struct_elem_ptr11, align 8
  store i64 %struct_elem12, i64* %struct_elem_ptr13, align 8
  %struct_elem_ptr240 = getelementptr inbounds { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }, { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0, i64 0, i32 7, i64 9
  store i64 %struct_elem14, i64* %struct_elem_ptr240, align 8
  ret { i8*, i64, i64, i8*, i8*, i8*, i8*, [9 x i64] }* %0
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare double @llvm.fabs.f64(double) #1

; Function Attrs: nofree nosync willreturn
declare void @llvm.experimental.stackmap(i64, i32, ...) #2

; Function Attrs: argmemonly nofree nosync nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #3

; Function Attrs: inaccessiblememonly norecurse nounwind speculatable willreturn
define noalias i8* @malloc_wrapper(i64 %0) #4 {
entry:
  %ptr = call i8* inttoptr (i64 140150815502352 to i8* (i64)*)(i64 %0) #5
  %malloc_is_not_null.not = icmp eq i8* %ptr, null
  br i1 %malloc_is_not_null.not, label %error, label %success, !prof !1

success:                                          ; preds = %entry
  ret i8* %ptr

error:                                            ; preds = %entry
  call void inttoptr (i64 140150815502408 to void ()*)() #5
  call void inttoptr (i64 140150815502464 to void ()*)() #5
  ret i8* null
}

attributes #0 = { hot nofree norecurse nounwind willreturn "target-cpu"="znver1" "target-features"="+sse2,-tsxldtrk,+cx16,+sahf,-tbm,-avx512ifma,+sha,-gfni,-fma4,-vpclmulqdq,+prfchw,+bmi2,-cldemote,+fsgsbase,-ptwrite,-amx-tile,-uintr,+popcnt,-widekl,+aes,-avx512bitalg,-movdiri,+xsaves,-avx512er,-avxvnni,-avx512vnni,-amx-bf16,-avx512vpopcntdq,-pconfig,-clwb,-avx512f,+xsavec,+clzero,-pku,+mmx,-lwp,-rdpid,-xop,+rdseed,-waitpkg,-kl,-movdir64b,+sse4a,-avx512bw,+clflushopt,+xsave,-avx512vbmi2,+64bit,-avx512vl,-serialize,-hreset,-invpcid,-avx512cd,+avx,-vaes,-avx512bf16,+cx8,+fma,-rtm,+bmi,-enqcmd,+rdrnd,+mwaitx,+sse4.1,+sse4.2,+avx2,+fxsr,-wbnoinvd,+sse,+lzcnt,+pclmul,-prefetchwt1,+f16c,+ssse3,-sgx,-shstk,+cmov,-avx512vbmi,-amx-int8,+movbe,-avx512vp2intersect,+xsaveopt,-avx512dq,+adx,-avx512pf,+sse3" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nofree nosync willreturn }
attributes #3 = { argmemonly nofree nosync nounwind willreturn writeonly }
attributes #4 = { inaccessiblememonly norecurse nounwind speculatable willreturn "target-cpu"="znver1" "target-features"="+sse2,-tsxldtrk,+cx16,+sahf,-tbm,-avx512ifma,+sha,-gfni,-fma4,-vpclmulqdq,+prfchw,+bmi2,-cldemote,+fsgsbase,-ptwrite,-amx-tile,-uintr,+popcnt,-widekl,+aes,-avx512bitalg,-movdiri,+xsaves,-avx512er,-avxvnni,-avx512vnni,-amx-bf16,-avx512vpopcntdq,-pconfig,-clwb,-avx512f,+xsavec,+clzero,-pku,+mmx,-lwp,-rdpid,-xop,+rdseed,-waitpkg,-kl,-movdir64b,+sse4a,-avx512bw,+clflushopt,+xsave,-avx512vbmi2,+64bit,-avx512vl,-serialize,-hreset,-invpcid,-avx512cd,+avx,-vaes,-avx512bf16,+cx8,+fma,-rtm,+bmi,-enqcmd,+rdrnd,+mwaitx,+sse4.1,+sse4.2,+avx2,+fxsr,-wbnoinvd,+sse,+lzcnt,+pclmul,-prefetchwt1,+f16c,+ssse3,-sgx,-shstk,+cmov,-avx512vbmi,-amx-int8,+movbe,-avx512vp2intersect,+xsaveopt,-avx512dq,+adx,-avx512pf,+sse3" }
attributes #5 = { nofree norecurse nosync nounwind willreturn }

!0 = !{!"guard_weights", i32 100, i32 0}
!1 = !{!"malloc_error_weights", i32 100, i32 0}
