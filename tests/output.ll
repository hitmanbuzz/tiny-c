define i32 @main() {
entry:
    %x_0 = alloca i32
    %x_0_mul_0 = mul i32 2, 3
    %x_0_add_1 = add i32 1, %x_0_mul_0
    store i32 %x_0_add_1, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    ret i32 %x_0_value_0
}
