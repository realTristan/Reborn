; ModuleID = 'probe1.b457ac1c-cgu.0'
source_filename = "probe1.b457ac1c-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::sync::atomic::AtomicUsize" = type { i64 }
%"std::sys_common::mutex::MovableMutex" = type { %"std::sys::windows::locks::mutex::Mutex" }
%"std::sys::windows::locks::mutex::Mutex" = type { %"core::cell::UnsafeCell<std::sys::windows::c::SRWLOCK>" }
%"core::cell::UnsafeCell<std::sys::windows::c::SRWLOCK>" = type { %"std::sys::windows::c::SRWLOCK" }
%"std::sys::windows::c::SRWLOCK" = type { i8* }
%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>" = type { i64, [2 x i64] }
%"std::sync::mutex::Mutex<i32>" = type { %"std::sys_common::mutex::MovableMutex", %"std::sync::poison::Flag", [3 x i8], i32 }
%"std::sync::poison::Flag" = type { %"core::sync::atomic::AtomicBool" }
%"core::sync::atomic::AtomicBool" = type { i8 }
%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err" = type { [1 x i64], { i64*, i8 } }
%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok" = type { [1 x i64], { i64*, i8 } }
%"core::fmt::Arguments" = type { { [0 x { [0 x i8]*, i64 }]*, i64 }, { i64*, i64 }, { [0 x { i8*, i64* }]*, i64 } }
%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"core::fmt::Formatter" = type { { i64, i64 }, { i64, i64 }, { {}*, [3 x i64]* }, i32, i32, i8, [7 x i8] }
%"core::fmt::builders::DebugStruct" = type { %"core::fmt::Formatter"*, i8, i8, [6 x i8] }

@_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17hc7328e6d1006a770E = external dllimport global %"core::sync::atomic::AtomicUsize"
@alloc41 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc42 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [12 x i8] }>, <{ [12 x i8] }>* @alloc41, i32 0, i32 0, i32 0), [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc64 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc65 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f\\library\\core\\src\\fmt\\mod.rs" }>, align 1
@alloc66 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc65, i32 0, i32 0, i32 0), [16 x i8] c"K\00\00\00\00\00\00\00\87\01\00\00\0D\00\00\00" }>, align 8
@alloc46 = private unnamed_addr constant <{ [49 x i8] }> <{ [49 x i8] c"there is no such thing as an acquire/release load" }>, align 1
@alloc47 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [49 x i8] }>, <{ [49 x i8] }>* @alloc46, i32 0, i32 0, i32 0), [8 x i8] c"1\00\00\00\00\00\00\00" }>, align 8
@alloc73 = private unnamed_addr constant <{ [79 x i8] }> <{ [79 x i8] c"/rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f\\library\\core\\src\\sync\\atomic.rs" }>, align 1
@alloc68 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc73, i32 0, i32 0, i32 0), [16 x i8] c"O\00\00\00\00\00\00\00$\0A\00\00\17\00\00\00" }>, align 8
@alloc51 = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"there is no such thing as a release load" }>, align 1
@alloc52 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @alloc51, i32 0, i32 0, i32 0), [8 x i8] c"(\00\00\00\00\00\00\00" }>, align 8
@alloc70 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc73, i32 0, i32 0, i32 0), [16 x i8] c"O\00\00\00\00\00\00\00#\0A\00\00\18\00\00\00" }>, align 8
@alloc56 = private unnamed_addr constant <{ [50 x i8] }> <{ [50 x i8] c"there is no such thing as an acquire/release store" }>, align 1
@alloc57 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [50 x i8] }>, <{ [50 x i8] }>* @alloc56, i32 0, i32 0, i32 0), [8 x i8] c"2\00\00\00\00\00\00\00" }>, align 8
@alloc72 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc73, i32 0, i32 0, i32 0), [16 x i8] c"O\00\00\00\00\00\00\00\16\0A\00\00\17\00\00\00" }>, align 8
@alloc61 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"there is no such thing as an acquire store" }>, align 1
@alloc62 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [42 x i8] }>, <{ [42 x i8] }>* @alloc61, i32 0, i32 0, i32 0), [8 x i8] c"*\00\00\00\00\00\00\00" }>, align 8
@alloc74 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [79 x i8] }>, <{ [79 x i8] }>* @alloc73, i32 0, i32 0, i32 0), [16 x i8] c"O\00\00\00\00\00\00\00\15\0A\00\00\18\00\00\00" }>, align 8
@alloc75 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ({ i64*, i8 }*)* @"_ZN4core3ptr98drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$$GT$17h732bdcd435bb494aE" to i8*), [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 ({ i64*, i8 }*, %"core::fmt::Formatter"*)* @"_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h11d1fe98498ee2fdE" to i8*) }>, align 8
@alloc79 = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"PoisonError" }>, align 1
@alloc80 = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"<anon>" }>, align 1
@alloc81 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [6 x i8] }>, <{ [6 x i8] }>* @alloc80, i32 0, i32 0, i32 0), [16 x i8] c"\06\00\00\00\00\00\00\00\04\00\00\00\16\00\00\00" }>, align 8
@_ZN6probe15probe8MY_MUTEX17h19c69f431d5cf48fE = internal global <{ [9 x i8], [3 x i8], [4 x i8] }> <{ [9 x i8] zeroinitializer, [3 x i8] undef, [4 x i8] c"\01\00\00\00" }>, align 8

