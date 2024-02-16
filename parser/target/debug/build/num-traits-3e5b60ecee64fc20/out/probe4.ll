; ModuleID = 'probe4.80b1d774d8757226-cgu.0'
source_filename = "probe4.80b1d774d8757226-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_19b8b6881a3dee558abb1547c6c18dea = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/58e967a9cc3bd39122e8cb728e8cec6e3a4eeef2\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_de9ccb151f7980044d41f41af3a4b210 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_19b8b6881a3dee558abb1547c6c18dea, [16 x i8] c"K\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17he4b6e7fbe95a5809E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1a951cd17367531fE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h8fa8fc38d8850113E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_de9ccb151f7980044d41f41af3a4b210) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1a951cd17367531fE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h8fa8fc38d8850113E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0-nightly (58e967a9c 2023-09-03)"}
