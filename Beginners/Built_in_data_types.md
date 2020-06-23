---
layout: default
title:  Built-in data types
parent: Rust for Beginners
nav_order: 4
---

# Built-in data types
- Let's look at the basic types provided by the language, such as integers, floats, Booleans, and characters.

# Integer types

- The following integer types are available in Rust:

|   Unsigned    |   Signed  | 
|:-------------|:------------------|
| u8           |  i8  | 
| u16          | i16 | 
| u32          | i32 | 
| u64          | i64 |
|  usize       | isize |


- The `u` means unsigned, while the `i` means signed, and the number following it is the number of bits. For instance, a number of the `u8` type can be between `0 `and `255`, inclusive. 
- And a number of the i16 type can be between `-32768` and `32767`, inclusive. 
- The size variants are the pointer-sized integer types: `usize` and isize are `64-bit` on a` 64-bit` CPU. 
- The default integer type is `i32`, which means that this type will be used by the type inference when it cannot choose a more specific type.

# floating-point types

- There are two floating-point types: `f32` and `f64`, the latter being the default. The number following `f `represents the number of bits for the type.
An example value is `0.31415e1`.

# Boolean type

The bool type admits two values: ` true` and `false`.

# Character type

The char type represents a Unicode character. An example unicode scalar value is '€'.