; std::sys_common::mutex::MovableMutex::raw_unlock
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std10sys_common5mutex12MovableMutex10raw_unlock17hb890a42d59020666E(%"std::sys_common::mutex::MovableMutex"* align 8 %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"std::sys_common::mutex::MovableMutex"* %self to %"std::sys::windows::locks::mutex::Mutex"*
; call std::sys::windows::locks::mutex::Mutex::unlock
  call void @_ZN3std3sys7windows5locks5mutex5Mutex6unlock17hee28a6845c68758cE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; std::sys_common::mutex::MovableMutex::raw_lock
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std10sys_common5mutex12MovableMutex8raw_lock17h89fc68aa377c4a33E(%"std::sys_common::mutex::MovableMutex"* align 8 %self) unnamed_addr #0 {
start:
  %_2 = bitcast %"std::sys_common::mutex::MovableMutex"* %self to %"std::sys::windows::locks::mutex::Mutex"*
; call std::sys::windows::locks::mutex::Mutex::lock
  call void @_ZN3std3sys7windows5locks5mutex5Mutex4lock17hd022e0431409bf1bE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; std::sys::windows::locks::mutex::raw
; Function Attrs: inlinehint uwtable
define internal %"std::sys::windows::c::SRWLOCK"* @_ZN3std3sys7windows5locks5mutex3raw17hd3d133dd3a13c12eE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %m) unnamed_addr #0 {
start:
  %_2 = bitcast %"std::sys::windows::locks::mutex::Mutex"* %m to %"core::cell::UnsafeCell<std::sys::windows::c::SRWLOCK>"*
  %_2.i = bitcast %"core::cell::UnsafeCell<std::sys::windows::c::SRWLOCK>"* %_2 to %"std::sys::windows::c::SRWLOCK"*
  br label %bb1

bb1:                                              ; preds = %start
  ret %"std::sys::windows::c::SRWLOCK"* %_2.i
}

; std::sys::windows::locks::mutex::Mutex::lock
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std3sys7windows5locks5mutex5Mutex4lock17hd022e0431409bf1bE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %self) unnamed_addr #0 {
start:
; call std::sys::windows::locks::mutex::raw
  %_3 = call %"std::sys::windows::c::SRWLOCK"* @_ZN3std3sys7windows5locks5mutex3raw17hd3d133dd3a13c12eE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  call void @AcquireSRWLockExclusive(%"std::sys::windows::c::SRWLOCK"* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; std::sys::windows::locks::mutex::Mutex::unlock
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std3sys7windows5locks5mutex5Mutex6unlock17hee28a6845c68758cE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %self) unnamed_addr #0 {
start:
; call std::sys::windows::locks::mutex::raw
  %_3 = call %"std::sys::windows::c::SRWLOCK"* @_ZN3std3sys7windows5locks5mutex3raw17hd3d133dd3a13c12eE(%"std::sys::windows::locks::mutex::Mutex"* align 8 %self)
  br label %bb1

bb1:                                              ; preds = %start
  call void @ReleaseSRWLockExclusive(%"std::sys::windows::c::SRWLOCK"* %_3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; std::sync::mutex::Mutex<T>::lock
; Function Attrs: uwtable
define void @"_ZN3std4sync5mutex14Mutex$LT$T$GT$4lock17had30de60cf66c219E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %0, %"std::sync::mutex::Mutex<i32>"* align 8 %self) unnamed_addr #1 {
start:
  %_3 = bitcast %"std::sync::mutex::Mutex<i32>"* %self to %"std::sys_common::mutex::MovableMutex"*
; call std::sys_common::mutex::MovableMutex::raw_lock
  call void @_ZN3std10sys_common5mutex12MovableMutex8raw_lock17h89fc68aa377c4a33E(%"std::sys_common::mutex::MovableMutex"* align 8 %_3)
  br label %bb1

bb1:                                              ; preds = %start
; call std::sync::mutex::MutexGuard<T>::new
  call void @"_ZN3std4sync5mutex19MutexGuard$LT$T$GT$3new17h2543b24a68f41bd0E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %0, %"std::sync::mutex::Mutex<i32>"* align 8 %self)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; std::sync::mutex::MutexGuard<T>::new
; Function Attrs: uwtable
define void @"_ZN3std4sync5mutex19MutexGuard$LT$T$GT$3new17h2543b24a68f41bd0E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %0, %"std::sync::mutex::Mutex<i32>"* align 8 %lock) unnamed_addr #1 {
start:
  %_4 = alloca i64*, align 8
  %_3 = getelementptr inbounds %"std::sync::mutex::Mutex<i32>", %"std::sync::mutex::Mutex<i32>"* %lock, i32 0, i32 1
; call std::sync::poison::Flag::guard
  %1 = call { i8, i8 } @_ZN3std4sync6poison4Flag5guard17h3666706c94a22579E(%"std::sync::poison::Flag"* align 1 %_3)
  %2 = extractvalue { i8, i8 } %1, 0
  %_2.0 = trunc i8 %2 to i1
  %_2.1 = extractvalue { i8, i8 } %1, 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = bitcast i64** %_4 to %"std::sync::mutex::Mutex<i32>"**
  store %"std::sync::mutex::Mutex<i32>"* %lock, %"std::sync::mutex::Mutex<i32>"** %3, align 8
  %4 = load i64*, i64** %_4, align 8, !nonnull !1, !align !2, !noundef !1
; call std::sync::poison::map_result
  call void @_ZN3std4sync6poison10map_result17h2a4a68b929f8a1bfE(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %0, i1 zeroext %_2.0, i8 %_2.1, i64* align 8 %4)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; std::sync::mutex::MutexGuard<T>::new::{{closure}}
; Function Attrs: inlinehint uwtable
define { i64*, i8 } @"_ZN3std4sync5mutex19MutexGuard$LT$T$GT$3new28_$u7b$$u7b$closure$u7d$$u7d$17h34770ce5184b6028E"(i64* align 8 %_1, i1 zeroext %guard) unnamed_addr #0 {
start:
  %0 = alloca { i64*, i8 }, align 8
  %_5 = bitcast i64* %_1 to %"std::sync::mutex::Mutex<i32>"*
  %1 = bitcast { i64*, i8 }* %0 to %"std::sync::mutex::Mutex<i32>"**
  store %"std::sync::mutex::Mutex<i32>"* %_5, %"std::sync::mutex::Mutex<i32>"** %1, align 8
  %2 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 1
  %3 = zext i1 %guard to i8
  store i8 %3, i8* %2, align 8
  %4 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 0
  %5 = load i64*, i64** %4, align 8, !nonnull !1, !align !2, !noundef !1
  %6 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 1
  %7 = load i8, i8* %6, align 8, !range !3, !noundef !1
  %8 = trunc i8 %7 to i1
  %9 = zext i1 %8 to i8
  %10 = insertvalue { i64*, i8 } undef, i64* %5, 0
  %11 = insertvalue { i64*, i8 } %10, i8 %9, 1
  ret { i64*, i8 } %11
}

