; ModuleID = 'source.tc'
source_filename = "source.tc"

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 50, ptr %x, align 4
  %x1 = load i32, ptr %x, align 4
  ret i32 %x1
}
