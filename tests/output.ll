; ModuleID = 'source.tc'
source_filename = "source.tc"

define i32 @main() {
entry:
  %x = alloca i32, align 4
  store i32 5, ptr %x, align 4
  br label %if_cond

if_merge:                                         ; preds = %if_body3, %if_body1
  %x7 = load i32, ptr %x, align 4
  ret i32 %x7

if_body:                                          ; preds = %if_cond
  ret i32 0

if_cond:                                          ; preds = %entry
  %x4 = load i32, ptr %x, align 4
  %remtemp = srem i32 %x4, 2
  %cmptmp = icmp eq i32 %remtemp, 0
  br i1 %cmptmp, label %if_body, label %if_cond2

if_body1:                                         ; preds = %if_cond2
  store i32 67, ptr %x, align 4
  br label %if_merge

if_cond2:                                         ; preds = %if_cond
  %x5 = load i32, ptr %x, align 4
  %cmptmp6 = icmp eq i32 %x5, 5
  br i1 %cmptmp6, label %if_body1, label %if_body3

if_body3:                                         ; preds = %if_cond2
  store i32 69, ptr %x, align 4
  br label %if_merge
}
