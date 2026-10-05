define i32 @main() {
entry:
    %x_0 = alloca i32
    store i32 20, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    comp_srem_0 = srem i32 %x_0_value_0, 2
    comp_sgt_1 = icmp sgt i32 comp_srem_0, 1
    br i1 %comp_sgt_1, label %then_0, label %else
    ret i32 69
}
