define i32 @main() {
entry:
    %x_0 = alloca i1
    store i1 true, ptr %x_0
    %x_0_value_0 = load i1, ptr %x_0
    store i1 false, ptr %x_0
    %x_0_value_1 = load i1, ptr %x_0
    ret i32 1
}
