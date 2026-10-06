define i32 @main() {
entry:
    %x_0 = alloca i32
    store i32 30, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    %comp_sgt_0 = icmp sgt i32 %x_0_value_0, 5
    br i1 %comp_sgt_0, label %then_0, label %after_0
then_0:
    store i32 69, ptr %x_0
    br label %after_0
after_0:
    %x_0_value_1 = load i32, ptr %x_0
    ret i32 %x_0_value_1
}
