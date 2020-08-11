# Indefinite Loop - While and Loop

- While loop 

While loop also iterates until a specific condition is reached. However, in the case of a while loop, the number of iterations is not known in advance.

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/while_loop.png)

- Syntax 
The while keyword is followed by a condition that must be true for the loop to continue iterating and then begins the body of the loop.
the general syntax is :

![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/while_loop_syntax.png)


- Example 

The following example makes use of a while loop to print a variable value. The loop terminates when the variable’s value modulus `3` equals `1`

```

fn main() {
  let mut var = 1; //define an integer variable
  let mut found = false; // define a boolean variable
  // define a while loop
  while !found {
      var=var+1;
      //print the variable
      println!("{}", var);
      // if the modulus of variable is 1 then found is equal to true
      if var % 3 == 1 {
        found = true; 
      }
      println!("Loop runs");
  }
}

```
output

```
2
Loop runs
3
Loop runs
4
Loop runs

```

# Explanation
 - A mutable variable, var, is defined on line 2.
 - A mutable variable, found, is defined on line 3.
 
 
# while loop definition

while loop is defined on line 5. while loop is followed by a variable found. found is initially set to false. !found means that the loop will continue to iterate until the value of found evaluates to be true.The loop terminates when found is set to true.
From here the body of the while loop starts

# while loop body

- The body of the loop is defined from line 5 to line 14.
- In each iteration:
   - The value of the variable var is incremented by 1 on line 6 and then printed on line 8.
   - If the value of the var modulus 3 is equal to 1, then the value of found is set to true else it prints “loop runs” on line 13 and the loop continues.
   
# The following illustration traces the execution of the program:
   








