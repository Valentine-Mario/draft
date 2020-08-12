; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  %b = alloca i64
  store i64 0, i64* %a
  %a1 = load i64, i64* %a
  %addtmp = add i64 %a1, 1
  %a2 = load i64, i64* %a
  %subtmp = sub i64 %a2, 1
  %a3 = load i64, i64* %a
  store i64 12, i64* %b
  %a4 = load i64, i64* %a
  %is_nonzero = icmp ne i64 %a4, 0
  br i1 %is_nonzero, label %entry5, label %entry6

entry5:                                           ; preds = %entry
  store i64 10, i64* %b
  br label %entry7

entry6:                                           ; preds = %entry
  store i64 12, i64* %b
  br label %entry7

entry7:                                           ; preds = %entry6, %entry5
  %iftmp = phi i64 [ 10, %entry5 ], [ 12, %entry6 ]
  %b8 = load i64, i64* %b
  ret i64 %b8
}
