CODE SEGMENT
    ASSUME CS:CODE
    ORG 100h
Start:
; (a - 4 & 2) / (!b + (c ^ 5)) - !d
MOV AX, a   ; declare AX = a
SUB AL, 4   ; AX = a - 4
AND AL, 2   ; AL = AL & 2
MOV BL, AL  ; store AL in BL
MOV AX, b   ; declate AX = b
NOT AL      ; AL = !AL
MOV CX, c   ; declare CX = c
OR CL, 5    ; CL = CL ^ 5
ADD AL, CL  ; AL += CL
MOV DX, AX  ; move AX to DX
MOV AX, BX  ; move BX to AX
IDIV DL     ; AX / DL, store in AX
MOV CX, d   ; declare CX = d
NOT CL      ; CL = !CL
SUB AL, CL  ; AL = AL - CL

a DW 42
b DW 60
c DW 16
d DW 254

CODE ENDS
END Start