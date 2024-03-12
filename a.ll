; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@array = global [30000 x i8] zeroinitializer

declare i8 @getchar()

declare i32 @putchar(i8)

define i32 @main() {
entry_block:
  %pointer = alloca ptr, align 8
  store ptr @array, ptr %pointer, align 8
  %pointer1 = load i8, ptr %pointer, align 1
  %new_value = add i8 %pointer1, 1
  store i8 %new_value, ptr %pointer, align 1
  %pointer2 = load i8, ptr %pointer, align 1
  %new_value3 = add i8 %pointer2, 1
  store i8 %new_value3, ptr %pointer, align 1
  %pointer4 = load i8, ptr %pointer, align 1
  %new_value5 = add i8 %pointer4, 1
  store i8 %new_value5, ptr %pointer, align 1
  %pointer6 = load i8, ptr %pointer, align 1
  %new_value7 = add i8 %pointer6, 1
  store i8 %new_value7, ptr %pointer, align 1
  %pointer8 = load i8, ptr %pointer, align 1
  %new_value9 = add i8 %pointer8, 1
  store i8 %new_value9, ptr %pointer, align 1
  %pointer10 = load i8, ptr %pointer, align 1
  %new_value11 = add i8 %pointer10, 1
  store i8 %new_value11, ptr %pointer, align 1
  %pointer12 = load i8, ptr %pointer, align 1
  %new_value13 = add i8 %pointer12, 1
  store i8 %new_value13, ptr %pointer, align 1
  %pointer14 = load i8, ptr %pointer, align 1
  %new_value15 = add i8 %pointer14, 1
  store i8 %new_value15, ptr %pointer, align 1
  %pointer16 = load i8, ptr %pointer, align 1
  %new_value17 = add i8 %pointer16, 1
  store i8 %new_value17, ptr %pointer, align 1
  %pointer18 = load i8, ptr %pointer, align 1
  %new_value19 = add i8 %pointer18, 1
  store i8 %new_value19, ptr %pointer, align 1
  %pointer20 = load i8, ptr %pointer, align 1
  %new_value21 = add i8 %pointer20, 1
  store i8 %new_value21, ptr %pointer, align 1
  %pointer22 = load i8, ptr %pointer, align 1
  %new_value23 = add i8 %pointer22, 1
  store i8 %new_value23, ptr %pointer, align 1
  %pointer24 = load i8, ptr %pointer, align 1
  %new_value25 = add i8 %pointer24, 1
  store i8 %new_value25, ptr %pointer, align 1
  %pointer26 = load i8, ptr %pointer, align 1
  %new_value27 = add i8 %pointer26, 1
  store i8 %new_value27, ptr %pointer, align 1
  %pointer28 = load i8, ptr %pointer, align 1
  %new_value29 = add i8 %pointer28, 1
  store i8 %new_value29, ptr %pointer, align 1
  %pointer30 = load i8, ptr %pointer, align 1
  %new_value31 = add i8 %pointer30, 1
  store i8 %new_value31, ptr %pointer, align 1
  %pointer32 = load i8, ptr %pointer, align 1
  %new_value33 = add i8 %pointer32, 1
  store i8 %new_value33, ptr %pointer, align 1
  %pointer34 = load i8, ptr %pointer, align 1
  %new_value35 = add i8 %pointer34, 1
  store i8 %new_value35, ptr %pointer, align 1
  %pointer36 = load i8, ptr %pointer, align 1
  %new_value37 = add i8 %pointer36, 1
  store i8 %new_value37, ptr %pointer, align 1
  %pointer38 = load i8, ptr %pointer, align 1
  %new_value39 = add i8 %pointer38, 1
  store i8 %new_value39, ptr %pointer, align 1
  %pointer40 = load i8, ptr %pointer, align 1
  %new_value41 = add i8 %pointer40, 1
  store i8 %new_value41, ptr %pointer, align 1
  %pointer42 = load i8, ptr %pointer, align 1
  %new_value43 = add i8 %pointer42, 1
  store i8 %new_value43, ptr %pointer, align 1
  %pointer44 = load i8, ptr %pointer, align 1
  %new_value45 = add i8 %pointer44, 1
  store i8 %new_value45, ptr %pointer, align 1
  %pointer46 = load i8, ptr %pointer, align 1
  %new_value47 = add i8 %pointer46, 1
  store i8 %new_value47, ptr %pointer, align 1
  %pointer48 = load i8, ptr %pointer, align 1
  %new_value49 = add i8 %pointer48, 1
  store i8 %new_value49, ptr %pointer, align 1
  %pointer50 = load i8, ptr %pointer, align 1
  %new_value51 = add i8 %pointer50, 1
  store i8 %new_value51, ptr %pointer, align 1
  %pointer52 = load i8, ptr %pointer, align 1
  %new_value53 = add i8 %pointer52, 1
  store i8 %new_value53, ptr %pointer, align 1
  %pointer54 = load i8, ptr %pointer, align 1
  %new_value55 = add i8 %pointer54, 1
  store i8 %new_value55, ptr %pointer, align 1
  %pointer56 = load i8, ptr %pointer, align 1
  %new_value57 = add i8 %pointer56, 1
  store i8 %new_value57, ptr %pointer, align 1
  %pointer58 = load i8, ptr %pointer, align 1
  %new_value59 = add i8 %pointer58, 1
  store i8 %new_value59, ptr %pointer, align 1
  %pointer60 = load i8, ptr %pointer, align 1
  %new_value61 = add i8 %pointer60, 1
  store i8 %new_value61, ptr %pointer, align 1
  %pointer62 = load i8, ptr %pointer, align 1
  %new_value63 = add i8 %pointer62, 1
  store i8 %new_value63, ptr %pointer, align 1
  %pointer64 = load i8, ptr %pointer, align 1
  %new_value65 = add i8 %pointer64, 1
  store i8 %new_value65, ptr %pointer, align 1
  %value = load i8, ptr %pointer, align 1
  %call_putchar = call i32 @putchar(i8 %value)
  %pointer66 = load ptr, ptr %pointer, align 8
  %new_pointer = getelementptr inbounds ptr, ptr %pointer66, i32 1
  store ptr %new_pointer, ptr %pointer66, align 8
  %pointer67 = load ptr, ptr %pointer, align 8
  %new_pointer68 = getelementptr inbounds i8, ptr %pointer67, i64 -1
  store ptr %new_pointer68, ptr %pointer67, align 8
  %value69 = load i8, ptr %pointer, align 1
  %call_putchar70 = call i32 @putchar(i8 %value69)
  ret i32 0
}
