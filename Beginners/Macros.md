# Macros

Macro rules, also called macros by example, are a way to avoid code duplication by generating code at compile time.
We will implement a simple macro to implement our BitSet trait for integer types:

```
macro_rules! int_bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self &= !(1 << index);
            }

            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }

            fn set(&mut self, index: usize) {
                *self |= 1 << index;
            }
        }
    };
}


```
The name of the int_bitset macro is written after `macro_rules!`. A macro can have multiple rules, similar to match arms, 
but it matches on Rust syntactic elements instead, with types, expressions, blocks of code, and so on. Here we only have one rule and it matches against a single type since we use `:ty`. 
The part before `:ty ($ty)` is the name for the element that was matched. Inside the curly brackets, after the `=>` symbol, we see the actual code that will be generated. 
It is the same as our previous implementation of` BitSet` for `u64`, except that it uses the meta-variable `$ty` instead of `u64`.

To avoid a lot of boilerplate code, we can then use this macro as follows:

```

int_bitset!(i32);
int_bitset!(u8);
int_bitset!(u64);

```

# Multiple pattern rules

Let's write a macro that will simplify the implementation of the traits to overload operators. 
This macro will have two rules: one for the `+` and one for the `- `operators. Here's the first rule of the macro:

```
macro_rules! op {
    (+ $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Add for $self_type {
            type Output = $self_type;

            fn add($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };
    // …

```

n this pattern, we use other types of syntactic elements: ident, which is an identifier, and `<span&gt;expr`, which is an expression. The trait `(::std::ops::Add)`
is fully qualified so that the code using this macro won't need to import the Add trait.

And here's the rest of the macro:

```
    (- $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Sub for $self_type {
            type Output = $self_type;

            fn sub($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };
}

```

We can then use this macro with our Point type, like this:

```
op!(+ self:Point, other {
    Point {
        x: self.x + other.x,
        y: self.y + other.y,
    }
});

op!(- self:Point, other {
    Point {
        x: self.x - other.x,
        y: self.y - other.y,
    }
});

```

Let's see how the matching works:

For the first macro call, we start with `+`, so the first branch is taken because it matches `+`, which is the start of this branch. 
Next we have self, which is an identifier, so it matches the ident pattern and this is assigned to the `$_self meta-variable`. Then, we have `:` which matches the colon in the pattern. 
After that, we have Point, which matches the `$self_type meta-variable` of the ty type (for matching on a type). Then we have , which matches the comma in the pattern.
Next, we have other, which matches the next item in the pattern, which is the $other meta-variable of the ident type. Finally, we have `{` Point `{ … } }`, which matches the expression required at 
the end of the pattern. This is why these macros are called macros by example, we write what the call should look like and the user must match the example (or pattern).

- As an exercise to the reader, try the following:

    - Add the missing operators: * and /
    - Add the ability to specify the types of $other and the return type in the pattern
    -  If you haven't already done this in the previous point, add more tokens so that it looks more like a function declaration: `+(self: Point, other: Point) -> Point { … }`
    - Try moving the operator in the pattern after the `$self_type meta-variable` to see the limitations of `macro_rules`

# Repetitions

In a macro pattern, it is also possible to match against an unlimited number of patterns, using the repetition operators `+` and `*`. 
They behave exactly like the same operators in regular expressions:

    - + matches 1 or more times.
    - * matches 0, 1, or more times.
    
Let's write a very useful macro, a macro to provide syntactic sugar to create HashMaps:
Note: A HashMap is a data structure from Rust's standard library that maps keys to values.

```
macro_rules! hash {
    ($( $key:expr => $value:expr ),*) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    }};
}

```
As we can see, we use the `* `operator here. The comma before it specify the separator token: 
this token must be present between each occurrence of the pattern between parentheses (which is the pattern that can be repeated). 
Don't forget the leading `$` before the opening parenthesis; without it, the macro will match the literal (. Inside the parentheses, we see a normal pattern, an expression, followed by the => operator, followed by another expression. The body of this rule is particular, since it uses two pairs of curly brackets instead of only one.

