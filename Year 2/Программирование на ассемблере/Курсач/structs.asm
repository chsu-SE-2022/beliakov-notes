.model small
.stack 100h

.data
; define struct "Klava"
; size of Klava = 8 bytes
Klava STRUC
    keyboard_type   dw 100  ; percentage
    key_count       dw 104  ;
    weight          dw 500  ;
    color           dw 33   ; ANSI
Klava ENDS
; TKL: 80%, 87 keys
; 60%: 60%, 68 keys
; FS : 100%, 104 keys
Kbs Klava<60, 68, 1, 33>, <100, 104, 1, 33>, <80, 87, 1, 33>, <60, 68, 1, 33>, <100, 104, 1, 33>, <80, 87, 1, 33>, <60, 68, 1, 33>, <100, 104, 1, 33>, <80, 87, 1, 33>, <60, 68, 1, 33>, <100, 104, 1, 33>, <80, 87, 1, 33>, <60, 68, 1, 33>, <100, 104, 1, 33>, <80, 87, 1, 33>, 
.code
ORG 100h
start:
    mov ax, @data           ; set AX to data segment
    mov DS, AX              ; set DS to point to data segment
    xor AX, AX              ; null AX
    mov CX, 15              ; set CX to the amount of keyboards (15)
    xor BX, BX
    xor DI, DI              ; null registers
    XOR SP, SP
    cmp CX, 0
    JA iterate
    
    iterate:
        DEC CX                  ; decrement CX
        XOR DX, DX              ; empty DX
        MOV DX, Kbs.weight[BX]  ; DX = i'th kb weight
        CMP DX, 400             ; DX <= 400
        JLE check_color         ; true => jump to color check
        ADD BX, TYPE Klava      ; shift address
        CMP CX, 0               ; false => check CX != 0
        JA iterate              ; true => next iteration
        JMP stop                ; false => stop
        check_color:            ; weight correct => check color
        XOR DX, DX              ; empty DX
        MOV DX, Kbs.color[BX]   ; DX = i'th kb color
        CMP DX, 33              ; DX == 33
        JE increase             ; true => jump to addition
        ADD BX, TYPE Klava      ; shift address
        CMP CX, 0               ; check CX != 0
        JA iterate              ; true => next iteration
        JMP stop                ; false => stop
    increase:
        INC SP                  ; color correct => increment SP
        ADD BX, TYPE Klava      ; shift address
        CMP CX, 0               ; check CX != 0
        JA iterate              ; true => next iteration
        JMP stop                ; false => stop
    stop:
        INT 21h
END start