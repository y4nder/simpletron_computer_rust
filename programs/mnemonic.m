; ------------------------------------------------------------
; Author: Leander Lorenz B. Lubguban BSCS 3-A
; Program: Factorial using Simpletron (current assembler)
;
; Description:
; - Reads an integer n from input
; - Computes n! using a loop
; - Stores the result in variable `fact`
; - Outputs the factorial result
;
; Requirements:
; - Explicit VAR declarations
; - Explicit LOADM / LOADI / MULM
; - Labels use `label:` syntax
; ------------------------------------------------------------

; --------------------
; Variable declarations
; --------------------
VAR n           ; variable to store input number n
VAR fact        ; variable to store factorial result

; --------------------
; Initialization
; --------------------
READ   n        ; read input value into variable n
LOADI  1        ; load constant 1 into accumulator
STORE  fact     ; initialize fact = 1
LOADM  n        ; load n into accumulator (for loop condition)

; --------------------
; Loop: while (n != 0)
; --------------------
loop:
JZ     display  ; if accumulator == 0, jump to display (end loop)

LOADM  fact     ; load current factorial value
MULM   n        ; multiply by n
STORE  fact     ; store updated factorial back to fact

LOADM  n        ; load n
SUBI   1        ; n = n - 1
STORE  n        ; store updated n

JMP    loop     ; repeat loop

; --------------------
; Output result
; --------------------
display:
WRITE  fact     ; output factorial result
HALT            ; stop program execution

