@.str.4 = private constant [14 x i8] c"100 not found\00"
@.str.3 = private constant [10 x i8] c"Found 100\00"
@.str.2 = private constant [13 x i8] c"55 not found\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.0 = private constant [9 x i8] c"Found 55\00"
declare i32 @printf(i8*, ...)

  define i1 @binary_search(i32* %arr, i32 %len, i32 %target) {
  entry:
  %ret = alloca i1, align 1
  store i1 true, i1* %ret, align 1
  %low = alloca i32, align 4
  store i32 0, i32* %low, align 4
  %t0 = sub i32 %len, 1
  %high = alloca i32, align 4
  store i32 %t0, i32* %high, align 4
  br label %for.cond.0
for.cond.0:
  %t1 = load i32, i32* %low, align 4
  %t2 = load i32, i32* %high, align 4
  %t3 = icmp sle i32 %t1, %t2
  br i1 %t3, label %for.body.0, label %for.end.0
for.body.0:
  %t4 = load i32, i32* %low, align 4
  %t5 = load i32, i32* %high, align 4
  %t6 = add i32 %t4, %t5
  %t7 = sdiv i32 %t6, 2
  %median = alloca i32, align 4
  store i32 %t7, i32* %median, align 4
  %t8 = load i32, i32* %median, align 4
  %t9 = getelementptr inbounds i32, i32* %arr, i32 %t8
  %t10 = load i32, i32* %t9, align 4
  %t11 = icmp slt i32 %t10, %target
  br i1 %t11, label %if.then.1, label %if.else.1
if.then.1:
  %t12 = load i32, i32* %median, align 4
  %t13 = add i32 %t12, 1
  store i32 %t13, i32* %low, align 4
  br label %if.end.1
if.else.1:
  %t14 = load i32, i32* %median, align 4
  %t15 = sub i32 %t14, 1
  store i32 %t15, i32* %high, align 4
  br label %if.end.1
if.end.1:
  br label %for.step.0
for.step.0:
  br label %for.cond.0
for.end.0:
  %t16 = load i32, i32* %low, align 4
  %t17 = icmp eq i32 %t16, %len
  br i1 %t17, label %if.then.2, label %if.end.2
if.then.2:
  store i1 false, i1* %ret, align 1
  br label %if.end.2
if.end.2:
  %t18 = load i32, i32* %low, align 4
  %t19 = getelementptr inbounds i32, i32* %arr, i32 %t18
  %t20 = load i32, i32* %t19, align 4
  %t21 = icmp ne i32 %t20, %target
  br i1 %t21, label %if.then.3, label %if.end.3
if.then.3:
  store i1 false, i1* %ret, align 1
  br label %if.end.3
if.end.3:
  %t22 = load i1, i1* %ret, align 1
  ret i1 %t22
  ret i1 0
  }

  define i32 @main() {
  entry:
  %sorted = alloca [7 x i32], align 4
  %t23 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 0
  store i32 8, i32* %t23, align 4
  %t24 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 1
  store i32 22, i32* %t24, align 4
  %t25 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 2
  store i32 37, i32* %t25, align 4
  %t26 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 3
  store i32 55, i32* %t26, align 4
  %t27 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 4
  store i32 56, i32* %t27, align 4
  %t28 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 5
  store i32 102, i32* %t28, align 4
  %t29 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 6
  store i32 115, i32* %t29, align 4
  %t30 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 0
  %t31 = call i1 @binary_search(i32* %t30, i32 7, i32 55)
  br i1 %t31, label %if.then.4, label %if.else.4
if.then.4:
  %t32 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([9 x i8], [9 x i8]* @.str.0, i32 0, i32 0))
  br label %if.end.4
if.else.4:
  %t33 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str.2, i32 0, i32 0))
  br label %if.end.4
if.end.4:
  %t34 = getelementptr inbounds [7 x i32], [7 x i32]* %sorted, i32 0, i32 0
  %t35 = call i1 @binary_search(i32* %t34, i32 7, i32 100)
  br i1 %t35, label %if.then.5, label %if.else.5
if.then.5:
  %t36 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([10 x i8], [10 x i8]* @.str.3, i32 0, i32 0))
  br label %if.end.5
if.else.5:
  %t37 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.4, i32 0, i32 0))
  br label %if.end.5
if.end.5:
  ret i32 0
  }

