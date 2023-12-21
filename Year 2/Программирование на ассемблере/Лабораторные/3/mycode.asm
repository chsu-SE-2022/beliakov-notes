
CODE SEGMENT
	ASSUME CS:CODE
	ORG 100H
Start:
 
a DB  6
b DB  7
c DB  1
d DB  0

MOV AL,a
cmp al,b   
jbe Sravnenie1

cmp al,c
jbe Cmax
mov dl,a
jmp konec

Sravnenie1:  
mov bl,b
cmp bl,c
jbe Cmax 
mov dl,b
jmp konec  

Cmax:  
mov dl,c
jmp konec


 
Konec:
mov ah, 04Ch
int 21h 	

CODE ENDS
	END Start