; std::sync::poison::map_result
; Function Attrs: uwtable
define void @_ZN3std4sync6poison10map_result17h2a4a68b929f8a1bfE(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %0, i1 zeroext %1, i8 %2, i64* align 8 %f) unnamed_addr #1 {
start:
  %_13 = alloca i8, align 1
  %_7 = alloca i8, align 1
  %result = alloca { i8, i8 }, align 1
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %result, i32 0, i32 0
  %4 = zext i1 %1 to i8
  store i8 %4, i8* %3, align 1
  %5 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %result, i32 0, i32 1
  store i8 %2, i8* %5, align 1
  %6 = bitcast { i8, i8 }* %result to i8*
  %7 = load i8, i8* %6, align 1, !range !3, !noundef !1
  %8 = trunc i8 %7 to i1
  %_3 = zext i1 %8 to i64
  switch i64 %_3, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %9 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %result, i32 0, i32 1
  %10 = load i8, i8* %9, align 1, !range !3, !noundef !1
  %t = trunc i8 %10 to i1
  %11 = zext i1 %t to i8
  store i8 %11, i8* %_7, align 1
  %12 = load i8, i8* %_7, align 1, !range !3, !noundef !1
  %13 = trunc i8 %12 to i1
; call std::sync::mutex::MutexGuard<T>::new::{{closure}}
  %14 = call { i64*, i8 } @"_ZN3std4sync5mutex19MutexGuard$LT$T$GT$3new28_$u7b$$u7b$closure$u7d$$u7d$17h34770ce5184b6028E"(i64* align 8 %f, i1 zeroext %13)
  %_5.0 = extractvalue { i64*, i8 } %14, 0
  %15 = extractvalue { i64*, i8 } %14, 1
  %_5.1 = trunc i8 %15 to i1
  br label %bb4

bb1:                                              ; preds = %start
  %16 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %result, i32 0, i32 1
  %17 = load i8, i8* %16, align 1, !range !3, !noundef !1
  %guard = trunc i8 %17 to i1
  %18 = zext i1 %guard to i8
  store i8 %18, i8* %_13, align 1
  %19 = load i8, i8* %_13, align 1, !range !3, !noundef !1
  %20 = trunc i8 %19 to i1
; call std::sync::mutex::MutexGuard<T>::new::{{closure}}
  %21 = call { i64*, i8 } @"_ZN3std4sync5mutex19MutexGuard$LT$T$GT$3new28_$u7b$$u7b$closure$u7d$$u7d$17h34770ce5184b6028E"(i64* align 8 %f, i1 zeroext %20)
  %_11.0 = extractvalue { i64*, i8 } %21, 0
  %22 = extractvalue { i64*, i8 } %21, 1
  %_11.1 = trunc i8 %22 to i1
  br label %bb5

bb5:                                              ; preds = %bb1
; call std::sync::poison::PoisonError<T>::new
  %23 = call { i64*, i8 } @"_ZN3std4sync6poison20PoisonError$LT$T$GT$3new17h90dbfcea8e73d4cdE"(i64* align 8 %_11.0, i1 zeroext %_11.1)
  %_10.0 = extractvalue { i64*, i8 } %23, 0
  %24 = extractvalue { i64*, i8 } %23, 1
  %_10.1 = trunc i8 %24 to i1
  br label %bb6

bb6:                                              ; preds = %bb5
  %25 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %0 to %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err"*
  %26 = getelementptr inbounds %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err", %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err"* %25, i32 0, i32 1
  %27 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %26, i32 0, i32 0
  store i64* %_10.0, i64** %27, align 8
  %28 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %26, i32 0, i32 1
  %29 = zext i1 %_10.1 to i8
  store i8 %29, i8* %28, align 8
  %30 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %0 to i64*
  store i64 1, i64* %30, align 8
  br label %bb7

bb7:                                              ; preds = %bb4, %bb6
  ret void

bb4:                                              ; preds = %bb3
  %31 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %0 to %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok"*
  %32 = getelementptr inbounds %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok", %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok"* %31, i32 0, i32 1
  %33 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %32, i32 0, i32 0
  store i64* %_5.0, i64** %33, align 8
  %34 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %32, i32 0, i32 1
  %35 = zext i1 %_5.1 to i8
  store i8 %35, i8* %34, align 8
  %36 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %0 to i64*
  store i64 0, i64* %36, align 8
  br label %bb7
}

; std::sync::poison::PoisonError<T>::new
; Function Attrs: uwtable
define zeroext i1 @"_ZN3std4sync6poison20PoisonError$LT$T$GT$3new17h00ad21c109e4d095E"(i1 zeroext %guard) unnamed_addr #1 {
start:
  %0 = alloca i8, align 1
  %1 = zext i1 %guard to i8
  store i8 %1, i8* %0, align 1
  %2 = load i8, i8* %0, align 1, !range !3, !noundef !1
  %3 = trunc i8 %2 to i1
  ret i1 %3
}

; std::sync::poison::PoisonError<T>::new
; Function Attrs: uwtable
define { i64*, i8 } @"_ZN3std4sync6poison20PoisonError$LT$T$GT$3new17h90dbfcea8e73d4cdE"(i64* align 8 %guard.0, i1 zeroext %guard.1) unnamed_addr #1 {
start:
  %0 = alloca { i64*, i8 }, align 8
  %1 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 0
  store i64* %guard.0, i64** %1, align 8
  %2 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 1
  %3 = zext i1 %guard.1 to i8
  store i8 %3, i8* %2, align 8
  %4 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 0
  %5 = load i64*, i64** %4, align 8, !nonnull !1, !align !2, !noundef !1
  %6 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %0, i32 0, i32 1
  %7 = load i8, i8* %6, align 8, !range !3, !noundef !1
  %8 = trunc i8 %7 to i1
  %9 = zext i1 %8 to i8
  %10 = insertvalue { i64*, i8 } undef, i64* %5, 0
  %11 = insertvalue { i64*, i8 } %10, i8 %9, 1
  ret { i64*, i8 } %11
}

