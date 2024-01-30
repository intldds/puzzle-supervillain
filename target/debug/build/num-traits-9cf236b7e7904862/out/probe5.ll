; ModuleID = 'probe5.82cb830e20e209a6-cgu.0'
source_filename = "probe5.82cb830e20e209a6-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

@alloc_f002bb4cd3d290148dc859a7da848aa3 = private unnamed_addr constant <{ [84 x i8] }> <{ [84 x i8] c"/private/tmp/rust-20231106-26190-su0a9k/rustc-1.72.1-src/library/core/src/num/mod.rs" }>, align 1
@alloc_88ef748dd11f36dbb00c4d41d92adb91 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_f002bb4cd3d290148dc859a7da848aa3, [16 x i8] c"T\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe5::probe
; Function Attrs: uwtable
define void @_ZN6probe55probe17h349a8eb8e624b56fE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7c4a24da8a0fcdb5E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hc241fa596ee7b5bfE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_88ef748dd11f36dbb00c4d41d92adb91) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7c4a24da8a0fcdb5E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hc241fa596ee7b5bfE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
