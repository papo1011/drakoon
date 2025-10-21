@.str.0 = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)

@C = global i32 8, align 4
  define i32 @add(i32 %a, i32 %b) {
  entry:
  %t0 = load i32, i32* @C, align 4
  %t1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t0)
  %t2 = add i32 %a, %b
  %t3 = load i32, i32* @C, align 4
  %t4 = add i32 %t2, %t3
  ret i32 %t4
  ret i32 0
  }

  define i8 @compute() {
  entry:
  %t5 = call i32 @add(i32 2, i32 40)
  %t6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t5)
  ret i8 0
  }

  define i32 @main() {
  entry:
  %t7 = call i8 @compute()
  ret i32 0
  }

