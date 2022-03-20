# Chapter 3

## Variables (Mutability), Constants

You aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.

**You are unable to mutate a variables type.**

### Constants 

You declare constants using the ```const``` keyword instead of the ```let``` keyword, and the type of the value must be annotated.

Constants can be declared in any scope, including the global scope.

Constants are valid for the entire time a program runs, within the scope they were declared in

**Rust’s naming convention for constants is to use all uppercase with underscores between words**

## Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used

Shadowing is different from marking a variable as mut. 
A compile-time error will result if you try assign this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

## Data Types

Rust is a __statically typed__ language.
i.e. it must know the types of all variables at compile time. 

### Scalar Type

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

### Integer Type

An integer is a number without a fractional component. 

Length	|Signed	|Unsigned
--- | --- | ---
8-bit |i8 |u8
16-bit |i16	|u16
32-bit |i32	|u32
64-bit |i64	|u64
128-bit |i128 |u128
arch |isize |usize