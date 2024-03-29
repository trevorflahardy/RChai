# Chai

Chai is a LLVM-compiled programming language written in Rust, aimed to be blazingly fast while user-friendly.

## Current State

The project is currently in active development. Essentially no core features of the language have been implemented, including variable declarations, control flow statements, and basic data types. There are still MANY areas that require refinement and optimization.

## Future Plans

In the upcoming months, the main focus will be on the following areas:

- Adding support for advanced data structures such as arrays, structs, etc.
- Implementing a module system to enable code organization and reusability.
- Enhancing the error handling mechanism to provide more informative error messages.
- Improving the performance of the compiler and runtime.


Looking forward, the following will also be attended to:

- Optimizing the language for speed, safety, and easy control flow.
- Bootstrapping the compiler for future releases.

## Goals

Some main goals for Chai are:

- To create a language that is easy to learn and use, while still being powerful and expressive.
- To provide a seamless integration with existing Rust code, allowing developers to leverage the Rust ecosystem.

## Expected Syntax
```rust
struct SomeStruct {
    ... // Fields
}

fn some_function(param1: String, param2: u32) -> SomeReturnType {
    ...
}

some_function("This is param 1", 10);
print("Hello from Chai!");
```