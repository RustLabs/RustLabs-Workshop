# Pattern matching

So how can we know which variant is in a variable whose type is an enumeration and how to get the values out of it?
For that, we need to use pattern matching. The match expression is one way to do pattern matching. Let's see how to use it to compute the result of an expression:

```

fn print_expr(expr: Expr) {
    match expr {
        Expr::Null => println!("No value"),
        Expr::Add(x, y) => println!("{}", x + y),
        Expr::Sub(x, y) => println!("{}", x - y),
        Expr::Mul(x, y) => println!("{}", x * y),
        Expr::Div { dividend: x, divisor: 0 } => println!("Divisor 
         is zero"),
        Expr::Div { dividend: x, divisor: y } => println!("{}",  
        x/y),
        Expr::Val(x) => println!("{}", x),
    }
}



```
A match expression is a way to check whether a value follows a certain pattern and executes different codes for different patterns. In this case, we match over an enumerated type, so we check for each variant. If the expression is `Expr::Add`, the code on the right of` => `is executed: `println!("{}", x + y)`. By writing variable names inside the parentheses next to `Expr::Add`, 
we specify that the actual values of this variant are bound to these names. By doing so, we can use these variable names on the right side of `=>`