; std::sync::poison::Flag::get
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN3std4sync6poison4Flag3get17h2cab2a5a164c6fefE(%"std::sync::poison::Flag"* align 1 %self) unnamed_addr #0 {
start:
  %_3 = alloca i8, align 1
  %_2 = bitcast %"std::sync::poison::Flag"* %self to %"core::sync::atomic::AtomicBool"*
  store i8 0, i8* %_3, align 1
  %0 = load i8, i8* %_3, align 1, !range !4, !noundef !1
; call core::sync::atomic::AtomicBool::load
  %1 = call zeroext i1 @_ZN4core4sync6atomic10AtomicBool4load17h5d5317445a2ceb3aE(%"core::sync::atomic::AtomicBool"* align 1 %_2, i8 %0)
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %1
}

; std::sync::poison::Flag::done
; Function Attrs: inlinehint uwtable
define internal void @_ZN3std4sync6poison4Flag4done17h60ae6d453c9d1345E(%"std::sync::poison::Flag"* align 1 %self, i8* align 1 %guard) unnamed_addr #0 {
start:
  %_9 = alloca i8, align 1
  %_3 = alloca i8, align 1
  %0 = load i8, i8* %guard, align 1, !range !3, !noundef !1
  %_5 = trunc i8 %0 to i1
  %_4 = xor i1 %_5, true
  br i1 %_4, label %bb2, label %bb1

bb1:                                              ; preds = %start
  store i8 0, i8* %_3, align 1
  br label %bb3

bb2:                                              ; preds = %start
; call std::thread::panicking
  %_6 = call zeroext i1 @_ZN3std6thread9panicking17h925977b625684a73E()
  br label %bb4

bb4:                                              ; preds = %bb2
  %1 = zext i1 %_6 to i8
  store i8 %1, i8* %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb1, %bb4
  %2 = load i8, i8* %_3, align 1, !range !3, !noundef !1
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb5, label %bb7

bb7:                                              ; preds = %bb6, %bb3
  ret void

bb5:                                              ; preds = %bb3
  %_8 = bitcast %"std::sync::poison::Flag"* %self to %"core::sync::atomic::AtomicBool"*
  store i8 0, i8* %_9, align 1
  %4 = load i8, i8* %_9, align 1, !range !4, !noundef !1
; call core::sync::atomic::AtomicBool::store
  call void @_ZN4core4sync6atomic10AtomicBool5store17ha82b9faeb3a2c2b8E(%"core::sync::atomic::AtomicBool"* align 1 %_8, i1 zeroext true, i8 %4)
  br label %bb6

bb6:                                              ; preds = %bb5
  br label %bb7
}

; std::sync::poison::Flag::guard
; Function Attrs: inlinehint uwtable
define internal { i8, i8 } @_ZN3std4sync6poison4Flag5guard17h3666706c94a22579E(%"std::sync::poison::Flag"* align 1 %self) unnamed_addr #0 {
start:
  %ret = alloca i8, align 1
  %0 = alloca { i8, i8 }, align 1
; call std::thread::panicking
  %_3 = call zeroext i1 @_ZN3std6thread9panicking17h925977b625684a73E()
  br label %bb1

bb1:                                              ; preds = %start
  %1 = zext i1 %_3 to i8
  store i8 %1, i8* %ret, align 1
; call std::sync::poison::Flag::get
  %_4 = call zeroext i1 @_ZN3std4sync6poison4Flag3get17h2cab2a5a164c6fefE(%"std::sync::poison::Flag"* align 1 %self)
  br label %bb2

bb2:                                              ; preds = %bb1
  br i1 %_4, label %bb3, label %bb5

bb5:                                              ; preds = %bb2
  %2 = load i8, i8* %ret, align 1, !range !3, !noundef !1
  %_8 = trunc i8 %2 to i1
  %3 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1
  %4 = zext i1 %_8 to i8
  store i8 %4, i8* %3, align 1
  %5 = bitcast { i8, i8 }* %0 to i8*
  store i8 0, i8* %5, align 1
  br label %bb6

bb3:                                              ; preds = %bb2
  %6 = load i8, i8* %ret, align 1, !range !3, !noundef !1
  %_7 = trunc i8 %6 to i1
; call std::sync::poison::PoisonError<T>::new
  %_6 = call zeroext i1 @"_ZN3std4sync6poison20PoisonError$LT$T$GT$3new17h00ad21c109e4d095E"(i1 zeroext %_7)
  br label %bb4

bb4:                                              ; preds = %bb3
  %7 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1
  %8 = zext i1 %_6 to i8
  store i8 %8, i8* %7, align 1
  %9 = bitcast { i8, i8 }* %0 to i8*
  store i8 1, i8* %9, align 1
  br label %bb6

bb6:                                              ; preds = %bb5, %bb4
  %10 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 0
  %11 = load i8, i8* %10, align 1, !range !3, !noundef !1
  %12 = trunc i8 %11 to i1
  %13 = getelementptr inbounds { i8, i8 }, { i8, i8 }* %0, i32 0, i32 1
  %14 = load i8, i8* %13, align 1
  %15 = zext i1 %12 to i8
  %16 = insertvalue { i8, i8 } undef, i8 %15, 0
  %17 = insertvalue { i8, i8 } %16, i8 %14, 1
  ret { i8, i8 } %17
}

; std::thread::panicking
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN3std6thread9panicking17h925977b625684a73E() unnamed_addr #0 {
start:
; call std::panicking::panicking
  %0 = call zeroext i1 @_ZN3std9panicking9panicking17hc0748bb032f7c8e3E()
  br label %bb1

bb1:                                              ; preds = %start
  ret i1 %0
}

; std::panicking::panic_count::count_is_zero
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN3std9panicking11panic_count13count_is_zero17hb7b290e802bdd59fE() unnamed_addr #0 {
start:
  %_5 = alloca i8, align 1
  %0 = alloca i8, align 1
  store i8 0, i8* %_5, align 1
  %1 = load i8, i8* %_5, align 1, !range !4, !noundef !1