First, let's look at how we use this macro, and we'll go back to this peculiarity right after:

```
let hashmap = hash! {
    "one" => 1,
    "two" => 2
};

```
If we were to use only one pair of curly brackets, like this:

```
macro_rules! hash {
    ($( $key:expr => $value:expr ),*) => {
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    };
}

```
The compiler will try to generate the following code, which doesn't compile:

```
let hashmap = let mut hashmap = ::std::collections::HashMap::new();
    hashmap.insert("one", 1);
    hashmap.insert("two", 2);
    hashmap;

```
It doesn't compile because Rust wants an expression on the right-hand side of `=`. To transform this code into an expression,
we simply need to add the curly brackets:

```
let hashmap = {
    let mut hashmap = ::std::collections::HashMap::new();
    hashmap.insert("one", 1);
    hashmap.insert("two", 2);
    hashmap
};

```
Hence the second pair of curly brackets.

There's one remaining line that requires an explanation in the body of the macro:

```
$(hashmap.insert($key, $value);)*

```
This means that the statement will be repeated as many times as there are pairs of key/values. Notice that `;` is inside the parentheses`;` and 
there's no separator before `* `because every statement needs to end with a semicolon. But it's still possible to specify a separator here, as shown in the following example:

```
let keys = [$($key),*];

```
This will expand all the `$keys`, separating them by a comma. For instance, with a call like:

```
hash! {
    "one" => 1,
    "two" => 2
}

```
It will results in:
```
let keys = ["one", "two"];

```
# Optional quantifier

- In the macro_rules system, there's no way to specify that a pattern is optional, like with the `? `quantifier in regular expressions. If we wanted to allow the user of our hash macro to use a trailing comma,
- we could change the rule by moving the comma inside the parentheses: `($( $key:expr => $value:expr,)*)`.

- However, it will force the user to write a trailing macro. If we want to allow both variants, we can use the following trick, which uses the `*` operator:` ($( $key:expr => $value:expr )`,`* $(,)* )`.

- This means that a comma must be used between each pattern and we can use any number of commas after the last pattern, including no comma at all.

# lab

main.rs

```
#[macro_use]
mod module;

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    let p4 = p1 - p2;
    println!("{:?}", p4);

    let hashmap = hash! {
        "one" => 1,
        "two" => 2,
    };
    println!("{:?}", hashmap);

    /*let hashmap = {
        let mut hashmap = ::std::collections::HashMap::new();
        hashmap.insert("one", 1);
        hashmap.insert("two", 2);
        hashmap
    };*/
}

trait BitSet {
    fn clear(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;
    fn set(&mut self, index: usize);
}

use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

impl<T: BitAnd<Output=T> + BitAndAssign + BitOrAssign + Not + PartialEq + Shl<T> + Shr<usize, Output=T>> BitSet for T {
    fn clear(&mut self, index: usize) {
        *self &= !(1 << index);
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= 1 << index;
    }
}

/*macro_rules! int_bitset {
    ($ty:ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self &= !(1 << index);
            }

            fn is_set(&self, index: usize) -> bool {
                (*self >> index) & 1 == 1
            }

            fn set(&mut self, index: usize) {
                *self |= 1 << index;
            }
        }
    };
}*/

/*int_bitset!(i32);
int_bitset!(u8);
int_bitset!(u64);*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

macro_rules! op {
    (+ $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Add for $self_type {
            type Output = $self_type;

            fn add($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };
    (- $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Sub for $self_type {
            type Output = $self_type;

            fn sub($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };
}

op!(+ self:Point, other {
    Point {
        x: self.x + other.x,
        y: self.y + other.y,
    }
});

op!(- self:Point, other {
    Point {
        x: self.x - other.x,
        y: self.y - other.y,
    }
});



```

module.rs 

```

#[macro_export]
macro_rules! hash {
    ($( $key:expr => $value:expr ),* $(,)* ) => {{
        //let keys = [$($key),*];
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
    }};
}




```

