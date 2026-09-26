define i32 @main() {
entry:
    %x_0 = alloca i32
    %x_0_add_0 = add i32 5, 5
    store i32 %x_0_add_0, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    ret i32 %x_0_value_0
}
