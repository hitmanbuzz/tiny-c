; ModuleID = 'source.tc'
source_filename = "source.tc"

define i32 @main() {
entry:
  %x = alloca float, align 4
  store float 1.000000e+00, ptr %x, align 4
  %y = alloca float, align 4
  store float 2.000000e+00, ptr %y, align 4
  %result = alloca float, align 4
  %x1 = load float, ptr %x, align 4
  %y2 = load float, ptr %y, align 4
  %addtmp = fadd float %x1, %y2
  store float %addtmp, ptr %result, align 4
  ret i32 69
}
