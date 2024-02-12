fn main() {
// Variables and mutability //////////////////////////////////////////////////////////////////////////////////////////////////////////


let mut x = 5;                      // mut is short for mutable
println!("the value of x is {}", x);

x = 6;                                   // x is mutable, so we can change its value
println!("the value of x is {}", x);


let y = 5;                          // x is immutable by default
println!("the 2nd value of x is {}", y);

let y = "six";                     // we can shadow y, which means we can redeclare x with a new value
println!("the 2nd value of x is {}", y);
const CONSTANT: u32 = 100_000;           // constants are always immutable

// Data types ///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// integer types
let x: u8 = 255; // 0 to 255
let x: i8 = -128; // -128 to 127
let x: u16 = 65535; // 0 to 65535
let x: i16 = -32768; // -32768 to 32767
let x: u32 = 4294967295; // 0 to 4294967295
let x: i32 = -2147483648; // -2147483648 to 2147483647
let x: u64 = 18446744073709551615; // 0 to 18446744073709551615
let x: i64 = -9223372036854775808; // -9223372036854775808 to 9223372036854775807
let x: u128 = 340282366920938463463374607431768211455; // 0 to 340282366920938463463374607431768211455
let x: i128 = -170141183460469231731687303715884105728; // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727

let a = 98_222; // with underscores for readability
let b = 0xff; // hexadecimal
let c = 0o77; // octal
let d = 0b1111_0000; // binary
let e = b'A'; // byte (u8 only)


// Floating Point Types


let x: f32 = 2.0; // f64
let y: f32 = 3.0; // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;


// Boolean Type

let t: bool = true;
let f: bool = false; 


// Character Type
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';

// Compound Types

    // Tuple Type
    let tup: (&str, i32, f32) = ("hello", 5, 6.4);
    let tup = ("hello", 5, 6.4);

    let (x, y, z) = tup; // destructuring

    let float = tup.2; // access by index from 0

    // Array Type
    let error_codes = [1, 2, 3, 4, 5];
    let error_codes: [i32; 5] = [1, 2, 3, 4, 5]; // type and length

    let not_found = error_codes[0];

    let byte = [0; 5]; // [0, 0, 0, 0, 0]


// Functions ///////////////////////////////////////////////////////////////////////////////////////////////////////////////



}

