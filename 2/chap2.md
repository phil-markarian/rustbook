# Chapter 2


## Basics 

The :: syntax in the ::new line indicates that new is an associated function of the String type. 

An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a **static method**.

The & indicates that this argument is a **reference**, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

**When you call a method** with the .foo() syntax, it’s often wise to introduce a newline and other whitespace to help break up long lines.

read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case, an io::Result.

## Enum, result
The Result types are **enumerations**, often referred to as enums. An enumeration is a type that can have a **fixed set of values**, and those values are called the **enum’s variants**.

## Cargo & Crates

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code intended to be used in other programs.

Cargo understands **Semantic Versioning** (sometimes called SemVer), which is a standard for writing version numbers: ```rand="0.4.0"```

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise

Cargo will see that the Cargo.lock file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically

Cargo provides another command, update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml

## RNG
The rand::thread_rng function will give us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. gen_range method on the random number generator.
The gen_range method takes two numbers as arguments and generates a random number between them. It’s inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1 and 101 to request a number between 1 and 100.

## Enum, 0rdering
Like Result, Ordering is another enum, but the variants for Ordering are Less, Greater, and Equal

cmp method ➌ compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with

## Match 
An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn

## Shadowing

Rust allows us to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type.

## Methods

The trim method on a String instance will eliminate any whitespace at the beginning and end

The parse method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type

The call to parse could easily cause an error. If, for example, the string contained A<%, there would be no way to convert that to a number. Because it might fail, the parse method returns a Result type, much as the read_line method does.

## Command line quirks

When the user presses ENTER, a newline character is added to the string. For example, if the user types 5 and presses ENTER, guess looks like this: 5\n. The \n represents “newline,” the result of pressing ENTER. The trim method eliminates \n, resulting in just 5.

## Breaking out of loops

Adding the break line after You win! makes the program exit the loop when the user guesses the secret number correctly.

## Handling invalid input

let’s make the game ignore a non-number so the user can continue guessing. We can do that by altering the line where guess is converted from a String to a u32.

If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resulting number. That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value.

If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error. The Err value does not match the Ok(num) pattern in the first match arm, but it does match the Err(_) pattern in the second arm. The underscore, _, is a catchall value;