; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %a = alloca i64
  store i64 99, i64* %a
  %a1 = load i64, i64* %a
  %addtmp = add i64 %a1, 40
  ret i64 %addtmp
}
