# Chai Programming Language Syntax
Chai is a statically typed language that is inforced by the compiler. Chai must know the size
and type of everything.

```chai
// Primitive types:
let unsigned_int_8: u8 = 0;
let unsigned_int_16: u16 = 0;
let unsigned_int_32: u32 = 0;
let unsigned_int_64: u64 = 0;

let signed_int_8: s8 = 0;
let signed_int_16: s16 = 0;
let signed_int_32: s32 = 0;
let signed_int_64: s64 = 0;

let float_8: f8 = 0;
let float_16: f16 = 0;
let float_32: f32 = 0;
let float_64: f64 = 0;

let array_of_item: [u8; 2] = [0, 1];

let some_char: char = 'A';
let some_char_array: [char; 2] = ['A', 'B'];

// Higher level types. All immutable by default unless it has owned access or the reference is a mutable reference.
let some_string: String = "This is a string!"

// :: denotes a class method
let some_vec: Vec[u8] = Vec::new();

// References
let some_char: char = 'A';
let some_char_ref: &char = &some_char;
print("Some char ref: " + some_char_ref);
```