define i32 @main() {
entry:
    %x_0 = alloca i32
    %y_1 = alloca i32
    store i32 10, ptr %y_1
    %y_1_value_0 = load i32, ptr %y_1
    store i32 67, ptr %x_0
    %x_0_value_0 = load i32, ptr %x_0
    store i32 69, ptr %x_0
    %x_0_value_1 = load i32, ptr %x_0
    store i32 30, ptr %y_1
    %y_1_value_1 = load i32, ptr %y_1
    store i32 %y_1_value_1, ptr %x_0
    %x_0_value_2 = load i32, ptr %x_0
    ret i32 %x_0_value_2
}
