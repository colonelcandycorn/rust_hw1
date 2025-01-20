# Homework 1 CS523
## Author
Sarah Dylan

## Program Description

The goal of homework one is to use Rule 110 of the Elementary
Cellular Automaton. Specifically, we are given an 8 character string
that consists of only '*' and '.'. These characters correspond to 1 and 0
respectively. And our goal is to apply rule 110 to get the next string.

We are given the starting string "\*.\*..\*.." and told to print it and then the next 9
strings in the sequence. I accomplished this by creating a series of functions that
perform the necessary operations.

The first transformation of "\*.\*..\*.." should be "\*\*\*.\*\*.\*" for example.

## Build and Run Instructions

1. **Prerequisites**:
    * Rust and cargo installed
2. **Clone the Repository**:
   ```
   git clone https://github.com/colonelcandycorn/rust_hw1.git
   cd rust_hw1
   ```
3. **Build the Project**:
    ```
    cargo build
    ```
4. **Run the Program**:
    ```
    cargo run
    ```
5. **Run tests**:
    ```
    cargo test
    ```

## Issues Encountered
* It's been a minute since I've dealt with bit manipulation. I know
that the instructions in the assignment specified that we could use 
an array of bools instead of u8's, but I opted with u8's because one
of the projects I was considering for this class would require knowledge
of bit manipulation
* I went back and forth with how to design these functions. Specifically,
I was debating between just having a two char arrays of size 8 and using
that to compute results or to just start with a string and output a new string
at the end. I ended up deciding on the more functional approach of leaving
the string that starts the whole process in `next_cellular_automaton_string(word: &str) -> String`.
I know that this uses up more memory, but I felt like it might be more realistic that
you'd want to be able to create like a vector of strings at the end.

## Useful Information

Although this is a binary crate. All the functions are contained in a library.