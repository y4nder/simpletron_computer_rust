VAR x
LOADI 1
JG greater
HALT

greater:
LOADI 1
JNZ nonzero
HALT

nonzero:
LOADI 50
WRITEA
HALT
