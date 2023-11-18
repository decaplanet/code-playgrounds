.global _start

_start:
    @ Set values to compare
    mov r0, #2
    add r0, r0, #2
    add r1, #8
    cmp r0, r1

    @ If r0 is greater than r1
    bgt greater

    @ Or else
    mov r2, #0
    b exit
	
greater:
    mov r2, #1
    b exit
	
exit:
    mov r7, #1
    swi #0
