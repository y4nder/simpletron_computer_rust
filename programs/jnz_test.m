VAR x
LOADI 5
STORE x

loop:
LOADM x
JZ end
SUBI 1
STORE x
JMP loop

end:
HALT
