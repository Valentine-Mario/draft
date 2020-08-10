; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %b = alloca i64
  %a = alloca i64
  store i64 9, i64* %a
  %a1 = load i64, i64* %a
  %addtmp = add i64 %a1, 9
  store i64 12, i64* %b
  %a2 = load i64, i64* %a
  %b3 = load i64, i64* %b
  %addtmp4 = add i64 %a2, %b3
  ret i64 %addtmp4
}
