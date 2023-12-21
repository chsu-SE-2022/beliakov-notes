
CODE SEGMENT
	ASSUME CS,DS:CODE
	ORG 100H
Start:
 


MOV AX,a
cmp AX,b   
jnlE F1

cmp aX,c
jnge F3

MOV BX,B
ADD BX,AX  
MOV AX,C
IMUL BX
MOV BX,6
SUB BX,AX
jmp konec

F1:  
mov BX,4
IMUL BX
MOV BX,AX 
MOV AX,B
MOV CX,C
IDIV CX
ADD BX,aX
MOV CX,6
SUB BX,CX
jmp konec  

F3:
MOV CX,AX  
MOV BX,B
MOV AX,7
ADD AX,BX
MOV BX,5
IMUL BX
MOV CX,AX 
MOV BX,a
MOV AX,3
IDIV BX
SUB AX,CX



jmp konec 



 
Konec:
mov ah, 04Ch
int 21h 	
         
           
a DW  3
b DW  2
c DW  2

CODE ENDS
	END Start

