# Task 9:
Create a program for hashing data from a file via open hashing    
Create a hash table as a class, and all operations with it - as methods of the class

## Part 1:    
Create a hash table with B<=30 buckets that allows printing the entire hash table in the terminal, searching for a value.
Use $h(x) = x \% B$ as a hashing function.

## Part 2:
Create a hash table, where the amount of buckets is user-defined. Create methods for search, collision counting, finding the longest collision chain and the load factor of the table.
Use $h(x) = (a\cdot{}x + c)\;mod\;B$ as a hashing function.
Analyze the load factor of the table based on different $a$ and $c$ constants

## Input:
### Variants 1, 4, 7, 10
A file of integers
### Variants 2, 5, 8, 11
A file of real numbers (floats)
### Variants 3, 6, 9, 12
A text file