@.str.11 = private constant [18 x i8] c"nested_loops() ->\00"
@.str.10 = private constant [22 x i8] c"zero_iterations(1) ->\00"
@.str.9 = private constant [20 x i8] c"for_only_cond(4) ->\00"
@.str.8 = private constant [22 x i8] c"first_over_three() ->\00"
@.str.7 = private constant [14 x i8] c"sum_for(5) ->\00"
@.str.6 = private constant [18 x i8] c"while_count(3) ->\00"
@.str.5 = private constant [3 x i8] c"j:\00"
@.str.4 = private constant [3 x i8] c"i:\00"
@.str.3 = private constant [28 x i8] c"this should not print (for)\00"
@.str.2 = private constant [4 x i8] c"%s\0A\00"
@.str.1 = private constant [30 x i8] c"this should not print (while)\00"
@.str.0 = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)

  define i32 @while_count(i32 %n) {
  entry:
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %while.cond.0
while.cond.0:
  %t0 = load i32, i32* %i, align 4
  %t1 = icmp slt i32 %t0, %n
  br i1 %t1, label %while.body.0, label %while.end.0
while.body.0:
  %t2 = load i32, i32* %i, align 4
  %t3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t2)
  %t4 = load i32, i32* %i, align 4
  %t5 = add i32 %t4, 1
  store i32 %t5, i32* %i, align 4
  br label %while.cond.0
while.end.0:
  %t6 = load i32, i32* %i, align 4
  ret i32 %t6
  ret i32 0
  }

  define i32 @sum_for(i32 %n) {
  entry:
  %s = alloca i32, align 4
  store i32 0, i32* %s, align 4
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %for.cond.1
for.cond.1:
  %t7 = load i32, i32* %i, align 4
  %t8 = icmp slt i32 %t7, %n
  br i1 %t8, label %for.body.1, label %for.end.1
for.body.1:
  %t9 = load i32, i32* %s, align 4
  %t10 = load i32, i32* %i, align 4
  %t11 = add i32 %t9, %t10
  store i32 %t11, i32* %s, align 4
  br label %for.step.1
for.step.1:
  %t12 = load i32, i32* %i, align 4
  %t13 = add i32 %t12, 1
  store i32 %t13, i32* %i, align 4
  br label %for.cond.1
for.end.1:
  %t14 = load i32, i32* %s, align 4
  ret i32 %t14
  ret i32 0
  }

  define i32 @first_over_three() {
  entry:
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %for.cond.2
for.cond.2:
  br i1 true, label %for.body.2, label %for.end.2
for.body.2:
  %t15 = load i32, i32* %i, align 4
  %t16 = icmp sgt i32 %t15, 3
  br i1 %t16, label %if.then.3, label %if.end.3
if.then.3:
  %t17 = load i32, i32* %i, align 4
  ret i32 %t17
  br label %if.end.3
if.end.3:
  br label %for.step.2
for.step.2:
  %t18 = load i32, i32* %i, align 4
  %t19 = add i32 %t18, 1
  store i32 %t19, i32* %i, align 4
  br label %for.cond.2
for.end.2:
  ret i32 0
  }

  define i32 @for_only_cond(i32 %limit) {
  entry:
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %for.cond.4
for.cond.4:
  %t20 = load i32, i32* %i, align 4
  %t21 = icmp slt i32 %t20, %limit
  br i1 %t21, label %for.body.4, label %for.end.4
for.body.4:
  %t22 = load i32, i32* %i, align 4
  %t23 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t22)
  %t24 = load i32, i32* %i, align 4
  %t25 = add i32 %t24, 1
  store i32 %t25, i32* %i, align 4
  br label %for.step.4
for.step.4:
  br label %for.cond.4
for.end.4:
  %t26 = load i32, i32* %i, align 4
  ret i32 %t26
  ret i32 0
  }

  define i8 @zero_iterations(i32 %n) {
  entry:
  %i = alloca i32, align 4
  store i32 %n, i32* %i, align 4
  br label %while.cond.5
while.cond.5:
  %t27 = load i32, i32* %i, align 4
  %t28 = icmp slt i32 %t27, 0
  br i1 %t28, label %while.body.5, label %while.end.5
while.body.5:
  %t29 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([30 x i8], [30 x i8]* @.str.1, i32 0, i32 0))
  br label %while.cond.5
while.end.5:
  %j = alloca i32, align 4
  store i32 0, i32* %j, align 4
  br label %for.cond.6
for.cond.6:
  %t30 = load i32, i32* %j, align 4
  %t31 = icmp slt i32 %t30, 0
  br i1 %t31, label %for.body.6, label %for.end.6
for.body.6:
  %t32 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([28 x i8], [28 x i8]* @.str.3, i32 0, i32 0))
  br label %for.step.6
for.step.6:
  %t33 = load i32, i32* %j, align 4
  %t34 = add i32 %t33, 1
  store i32 %t34, i32* %j, align 4
  br label %for.cond.6
for.end.6:
  ret i8 0
  }

  define i8 @nested_loops() {
  entry:
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %for.cond.7
for.cond.7:
  %t35 = load i32, i32* %i, align 4
  %t36 = icmp slt i32 %t35, 2
  br i1 %t36, label %for.body.7, label %for.end.7
for.body.7:
  %j = alloca i32, align 4
  store i32 0, i32* %j, align 4
  br label %while.cond.8
while.cond.8:
  %t37 = load i32, i32* %j, align 4
  %t38 = icmp slt i32 %t37, 2
  br i1 %t38, label %while.body.8, label %while.end.8
while.body.8:
  %t39 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.str.4, i32 0, i32 0))
  %t40 = load i32, i32* %i, align 4
  %t41 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t40)
  %t42 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.str.5, i32 0, i32 0))
  %t43 = load i32, i32* %j, align 4
  %t44 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t43)
  %t45 = load i32, i32* %j, align 4
  %t46 = add i32 %t45, 1
  store i32 %t46, i32* %j, align 4
  br label %while.cond.8
while.end.8:
  br label %for.step.7
for.step.7:
  %t47 = load i32, i32* %i, align 4
  %t48 = add i32 %t47, 1
  store i32 %t48, i32* %i, align 4
  br label %for.cond.7
for.end.7:
  ret i8 0
  }

  define i32 @main() {
  entry:
  %t49 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.str.6, i32 0, i32 0))
  %t50 = call i32 @while_count(i32 3)
  %t51 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t50)
  %t52 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.7, i32 0, i32 0))
  %t53 = call i32 @sum_for(i32 5)
  %t54 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t53)
  %t55 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([22 x i8], [22 x i8]* @.str.8, i32 0, i32 0))
  %t56 = call i32 @first_over_three()
  %t57 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t56)
  %t58 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([20 x i8], [20 x i8]* @.str.9, i32 0, i32 0))
  %t59 = call i32 @for_only_cond(i32 4)
  %t60 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0), i32 %t59)
  %t61 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([22 x i8], [22 x i8]* @.str.10, i32 0, i32 0))
  %t62 = call i8 @zero_iterations(i32 1)
  %t63 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i32 0, i32 0), i8* getelementptr inbounds ([18 x i8], [18 x i8]* @.str.11, i32 0, i32 0))
  %t64 = call i8 @nested_loops()
  ret i32 0
  }