; call core::sync::atomic::AtomicUsize::load
  %_2 = call i64 @_ZN4core4sync6atomic11AtomicUsize4load17hf9dd3e3fb5bd7abeE(%"core::sync::atomic::AtomicUsize"* align 8 @_ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17hc7328e6d1006a770E, i8 %1)
  br label %bb1

bb1:                                              ; preds = %start
  %_1 = and i64 %_2, 9223372036854775807
  %2 = icmp eq i64 %_1, 0
  br i1 %2, label %bb2, label %bb3

bb2:                                              ; preds = %bb1
  store i8 1, i8* %0, align 1
  br label %bb4

bb3:                                              ; preds = %bb1
; call std::panicking::panic_count::is_zero_slow_path
  %3 = call zeroext i1 @_ZN3std9panicking11panic_count17is_zero_slow_path17h2f34c32169652de1E()
  %4 = zext i1 %3 to i8
  store i8 %4, i8* %0, align 1
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %5 = load i8, i8* %0, align 1, !range !3, !noundef !1
  %6 = trunc i8 %5 to i1
  ret i1 %6
}

; std::panicking::panicking
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN3std9panicking9panicking17hc0748bb032f7c8e3E() unnamed_addr #0 {
start:
; call std::panicking::panic_count::count_is_zero
  %_1 = call zeroext i1 @_ZN3std9panicking11panic_count13count_is_zero17hb7b290e802bdd59fE()
  br label %bb1

bb1:                                              ; preds = %start
  %0 = xor i1 %_1, true
  ret i1 %0
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %0, [0 x { [0 x i8]*, i64 }]* align 8 %pieces.0, i64 %pieces.1, [0 x { i8*, i64* }]* align 8 %args.0, i64 %args.1) unnamed_addr #0 {
start:
  %_24 = alloca { i64*, i64 }, align 8
  %_16 = alloca %"core::fmt::Arguments", align 8
  %_3 = alloca i8, align 1
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_12 = add i64 %args.1, 1
  %_9 = icmp ugt i64 %pieces.1, %_12
  %1 = zext i1 %_9 to i8
  store i8 %1, i8* %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, i8* %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, i8* %_3, align 1, !range !3, !noundef !1
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  %4 = bitcast { i64*, i64 }* %_24 to {}**
  store {}* null, {}** %4, align 8
  %5 = bitcast %"core::fmt::Arguments"* %0 to { [0 x { [0 x i8]*, i64 }]*, i64 }*
  %6 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %5, i32 0, i32 0
  store [0 x { [0 x i8]*, i64 }]* %pieces.0, [0 x { [0 x i8]*, i64 }]** %6, align 8
  %7 = getelementptr inbounds { [0 x { [0 x i8]*, i64 }]*, i64 }, { [0 x { [0 x i8]*, i64 }]*, i64 }* %5, i32 0, i32 1
  store i64 %pieces.1, i64* %7, align 8
  %8 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 1
  %9 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_24, i32 0, i32 0
  %10 = load i64*, i64** %9, align 8, !align !2
  %11 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_24, i32 0, i32 1
  %12 = load i64, i64* %11, align 8
  %13 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %8, i32 0, i32 0
  store i64* %10, i64** %13, align 8
  %14 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %8, i32 0, i32 1
  store i64 %12, i64* %14, align 8
  %15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 2
  %16 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %15, i32 0, i32 0
  store [0 x { i8*, i64* }]* %args.0, [0 x { i8*, i64* }]** %16, align 8
  %17 = getelementptr inbounds { [0 x { i8*, i64* }]*, i64 }, { [0 x { i8*, i64* }]*, i64 }* %15, i32 0, i32 1
  store i64 %args.1, i64* %17, align 8
  ret void

bb4:                                              ; preds = %bb3
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_16, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc42 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb5

bb5:                                              ; preds = %bb4
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_16, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc66 to %"core::panic::location::Location"*)) #6
  unreachable
}

