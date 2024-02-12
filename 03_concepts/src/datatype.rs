/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::io;

/// data-type
/// - scalar
/// - compound
///
/// A scalar type represents a single value. Rust has four primary scalar types:
/// integers, floating-point numbers, Booleans, and characters.
///
///
/// #### integers
///
/// | Length  | Signed  | Unsigned |
// |---------|---------|----------|
// | 8-bit   | `i8`    | `u8`     |
// | 16-bit  | `i16`   | `u16`    |
// | 32-bit  | `i32`   | `u32`    |
// | 64-bit  | `i64`   | `u64`    |
// | 128-bit | `i128`  | `u128`   |
// | arch    | `isize` | `usize`  |
///
/// Decimal - 98_222
/// Hex - 0xff
/// Octal - 0o77
/// Binary - 0b1111_0000
/// Byte - 'A'
///
///
/// #### floating-point numbers
/// f32
/// f64
///
///
/// #### Numeric Operations
///
/// - addition
/// - subtraction
/// - multiplication
/// - division
/// - remainder
///
///
/// #### Boolean
/// - true
/// - false
///
///
/// #### Character
///
///
/// #### Compound
/// - Tuple
/// - Array

pub fn guessing_game() {
    println!("----------------------------------------------------------------:guessing_game");
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");
    println!("----------------------------------------------------------------:guessing_game");
}

pub fn floating_point() {
    println!("----------------------------------------------------------------:floating_point");
    let x = 2.0;
    let y: f32 = 10086.99;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("----------------------------------------------------------------:floating_point");
}

pub fn numeric_operation() {
    println!("----------------------------------------------------------------:numeric_operation");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    println!("The value of sum is: {sum}");
    println!("The value of difference is: {difference}");
    println!("The value of product is: {product}");
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    println!("The value of remainder is: {remainder}");
    println!("----------------------------------------------------------------:numeric_operation");
}

pub fn boolean() {
    println!("----------------------------------------------------------------:boolean");
    let t = true;
    let f = false;

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");
    println!("----------------------------------------------------------------:boolean");
}

pub fn character() {
    println!("----------------------------------------------------------------:character");
    let c = 'z';
    let z: char = 'Z';

    let heart_eyed_cat = ' ';

    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
    println!("----------------------------------------------------------------:character");
}

pub fn tuple() {
    println!("----------------------------------------------------------------:tuple");
    tuple_define();
    tuple_destructure();
    tuple_dot_accessing();
    println!("----------------------------------------------------------------:tuple");
}

pub fn tuple_define() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
}

pub fn tuple_destructure() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}")
}

pub fn tuple_dot_accessing() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
}

pub fn array() {
    println!("----------------------------------------------------------------:array");
    array_define();
    array_months();
    array_define_type();
    array_define_same_value();
    array_accessing();

    // array_invalid_accessing();

    println!("----------------------------------------------------------------:array");
}

fn array_define() {
    let a = [1, 2, 3, 4, 5];

    println!("The value of array.a is: {:?}", a);
}

fn array_months() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("The value of array.months is: {:?}", months);
}

fn array_define_type() {
    // i32
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of array.a is: {:?}", a);
}

fn array_define_same_value() {
    let a = [3; 5];

    println!("The value of array.a is: {:?}", a);
}

fn array_accessing() {
    let a = [1, 2, 3, 4, 5];

    println!("The first value of array.a is: {:?}", a[0]);
    println!("The second value of array.a is: {:?}", a[1]);
}

#[warn(dead_code)]
fn array_invalid_accessing() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // Input: 5
    // Output:
    // thread 'main' panicked at 03_concepts\src\datatype.rs:234:19:
    // index out of bounds: the len is 5 but the index is 5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\concepts.exe` (exit code: 101)
}
