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

diagram showing how pattern matching works:
![](https://raw.githubusercontent.com/sangam14/RustLabs/master/img/pattern.png)

A `match` can also be used to check whether a number is within a range.
This function converts an `ASCII` character (represented by u8 in Rust) to uppercase:
```
fn uppercase(c: u8) -> u8 {
    match c {
        b'a'...b'z' => c - 32,
        _ => c,
    }
}
```

Here, the `...` syntax represents an inclusive range. And the underscore `(_)` is used to mean literally everything else, this is very useful in Rust because match needs to be exhaustive.

You can convert u8 to char using the as syntax, as shown earlier:

```
println!("{}", uppercase(b'a') as char);

```
It is also possible to match against different patterns in a `match` by using the `| `operator:

```
fn is_alphanumeric(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' => true,
        _ => false,
    }
}


```

There are alternative syntaxes to do pattern matching. One of them is the if let construct. Let's rewrite our uppercase function using if let:

```

fn uppercase(c: u8) -> u8 {
    if let b'a'...b'z' = c {
        c - 32
    } else {
        c
    }
}


```

Unlike a `match`, if let does not need to be exhaustive. It does not even require an else branch, the rules used for the normal if expression also applies to if let. This construct can be more appropriate than match when you only want to match against one or two patterns.


# Irrefutable patterns

Another form of pattern matching is irrefutable patterns. A pattern is irrefutable when there's only one way to match it and it always succeeds. For instance, another way to get the elements of a tuple is with an irrefutable pattern:

```
let tuple = (24, 42);
let (a, b) = tuple;
println!("{}, {}", a, b);


```

In the second line, we assign the first element of the tuple to `a` and the second to `b`.