; core::ptr::drop_in_place<std::sync::mutex::MutexGuard<i32>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr60drop_in_place$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$17heb4e312572da021cE"({ i64*, i8 }* %_1) unnamed_addr #1 {
start:
; call <std::sync::mutex::MutexGuard<T> as core::ops::drop::Drop>::drop
  call void @"_ZN79_$LT$std..sync..mutex..MutexGuard$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb08f088864c65f85E"({ i64*, i8 }* align 8 %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>
; Function Attrs: uwtable
define void @"_ZN4core3ptr98drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$$GT$17h732bdcd435bb494aE"({ i64*, i8 }* %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<std::sync::mutex::MutexGuard<i32>>
  call void @"_ZN4core3ptr60drop_in_place$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$17heb4e312572da021cE"({ i64*, i8 }* %_1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::sync::atomic::AtomicBool::load
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN4core4sync6atomic10AtomicBool4load17h5d5317445a2ceb3aE(%"core::sync::atomic::AtomicBool"* align 1 %self, i8 %order) unnamed_addr #0 {
start:
  %_6 = bitcast %"core::sync::atomic::AtomicBool"* %self to i8*
  br label %bb1

bb1:                                              ; preds = %start
; call core::sync::atomic::atomic_load
  %_3 = call i8 @_ZN4core4sync6atomic11atomic_load17ha18470b111cb3efbE(i8* %_6, i8 %order)
  br label %bb2

bb2:                                              ; preds = %bb1
  %0 = icmp ne i8 %_3, 0
  ret i1 %0
}

; core::sync::atomic::AtomicBool::store
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core4sync6atomic10AtomicBool5store17ha82b9faeb3a2c2b8E(%"core::sync::atomic::AtomicBool"* align 1 %self, i1 zeroext %val, i8 %order) unnamed_addr #0 {
start:
  %_6 = bitcast %"core::sync::atomic::AtomicBool"* %self to i8*
  br label %bb1

bb1:                                              ; preds = %start
  %0 = icmp ule i1 %val, true
  call void @llvm.assume(i1 %0)
  %_7 = zext i1 %val to i8
; call core::sync::atomic::atomic_store
  call void @_ZN4core4sync6atomic12atomic_store17hb3811326635a697bE(i8* %_6, i8 %_7, i8 %order)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::sync::atomic::AtomicUsize::load
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core4sync6atomic11AtomicUsize4load17hf9dd3e3fb5bd7abeE(%"core::sync::atomic::AtomicUsize"* align 8 %self, i8 %order) unnamed_addr #0 {
start:
  %_5 = bitcast %"core::sync::atomic::AtomicUsize"* %self to i64*
  br label %bb1

bb1:                                              ; preds = %start
; call core::sync::atomic::atomic_load
  %0 = call i64 @_ZN4core4sync6atomic11atomic_load17h09595012babb283cE(i64* %_5, i8 %order)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i64 %0
}

; core::sync::atomic::atomic_load
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core4sync6atomic11atomic_load17h09595012babb283cE(i64* %dst, i8 %0) unnamed_addr #0 {
start:
  %_16 = alloca %"core::fmt::Arguments", align 8
  %_8 = alloca %"core::fmt::Arguments", align 8
  %1 = alloca i64, align 8
  %order = alloca i8, align 1
  store i8 %0, i8* %order, align 1
  %2 = load i8, i8* %order, align 1, !range !4, !noundef !1
  %_3 = zext i8 %2 to i64
  switch i64 %_3, label %bb2 [
    i64 0, label %bb5
    i64 1, label %bb9
    i64 2, label %bb3
    i64 3, label %bb1
    i64 4, label %bb7
  ]

bb2:                                              ; preds = %start
  unreachable

bb5:                                              ; preds = %start
  %3 = load atomic i64, i64* %dst monotonic, align 8
  store i64 %3, i64* %1, align 8
  br label %bb6

bb9:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_8, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc52 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb10

bb3:                                              ; preds = %start
  %4 = load atomic i64, i64* %dst acquire, align 8
  store i64 %4, i64* %1, align 8
  br label %bb4

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_16, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc47 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb11

bb7:                                              ; preds = %start
  %5 = load atomic i64, i64* %dst seq_cst, align 8
  store i64 %5, i64* %1, align 8
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb12

bb12:                                             ; preds = %bb6, %bb4, %bb8
  %6 = load i64, i64* %1, align 8
  ret i64 %6

bb11:                                             ; preds = %bb1
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_16, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc68 to %"core::panic::location::Location"*)) #6
  unreachable

bb4:                                              ; preds = %bb3
  br label %bb12

bb10:                                             ; preds = %bb9
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_8, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc70 to %"core::panic::location::Location"*)) #6
  unreachable

bb6:                                              ; preds = %bb5
  br label %bb12
}

; core::sync::atomic::atomic_load
; Function Attrs: inlinehint uwtable
define i8 @_ZN4core4sync6atomic11atomic_load17ha18470b111cb3efbE(i8* %dst, i8 %0) unnamed_addr #0 {
start:
  %_16 = alloca %"core::fmt::Arguments", align 8
  %_8 = alloca %"core::fmt::Arguments", align 8
  %1 = alloca i8, align 1
  %order = alloca i8, align 1
  store i8 %0, i8* %order, align 1
  %2 = load i8, i8* %order, align 1, !range !4, !noundef !1
  %_3 = zext i8 %2 to i64
  switch i64 %_3, label %bb2 [
    i64 0, label %bb5
    i64 1, label %bb9
    i64 2, label %bb3
    i64 3, label %bb1
    i64 4, label %bb7
  ]

bb2:                                              ; preds = %start
  unreachable

bb5:                                              ; preds = %start
  %3 = load atomic i8, i8* %dst monotonic, align 1
  store i8 %3, i8* %1, align 1
  br label %bb6

bb9:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_8, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc52 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb10

bb3:                                              ; preds = %start
  %4 = load atomic i8, i8* %dst acquire, align 1
  store i8 %4, i8* %1, align 1
  br label %bb4

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_16, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc47 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb11

bb7:                                              ; preds = %start
  %5 = load atomic i8, i8* %dst seq_cst, align 1
  store i8 %5, i8* %1, align 1
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb12

bb12:                                             ; preds = %bb6, %bb4, %bb8
  %6 = load i8, i8* %1, align 1
  ret i8 %6

bb11:                                             ; preds = %bb1
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_16, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc68 to %"core::panic::location::Location"*)) #6
  unreachable

bb4:                                              ; preds = %bb3
  br label %bb12

bb10:                                             ; preds = %bb9
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_8, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc70 to %"core::panic::location::Location"*)) #6
  unreachable

bb6:                                              ; preds = %bb5
  br label %bb12
}

; core::sync::atomic::atomic_store
; Function Attrs: inlinehint uwtable
define void @_ZN4core4sync6atomic12atomic_store17hb3811326635a697bE(i8* %dst, i8 %val, i8 %0) unnamed_addr #0 {
start:
  %_20 = alloca %"core::fmt::Arguments", align 8
  %_12 = alloca %"core::fmt::Arguments", align 8
  %order = alloca i8, align 1
  store i8 %0, i8* %order, align 1
  %1 = load i8, i8* %order, align 1, !range !4, !noundef !1
  %_4 = zext i8 %1 to i64
  switch i64 %_4, label %bb2 [
    i64 0, label %bb5
    i64 1, label %bb3
    i64 2, label %bb9
    i64 3, label %bb1
    i64 4, label %bb7
  ]

bb2:                                              ; preds = %start
  unreachable

bb5:                                              ; preds = %start
  store atomic i8 %val, i8* %dst monotonic, align 1
  br label %bb6

bb3:                                              ; preds = %start
  store atomic i8 %val, i8* %dst release, align 1
  br label %bb4

bb9:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_12, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc62 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb10

