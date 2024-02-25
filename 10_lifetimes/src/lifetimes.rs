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

// ----------------------------------------------------------------

use std::fmt::Display;

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ----------------------------------------------------------------

pub fn lifetimes() {
    println!("----------------------------------------------------------------:lifetimes");
    lifetimes_dangling_references();
    lifetimes_longest();

    lifetimes_different_scope_v1();
    lifetimes_different_scope_v2();
    lifetimes_different_scope_v3();

    lifetimes_struct();

    lifetimes_elision();

    lifetimes_generic_and_trait_bounds();
    println!("----------------------------------------------------------------:lifetimes");
}

fn lifetimes_dangling_references() {
    /*
    let r;
    {
        let x = 5;
        //  ^^ borrowed value does not live long enough
        r = &x;
    }
    println!("r: {}", r);
    */
}

/*
fn t1() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
*/

/*
fn t2() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
*/

fn lifetimes_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

/*
// ^ expected named lifetime parameter
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

/*
When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is
the part of the scope of x that overlaps with the scope of y. In other words,
the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of
the lifetimes of x and y. Because we’ve annotated the returned reference with
the same lifetime parameter 'a, the returned reference will also be valid for
the length of the smaller of the lifetimes of x and y.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_x<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

/*
fn longest_x_y<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");

    // returns a value referencing data owned by the current function
    // `result` is borrowed here
    result.as_str()
}
*/

fn lifetimes_different_scope_v1() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn lifetimes_different_scope_v2() {
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // ^^^^^^^ borrowed value does not live long enough
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

fn lifetimes_different_scope_v3() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest_x(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// fn first_word<'a>(s: &'a str) -> &'a str {}
fn lifetimes_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
Lifetimes on function or method parameters are called input lifetimes,
and lifetimes on return values are called output lifetimes.

The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is that, if there is exactly one input lifetime parameter,
that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters,
but one of them is &self or &mut self because this is a method,
the lifetime of self is assigned to all output lifetime parameters.
This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/
fn lifetimes_elision() {
    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let _word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn lifetimes_generic_and_trait_bounds() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
