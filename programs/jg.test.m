VAR x

LOADI 0
ADDI 1
STORE x

LOADM x
JN negative
JG positive
JMP zero

negative:
WRITE x
HALT

positive:
WRITE x
HALT

zero:
WRITE x
HALT
