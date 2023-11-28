@ Architecture: ARMv7
@ System: ARMv7 DE1-SoC

.global _start
_start:
    @ store initial values
	mov r0, #4
	mov r1, #2
	
	@ setting up a stack, frame pointer
	push {r0, r1}
	add r11, sp, #0
	
	@ load second value in the stack
	ldr r3, [r11, #4]

	pop {r0, r1}
	mov r11, sp
    swi #0