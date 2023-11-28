@ Architecture: ARMv7
@ System: ARMv7 DE1-SoC

.global _start
_start:
    @ Set values to compare
    mov r0, #4
    mov r1, #8
    cmp r0, r1

    @ if: r0 is greater than r1
    bgt greater
    @ else:
    mov r2, #0
    b exit
	
greater:
    mov r2, #1
    b exit
	
exit:
    swi #0
