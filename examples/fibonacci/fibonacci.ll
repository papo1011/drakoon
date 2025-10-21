@.str.4 = private constant [8 x i8] c"Output:\00"
@.str.3 = private constant [4 x i8] c"%d\0A\00"
@.str.2 = private constant [7 x i8] c"Input:\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.0 = private constant [10 x i8] c"Fibonacci\00"
declare i32 @printf(i8*, ...)

  define i32 @fibonacci_iterative(i32 %n) {
  entry:
  %t0 = icmp eq i32 %n, 0
  br i1 %t0, label %if.then.0, label %if.end.0
if.then.0:
  ret i32 0
  br label %if.end.0
if.end.0:
  %t1 = icmp eq i32 %n, 1
  br i1 %t1, label %if.then.1, label %if.end.1
if.then.1:
  ret i32 1
  br label %if.end.1
if.end.1:
  %a = alloca i32, align 4
  store i32 0, i32* %a, align 4
  %b = alloca i32, align 4
  store i32 1, i32* %b, align 4
  %i = alloca i32, align 4
  store i32 2, i32* %i, align 4
  br label %for.cond.2
for.cond.2:
  %t2 = load i32, i32* %i, align 4
  %t3 = icmp sle i32 %t2, %n
  br i1 %t3, label %for.body.2, label %for.end.2
for.body.2:
  %t4 = load i32, i32* %a, align 4
  %t5 = load i32, i32* %b, align 4
  %t6 = add i32 %t4, %t5
  %next = alloca i32, align 4
  store i32 %t6, i32* %next, align 4
  %t7 = load i32, i32* %b, align 4
  store i32 %t7, i32* %a, align 4
  %t8 = load i32, i32* %next, align 4
  store i32 %t8, i32* %b, align 4
  br label %for.step.2
for.step.2:
  %t9 = load i32, i32* %i, align 4
  %t10 = add i32 %t9, 1
  store i32 %t10, i32* %i, align 4
  br label %for.cond.2
for.end.2:
  %t11 = load i32, i32* %b, align 4
  ret i32 %t11
  ret i32 0
  }

  define i32 @main() {
  entry:
  %input = alloca i32, align 4
  store i32 10, i32* %input, align 4
  %t12 = load i32, i32* %input, align 4
  %t13 = call i32 @fibonacci_iterative(i32 %t12)
  %output = alloca i32, align 4
  store i32 %t13, i32* %output, align 4
  %t14 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.str.0, i32 0, i32 0))
  %t15 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([7 x i8], [7 x i8]* @.str.2, i32 0, i32 0))
  %t16 = load i32, i32* %input, align 4
  %t17 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i32 0, i32 0), i32 %t16)
  %t18 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str.4, i32 0, i32 0))
  %t19 = load i32, i32* %output, align 4
  %t20 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i32 0, i32 0), i32 %t19)
  ret i32 0
  }

