; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %b = alloca i64
  %a = alloca i64
  store i64 14, i64* %a
  store i64 107, i64* %b
  %b1 = load i64, i64* %b
  %a2 = load i64, i64* %a
  %subtmp = sub i64 %b1, %a2
  ret i64 %subtmp
}