bb1:                                              ; preds = %start
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h75ef5670dbebf77cE(%"core::fmt::Arguments"* sret(%"core::fmt::Arguments") %_20, [0 x { [0 x i8]*, i64 }]* align 8 bitcast (<{ i8*, [8 x i8] }>* @alloc57 to [0 x { [0 x i8]*, i64 }]*), i64 1, [0 x { i8*, i64* }]* align 8 bitcast (<{}>* @alloc64 to [0 x { i8*, i64* }]*), i64 0)
  br label %bb11

bb7:                                              ; preds = %start
  store atomic i8 %val, i8* %dst seq_cst, align 1
  br label %bb8

bb8:                                              ; preds = %bb7
  br label %bb12

bb12:                                             ; preds = %bb6, %bb4, %bb8
  ret void

bb11:                                             ; preds = %bb1
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_20, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc72 to %"core::panic::location::Location"*)) #6
  unreachable

bb10:                                             ; preds = %bb9
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"* %_12, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc74 to %"core::panic::location::Location"*)) #6
  unreachable

bb4:                                              ; preds = %bb3
  br label %bb12

bb6:                                              ; preds = %bb5
  br label %bb12
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint uwtable
define { i64*, i8 } @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hfb9c5e1e01e4eb21E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %self, %"core::panic::location::Location"* align 8 %0) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %e = alloca { i64*, i8 }, align 8
  %1 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %self to i64*
  %_2 = load i64, i64* %1, align 8, !range !5, !noundef !1
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %self to %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok"*
  %3 = getelementptr inbounds %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok", %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Ok"* %2, i32 0, i32 1
  %4 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %3, i32 0, i32 0
  %t.0 = load i64*, i64** %4, align 8, !nonnull !1, !align !2, !noundef !1
  %5 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %3, i32 0, i32 1
  %6 = load i8, i8* %5, align 8, !range !3, !noundef !1
  %t.1 = trunc i8 %6 to i1
  %7 = zext i1 %t.1 to i8
  %8 = insertvalue { i64*, i8 } undef, i64* %t.0, 0
  %9 = insertvalue { i64*, i8 } %8, i8 %7, 1
  ret { i64*, i8 } %9

bb1:                                              ; preds = %start
  %10 = bitcast %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %self to %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err"*
  %11 = getelementptr inbounds %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err", %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>::Err"* %10, i32 0, i32 1
  %12 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %11, i32 0, i32 0
  %13 = load i64*, i64** %12, align 8, !nonnull !1, !align !2, !noundef !1
  %14 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %11, i32 0, i32 1
  %15 = load i8, i8* %14, align 8, !range !3, !noundef !1
  %16 = trunc i8 %15 to i1
  %17 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %e, i32 0, i32 0
  store i64* %13, i64** %17, align 8
  %18 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %e, i32 0, i32 1
  %19 = zext i1 %16 to i8
  store i8 %19, i8* %18, align 8
  %_7.0 = bitcast { i64*, i8 }* %e to {}*
; invoke core::result::unwrap_failed
  invoke void @_ZN4core6result13unwrap_failed17h7f042806f6489227E([0 x i8]* align 1 bitcast (<{ [43 x i8] }>* @alloc75 to [0 x i8]*), i64 43, {}* align 1 %_7.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to [3 x i64]*), %"core::panic::location::Location"* align 8 %0) #6
          to label %unreachable unwind label %funclet_bb4

bb4:                                              ; preds = %funclet_bb4
; call core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>
  call void @"_ZN4core3ptr98drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$$GT$17h732bdcd435bb494aE"({ i64*, i8 }* %e) #7 [ "funclet"(token %cleanuppad) ]
  br label %bb5

funclet_bb4:                                      ; preds = %bb1
  %cleanuppad = cleanuppad within none []
  br label %bb4

unreachable:                                      ; preds = %bb1
  unreachable

bb5:                                              ; preds = %bb4
  cleanupret from %cleanuppad unwind to caller
}

; <std::sync::poison::PoisonError<T> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define zeroext i1 @"_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h11d1fe98498ee2fdE"({ i64*, i8 }* align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #1 {
start:
  %_4 = alloca %"core::fmt::builders::DebugStruct", align 8
; call core::fmt::Formatter::debug_struct
  call void @_ZN4core3fmt9Formatter12debug_struct17h4a5289a261857a4fE(%"core::fmt::builders::DebugStruct"* sret(%"core::fmt::builders::DebugStruct") %_4, %"core::fmt::Formatter"* align 8 %f, [0 x i8]* align 1 bitcast (<{ [11 x i8] }>* @alloc79 to [0 x i8]*), i64 11)
  br label %bb1

bb1:                                              ; preds = %start
; call core::fmt::builders::DebugStruct::finish_non_exhaustive
  %0 = call zeroext i1 @_ZN4core3fmt8builders11DebugStruct21finish_non_exhaustive17h54c579dbe67dbd1bE(%"core::fmt::builders::DebugStruct"* align 8 %_4)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret i1 %0
}

; <std::sync::mutex::MutexGuard<T> as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint uwtable
define void @"_ZN79_$LT$std..sync..mutex..MutexGuard$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb08f088864c65f85E"({ i64*, i8 }* align 8 %self) unnamed_addr #0 {
start:
  %0 = bitcast { i64*, i8 }* %self to %"std::sync::mutex::Mutex<i32>"**
  %_8 = load %"std::sync::mutex::Mutex<i32>"*, %"std::sync::mutex::Mutex<i32>"** %0, align 8, !nonnull !1, !align !2, !noundef !1
  %_3 = getelementptr inbounds %"std::sync::mutex::Mutex<i32>", %"std::sync::mutex::Mutex<i32>"* %_8, i32 0, i32 1
  %_5 = getelementptr inbounds { i64*, i8 }, { i64*, i8 }* %self, i32 0, i32 1
; call std::sync::poison::Flag::done
  call void @_ZN3std4sync6poison4Flag4done17h60ae6d453c9d1345E(%"std::sync::poison::Flag"* align 1 %_3, i8* align 1 %_5)
  br label %bb1

