@.str.3 = private constant [4 x i8] c"%f\0A\00"
@.str.2 = private constant [8 x i8] c"Result:\00"
@.str.1 = private constant [4 x i8] c"%s\0A\00"
@.str.0 = private constant [4 x i8] c"ERR\00"
declare i32 @printf(i8*, ...)

  define double @division(double %a, double %b) {
  entry:
  %t0 = fcmp oeq double %b, 0.0
  br i1 %t0, label %if.then.0, label %if.else.0
if.then.0:
  %t1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.0, i32 0, i32 0))
  ret double 0.0
  br label %if.end.0
if.else.0:
  %t2 = fdiv double %a, %b
  ret double %t2
  br label %if.end.0
if.end.0:
  ret double 0.0
  }

  define i32 @main() {
  entry:
  %t3 = call double @division(double 30.2, double 2.0)
  %a = alloca double, align 8
  store double %t3, double* %a, align 8
  %t4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str.2, i32 0, i32 0))
  %t5 = load double, double* %a, align 8
  %t6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i32 0, i32 0), double %t5)
  %t7 = call double @division(double 30.2, double 0.0)
  %b = alloca double, align 8
  store double %t7, double* %b, align 8
  %t8 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i32 0, i32 0), i8* getelementptr inbounds ([8 x i8], [8 x i8]* @.str.2, i32 0, i32 0))
  %t9 = load double, double* %b, align 8
  %t10 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.3, i32 0, i32 0), double %t9)
  ret i32 0
  }

