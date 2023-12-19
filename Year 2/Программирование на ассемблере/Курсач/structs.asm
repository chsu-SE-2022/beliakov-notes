.model small
.stack 100h

.data
; define struct "Klava"
; size of Klava = 7 bytes
Klava STRUC
    keyboard_type   dw 100  ; percentage
    key_count       db 104  ;
    weight          dw 500  ;
    color           db 0    ; ANSI
Klava ENDS
; TKL: 80%, 87 keys
; 60%: 60%, 68 keys
; FS : 100%, 104 keys
Kbs Klava<60, 68, 300, 33>
;    Klava<100, 104, 350, 35> 
;    Klava<80, 87, 600, 34>

.code
ORG 100h
start:
    mov ax, @data
    mov ds, ax              ; set DS to point to data segment
    mov dx, offset Kbs      ; point to the string
    a DW 1,2,3,4,5
;    mov ah, 09h             
; function to print the string
;    int 21h                 
; execute the function

;    mov ax, 4C00h           
; function to terminate the program
;    int 21h                 
; execute
END start

