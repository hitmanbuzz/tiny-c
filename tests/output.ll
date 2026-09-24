define i32 @main() {
entry:
    %x_0 = alloca i32
    store i32 1, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    %x_0_add_0 = add i32 %x_0_value_0, 2
    store i32 %x_0_add_0, ptr %x_0
    %x_0_value_1 = load i32, ptr %x_0
    %x_0_mul_1 = mul i32 %x_0_value_1, 2
    %x_0_add_2 = add i32 %x_0_mul_1, 5
    store i32 %x_0_add_2, ptr %x_0
    %x_0_value_2 = load i32, ptr %x_0
    ret i32 %x_0_value_2
}
