# Generics

Generics are a way to make a function or a type work for multiple types to avoid code duplication. Let's rewrite our` max` function to make it generic:

```
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}


```

The first thing to note is that there's a new part after the function name: this is where we declare the generic types. We declare a generic` T `type, `: PartialOrd `after it means that this` T `type must 
implement the `PartialOrd` trait. This is called a trait bound. We then use this T type for both of our parameters and the return type. Then, we see the same function body as the one from our non-generic function. 
We needed to add the trait bound because, by default, no operation is allowed on a generic type. The PartialOrd trait allows us to use the comparison operators.

We can then use this function with any type that implements `PartialOrd`:

```
println!("{}", max('a', 'z'));

```

This is using static dispatch as opposed to dynamic dispatch, meaning that the compiler will generate a max function specific to char in the resulting binary. 
Dynamic dispatch is another approach that resolves the right function to call at runtime, which is less efficient.



# The Option type

- Generics can also be used in a type. The Option type from the standard library is a generic type, defined as such:

```
enum Option<T> {
    Some(T),
    None,
}


```

This type is useful to encode the possibility of the absence of a value. None means no value, while Some(value) is used when there's a value.


# lab 

```
fn main() {
    println!("{}", max('a', 'z'));
}

enum Option<T> {
    Some(T),
    None,
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}


```

[copy code into rust plaground and run ](https://play.rust-lang.org/){: .btn .btn-purple .mr-2 }
