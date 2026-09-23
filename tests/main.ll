declare i32 @printf(i8*, ...)

@fmt = private unnamed_addr constant [7 x i8] c"%d %d\0A\00"

define i32 @main() {
entry:
  %x = alloca i32
  store i32 69, i32* %x
  %value = load i32, i32* %x
  %fmt_ptr = getelementptr inbounds [4 x i8], [4 x i8]* @fmt, i64 0, i64 0
  %call = call i32 (i8*, ...) @printf(i8* %fmt_ptr, i32 %value, i32 67)
  ret i32 0
}
