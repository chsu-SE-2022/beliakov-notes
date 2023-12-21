CODE SEGMENT
    ASSUME CS:CODE
    ORG 100h
Start:

; var 5 
; (a-4)/(b+c)-d
MOV AX,a    ; declare AX = a
SUB AX, 4   ; AX = a - 4
MOV BX,b    ; declare BX = b 
MOV CX,c    ; declare CX = c
ADD BX,CX   ; add BX+CX, store in BX
IDIV BL     ; divide AX/BX
MOV DX, d   ; declare DX = d
MOV BL, AH  ; move remainder to BH to not lose it
SUB AX,DX   ; substract AX-DX, store in AX
INT 21h
a DW 11
b DW 1
c DW 2
d DW 3
CODE ENDS
END Start
END Start
