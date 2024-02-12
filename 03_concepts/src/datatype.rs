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
