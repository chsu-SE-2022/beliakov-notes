.8086
.model small
.stack 100h
.data
n dw 5  ; limit for N
j dw 2
i dw 1
.code
MOV AX, @data   ; read from data segment
MOV DS, AX      
MOV BX, i       ; BX is i, incremented on each step
MOV CX, n       ; LOOP has CX steps, so CX = n makes it n steps
M1:	
    MOV AX, BX  ; AX = i
    IMUL j      ; i * j
    SUB AX, 1   ; (i * j) - 1
    ADD SI, AX  ; store in SI
    INC BX      ; increment i
loop M1
end
