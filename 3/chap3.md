# Chapter 3

## 3.1 Variables (Mutability), Constants

You aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.

* You are unable to mutate a variables type.

## Constants 

You declare constants using the ```const``` keyword instead of the ```let``` keyword, and the type of the value must be annotated.

Constants can be declared in any scope, including the global scope.

Constants are valid for the entire time a program runs, within the scope they were declared in

* Rust’s naming convention for constants is to use all uppercase with underscores between words

## Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used

Shadowing is different from marking a variable as mut. 
A compile-time error will result if you try assign this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

## 3.2 Data Types

Rust is a __statically typed__ language.
i.e. it must know the types of all variables at compile time. 

## Scalar Type

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

Signed and unsigned refer to whether it’s possible for the number to be negative.

* Each signed variant can store numbers from -(2<sup>n</sup> - 1) to 2<sup>n - 1</sup> - 1 inclusive.

* Unsigned variants can store numbers from 0 to to 2<sup>n</sup> - 1 inclusive.

**Integer Literals**

Number literals	| Example
--- | ---
Decimal	|98_222
Hex	|0xff
Octal	|0o77
Binary	|0b1111_0000
Byte (u8 only)	|b'A'

## Interger Overflow

Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur, which can result in one of two behaviors

1. (Debug mode) causes program to panic at runtime. 

2. (Relase mode) overflow occurs. Rust performs two's complement wrapping. For u8, the value 256 becomes 0.

To explicitly handle overflowe, use the below methods:

1. Wrap in all modes with the ```wrapping_*``` methods, such as ```wrapping_add```

2. Return the None value if there is overflow with the ```checked_*``` methods

3. Return the value and a boolean indicating whether there was overflow with the ```overflowing_* ```methods

4. Saturate at the value’s minimum or maximum values with ```saturating_*``` methods

## Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64.

* The default is f64.

* All floating-point types are signed.

* Floating-point numbers are represented according to the IEEE-754 standard.

## Numeric Operations

1. addition
2. subtraction
3. multiplication
4. division
5. remainder

* integer divison rounds down to the nearest integer. 

## Boolean Type

Two possible values:
1. True
2. False
   
* specified using ```bool```

## Character Type

* ```char``` literals use single quotes
* string literals use double quotes
* char type is four bytes in size and represents a Unicode Scalar Value.

## Compound Type
Compound types can group multiple values into one type.
1. Tuple
2. Array

* Tuple type
  - Group together a number of values with a variety of types into one compound type.  
  - Fixed length (cannot grow or shrink in size)
  - comma-separated list of values inside parentheses.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1); 
```
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.

```rust
let (x, y, z) = tup;

    println!("The value of y is: {}", y);
```

Breaking up something (a tuple) into parts is called ```destructuring```.

You can access a tuple by using a period. 

```rust
let five_hundered = tup.0;
```

* Array type
  - Every element of an array must have the same type. 
  - Arrays have a fixed length (unlike other languages).
  - comma-separated list inside square brackets

```rust
let a = [1, 2, 3, 4, 5];
```

Arrays are useful when:

1. Want your data allocated on the stack rather than the heap.
2. Want to ensure you always have a fixed number of elements.

* Unlike arrays, a vector is allowed to grow and shrink in size.

Declare an array type and size:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// duplicates the elment 5 times
let a = [3,5];
```

### Accessing Array Elments

Index starts at zero.

```rust
let first = a[0];
```
Rust won't allow you to access an element of an array that is past the end of the array. Rust panics. 

## 3.3 Functions

* Rust code uses snake case as the conventional style for function and variable names.

* Rust doesn’t care where you define your functions, only that they’re defined somewhere

## Parameteres

Functions can have parameters.
I.e. special variables that are part of a function's signature. 

Concrete values are called arguments and not parameters.

* In function signatures, you must declare the type of each parameter.
  
## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.

* Rust is an expression-based language.

```Statements``` are instructions that perform some action and do not return a value.

```Expressions``` evaluate to a resulting value.

The let keyword is a statment. 

Function definitions are statements. 

Since statements do not return values, you **can't** assign a let statement to another variable for example: 
```rust
let x = (let y = 6);
```

The y statement deso not return a vlaue, so there isn't anything for x to bind to. 

You can't do this in Rust. 
```rust
 x = y = 6
 ```
 
### Expresion examples
 * Expressions evalute to a value and make up most of the rest of the code you'll see in Rust. Example let y = 6, 6 is an expression that evalutes to the value 6.
 * Calling a function is an expression. 
 * Calling a macro is an expression.
 * A new scope block created with curly brackets is an expression.
  

**Expressions do not include ending semicolons**. If you add a semicolon to the end of an expression it turns into a statement. 

## Functions with Return Values

* Functions can return values to the code that calls them.
 * We don’t name return values, but we must declare their type after an arrow (->).

* Can return early by using the ```return``` keyword but most functions return the _last expression_ implicitly. 

Statements are expressed by (), the unit type.

## 3.4 Comments (skip)

## 3.5 Control Flow

### if Expressions

Rust does not try to convert non-Boolean types to a Boolean. Must provide if with a Boolean as its condition.

### Using if in a let statement

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

  * **Each arm of the if must be the same type**.

## Repetition with Loops

<font color=#FF0000> **Three kind of loops:**</font>
1. loop
2. while
3. for

### Loop

Excutes for forever until you tell it to stop.

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.

**Specifying a loop with a label**
```rust
let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
```

### Returning Values from Loops

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code

```rust
break counter * 2;
```

### Conditional Loops with while

While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop.

### Looping Through a Collection with for

The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
 
Instead of using a while you can use `for` and `Range` which is provided by the standard library.

Use `rev` to reverse the range.

```rust
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("Liftoff!!");
```

## Try this

1. Convert temperatures between Fahrenheit and Celsius.
2. Generate the nth Fibonacci number.
3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.




   




