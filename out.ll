; ModuleID = 'example_module'
source_filename = "example_module"

define i64 @main() {
entry:
  %val = alloca i64
  %a = alloca i64
  store i64 9, i64* %a
  store i64 8, i64* %val
  %val1 = load i64, i64* %val
  %subtmp = sub i64 %val1, 8
  ret i64 %subtmp
}
