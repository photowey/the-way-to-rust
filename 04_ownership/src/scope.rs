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

pub fn scope() {
    println!("----------------------------------------------------------------:scope");
    scope_define();
    println!("----------------------------------------------------------------:scope");
}

fn scope_define() {
    {
        // s is not valid here, it’s not yet declared

        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("The value of s is: {s}");
    } // this scope is now over, and s is no longer valid

    // When s comes into scope, it is valid.
    // It remains valid until it goes out of scope.
}

pub fn type_string() {
    println!("----------------------------------------------------------------:type_string");
    type_string_from();
    type_string_scope();
    type_string_move();
    println!("----------------------------------------------------------------:type_string");
}

pub fn type_string_from() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("The value of String::form.s is:{}", s); // This will print `hello, world!`

    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we're done with our String.
}

fn type_string_scope() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        println!("The value of s is: {s}");
    } // this scope is now over, and s is no
      // longer valid
}

fn type_string_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    // ^^ value borrowed here after move
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // move
    /*
    If you’ve heard the terms shallow copy and deep copy while working with other languages,
    the concept of copying the pointer, length, and capacity without copying
    the data probably sounds like making a shallow copy.
    But because Rust also invalidates the first variable,
    instead of being called a shallow copy, it’s known as a move

    In addition, there’s a design choice that’s implied by this:
    Rust will never automatically create “deep” copies of your data.
    Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    */
}

fn type_string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn ownership_function() {
    try_take_ownership_function();
    return_value_and_scope();
    take_ownership_return_tuple();

    // references
    take_references_function();

    // slice
    take_references_slice();
    take_references_string_slice();

    /*
    The ownership of a variable follows the same pattern every time:
    assigning a value to another variable moves it.
    When a variable that includes data on the heap goes out of scope,
    the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    */
}

// ----------------------------------------------------------------

fn try_take_ownership_function() {
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // Passing a variable to a function will move or copy, just as assignment does.

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("The value of x is: {x}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// ----------------------------------------------------------------

fn return_value_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("The value of s1 is: {s1}");
    //  ^^^^ value borrowed here after move
    // println!("The value of s2 is: {s2}");

    println!("The value of s3 is: {s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// ----------------------------------------------------------------

fn take_ownership_return_tuple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// ----------------------------------------------------------------

/*
References:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
*/
fn take_references_function() {
    take_references_function_reference();
    take_references_function_reference_and_change();

    take_references_function_handle_data_race();
    take_references_function_handle_data_race_big_problem();

    take_references_function_dangling_reference();
}

fn take_references_function_reference() {
    let s1 = String::from("hello");

    // &s1: These ampersands represent references,
    // and they allow you to refer to some value without taking ownership of it
    let len = calculate_length_reference(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_reference(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn take_references_function_reference_and_change() {
    let s = String::from("hello");
    change(&s);

    // 1: let mut s_mut
    let mut s_mut = String::from("hello");
    // 2: &mut s_mut
    mut_change(&mut s_mut);
}

fn change(some_string: &String) {
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // some_string.push_str(", world");
}

fn mut_change(some_string: &mut String) {
    // 3: &mut String
    some_string.push_str(", world");
}

// ----------------------------------------------------------------

fn take_references_function_handle_data_race() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

        println!("The value of r1 is: {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("The value of r2 is: {r2}");
}

#[warn(unused_mut)]
fn take_references_function_handle_data_race_big_problem() {
    // let mut s = String::from("hello");
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // ^^^^^^ mutable borrow occurs here
    // let r3 = &mut s; // BIG PROBLEM

    // -- immutable borrow later used here
    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);

    // We also cannot have a mutable reference while we have an immutable one to the same value.

    /*
    Users of an immutable reference don’t expect the value to suddenly change out from under them!
    However, multiple immutable references are allowed because no one who is just reading the data
    has the ability to affect anyone else’s reading of the data.
    */
}

// ----------------------------------------------------------------

fn take_references_function_dangling_reference() {
    take_references_function_dangling_reference_small_sample();
}

fn take_references_function_dangling_reference_small_sample() {
    // let reference_to_nothing = dangle();

    let no_dangle_string = no_dangle();
    println!("The value of no_dangle_string is: {no_dangle_string}");
}

/*
fn dangle() -> &String {
    // ^ expected named lifetime parameter
    let s = String::from("hello");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s

    // This works without any problems. Ownership is moved out, and nothing is deallocated.
}

// ----------------------------------------------------------------

/*
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.
*/
fn take_references_slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
                               // let string_slice_value = first_word_string_slice(&s);

    println!("Before: the value of word is: {word}");
    s.clear(); // this empties the String, making it equal to ""
    println!("After: the value of word is: {word}");

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // println!("The value of string_slice_value is: {string_slice_value}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn take_references_string_slice() {
    let s = String::from("hello world");
    // range: [x..y] -> [x,y)
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("The value of s[0..5] is: {hello}");
    println!("The value of s[6..11] is: {world}");
}

fn first_word_string_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
