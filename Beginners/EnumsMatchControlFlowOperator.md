# Enums and Match Control Flow Operator
The match statement can be used to compare values within an enum. The match statement can be written within a main function or any other user-defined function.

# Syntax 
The match statement can be written within a function be it main or any other user-defined function.

# Example 
The example below makes use of a match statement within a `print_direction` function.


```
enum KnightMove{

   Horizontal,Vertical

}

// print function 

fn print_direction(direction:KnightMove) {

   // match statement

   match direction {

      //execute if knight move is horizontal

      KnightMove::Horizontal => {

         println!("Move in horizontal direction");

      },

       //execute if knight move is vertical

      KnightMove::Vertical => {

         println!("Move in vertical direction");

      }

   }

}

fn main() {

   // invoke function `print_direction`

   let knight1 = KnightMove::Horizontal;

   let knight2 = KnightMove::Vertical;

   print_direction(knight1);

   print_direction(knight2);

}


```

output 

```
Move in horizontal direction
Move in vertical direction

```



