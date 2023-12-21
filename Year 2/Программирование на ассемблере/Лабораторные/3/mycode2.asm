

org 100h

mov ch,9
mov cl,0
mov bl,10

jmp for 

if:
mov ax,0
mov al,ch
div bl
add al,ah
cmp al,k
jne for
inc cl 
jmp for

for:
inc ch
cmp ch,99 
jna if 





k db 3


ret




