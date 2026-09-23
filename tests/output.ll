define i32 @main() {
entry:
    %x_0 = alloca i32
    %x_0_mul_0 = mul i32 6, 3
    %x_0_add_1 = add i32 1, %x_0_mul_0
    %x_0_add_2 = add i32 %x_0_add_1, 5
    %x_0_mul_3 = mul i32 2, 3
    %x_0_sub_4 = sub i32 %x_0_add_2, %x_0_mul_3
    store i32 %x_0_sub_4, ptr %x_0
    %x_value_0 = load i32, ptr %x_0

    ret i32 %x_value_0
}