bb1:                                              ; preds = %start
  %1 = bitcast { i64*, i8 }* %self to %"std::sync::mutex::Mutex<i32>"**
  %_9 = load %"std::sync::mutex::Mutex<i32>"*, %"std::sync::mutex::Mutex<i32>"** %1, align 8, !nonnull !1, !align !2, !noundef !1
  %_7 = bitcast %"std::sync::mutex::Mutex<i32>"* %_9 to %"std::sys_common::mutex::MovableMutex"*
; call std::sys_common::mutex::MovableMutex::raw_unlock
  call void @_ZN3std10sys_common5mutex12MovableMutex10raw_unlock17hb890a42d59020666E(%"std::sys_common::mutex::MovableMutex"* align 8 %_7)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; <std::sync::mutex::MutexGuard<T> as core::ops::deref::Deref>::deref
; Function Attrs: uwtable
define align 4 i32* @"_ZN81_$LT$std..sync..mutex..MutexGuard$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h41dbb24be75d1f05E"({ i64*, i8 }* align 8 %self) unnamed_addr #1 {
start:
  %0 = bitcast { i64*, i8 }* %self to %"std::sync::mutex::Mutex<i32>"**
  %_4 = load %"std::sync::mutex::Mutex<i32>"*, %"std::sync::mutex::Mutex<i32>"** %0, align 8, !nonnull !1, !align !2, !noundef !1
  %_3 = getelementptr inbounds %"std::sync::mutex::Mutex<i32>", %"std::sync::mutex::Mutex<i32>"* %_4, i32 0, i32 3
  br label %bb1

bb1:                                              ; preds = %start
  ret i32* %_3
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17he098e2c9f54edf27E() unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_4 = alloca %"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>", align 8
  %_3 = alloca { i64*, i8 }, align 8
; call std::sync::mutex::Mutex<T>::lock
  call void @"_ZN3std4sync5mutex14Mutex$LT$T$GT$4lock17had30de60cf66c219E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* sret(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>") %_4, %"std::sync::mutex::Mutex<i32>"* align 8 bitcast (<{ [9 x i8], [3 x i8], [4 x i8] }>* @_ZN6probe15probe8MY_MUTEX17h19c69f431d5cf48fE to %"std::sync::mutex::Mutex<i32>"*))
  br label %bb1

bb1:                                              ; preds = %start
; call core::result::Result<T,E>::unwrap
  %0 = call { i64*, i8 } @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17hfb9c5e1e01e4eb21E"(%"core::result::Result<std::sync::mutex::MutexGuard<i32>, std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<i32>>>"* %_4, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc81 to %"core::panic::location::Location"*))
  store { i64*, i8 } %0, { i64*, i8 }* %_3, align 8
  br label %bb2

bb2:                                              ; preds = %bb1
; invoke <std::sync::mutex::MutexGuard<T> as core::ops::deref::Deref>::deref
  %_1 = invoke align 4 i32* @"_ZN81_$LT$std..sync..mutex..MutexGuard$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h41dbb24be75d1f05E"({ i64*, i8 }* align 8 %_3)
          to label %bb3 unwind label %funclet_bb5

bb5:                                              ; preds = %funclet_bb5
; call core::ptr::drop_in_place<std::sync::mutex::MutexGuard<i32>>
  call void @"_ZN4core3ptr60drop_in_place$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$17heb4e312572da021cE"({ i64*, i8 }* %_3) #7 [ "funclet"(token %cleanuppad) ]
  br label %bb6

funclet_bb5:                                      ; preds = %bb2
  %cleanuppad = cleanuppad within none []
  br label %bb5

bb3:                                              ; preds = %bb2
; call core::ptr::drop_in_place<std::sync::mutex::MutexGuard<i32>>
  call void @"_ZN4core3ptr60drop_in_place$LT$std..sync..mutex..MutexGuard$LT$i32$GT$$GT$17heb4e312572da021cE"({ i64*, i8 }* %_3)
  br label %bb4

bb6:                                              ; preds = %bb5
  cleanupret from %cleanuppad unwind to caller

bb4:                                              ; preds = %bb3
  ret void
}

; Function Attrs: uwtable
declare dllimport void @AcquireSRWLockExclusive(%"std::sys::windows::c::SRWLOCK"*) unnamed_addr #1

; Function Attrs: uwtable
declare dllimport void @ReleaseSRWLockExclusive(%"std::sys::windows::c::SRWLOCK"*) unnamed_addr #1

; std::panicking::panic_count::is_zero_slow_path
; Function Attrs: cold noinline uwtable
declare zeroext i1 @_ZN3std9panicking11panic_count17is_zero_slow_path17h2f34c32169652de1E() unnamed_addr #2

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17hcab3b32ffb889e5aE(%"core::fmt::Arguments"*, %"core::panic::location::Location"* align 8) unnamed_addr #3

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #4

declare i32 @__CxxFrameHandler3(...) unnamed_addr #5

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core6result13unwrap_failed17h7f042806f6489227E([0 x i8]* align 1, i64, {}* align 1, [3 x i64]* align 8, %"core::panic::location::Location"* align 8) unnamed_addr #3

; core::fmt::Formatter::debug_struct
; Function Attrs: uwtable
declare void @_ZN4core3fmt9Formatter12debug_struct17h4a5289a261857a4fE(%"core::fmt::builders::DebugStruct"* sret(%"core::fmt::builders::DebugStruct"), %"core::fmt::Formatter"* align 8, [0 x i8]* align 1, i64) unnamed_addr #1

; core::fmt::builders::DebugStruct::finish_non_exhaustive
; Function Attrs: uwtable
declare zeroext i1 @_ZN4core3fmt8builders11DebugStruct21finish_non_exhaustive17h54c579dbe67dbd1bE(%"core::fmt::builders::DebugStruct"* align 8) unnamed_addr #1

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { cold noinline uwtable "target-cpu"="x86-64" }
attributes #3 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #4 = { inaccessiblememonly nofree nosync nounwind willreturn }
attributes #5 = { "target-cpu"="x86-64" }
attributes #6 = { noreturn }
attributes #7 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
!2 = !{i64 8}
!3 = !{i8 0, i8 2}
!4 = !{i8 0, i8 5}
!5 = !{i64 0, i64 2}
