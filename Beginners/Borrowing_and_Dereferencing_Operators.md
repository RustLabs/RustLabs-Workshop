# Borrowing and Dereferencing Operators

# Borrowing Operator

Borrowing means to reference the original data binding or to share the data.

References are just like pointers in C.

Two variables are involved in a borrowing relationship when the referenced variable holds a value that the referencing variable borrows. 
The referencing variable simply points to the memory location of the referenced variable.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/mem_ref_var.png)

The following illustration shows that operand 1 borrows the value of operand 2 using two types of operators:

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/operator_mem.png)

# Types 

Borrowing can be of two types:
   - Shared borrowing
   - A piece of data that is shared by single or multiple variables but it cannot be altered
   - Mutable borrowing
   - A piece of data that is shared and altered by a single variable (but the data is inaccessible to other variables at that time)
   - The following table summarizes the function of these two types.
   
  
| operator 	| operation  	| explanation 	|
|-	|-	|-	|
| Operand1 = & Operand2<br> 	| shared borrow  	| operand 1 can read data of  another operand 2   	|
| Operand1 = & mut Operand2  	| mutable borrow  	| Operand 1 can read and alter data of another operand2  	| 
 
 
  
