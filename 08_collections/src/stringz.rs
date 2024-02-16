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

pub fn string() {
    println!("----------------------------------------------------------------:string");
    string_simple();

    string_to_string();
    string_from();

    string_unicode();

    string_update();

    string_update_take_ownership();
    string_update_single_character();

    string_combine();
    string_combine_v1();
    string_combine_v2();

    string_index();
    string_wrapper_vec();

    string_hindi_byte();
    string_hindi_char();

    string_string_slice();
    println!("----------------------------------------------------------------:string");
}

pub fn string_simple() {
    let mut s = String::new();
}

pub fn string_to_string() {
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

pub fn string_from() {
    let s = String::from("initial contents");
}

// ----------------------------------------------------------------

pub fn string_unicode() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

// ----------------------------------------------------------------

pub fn string_update() {
    let mut s = String::from("foo");
    // push_str - string slice
    s.push_str("bar");
}

pub fn string_update_take_ownership() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}

pub fn string_update_single_character() {
    let mut s = String::from("lo");
    // push - letter
    s.push('l');
}

// ----------------------------------------------------------------

pub fn string_combine() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 is {s3}");
}

pub fn string_combine_v1() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("s is {s}");
}

pub fn string_combine_v2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
}

// ----------------------------------------------------------------

pub fn string_index() {
    let s1 = String::from("hello");
    // ^ `String` cannot be indexed by `{integer}`
    // let h = s1[0];
}

pub fn string_wrapper_vec() {
    let hello1 = String::from("Hola");
    let hello2 = String::from("Здравствуйте");
    let hello3 = "Здравствуйте";

    //  ^ string indices are ranges of `usize`
    // let answer = &hello3[0];
}

pub fn string_hindi_byte() {
    let s1 = String::from("नमस्ते");

    let mut ctx: Vec<&u8> = Vec::new();
    for x in s1.as_bytes() {
        ctx.push(x)
    }

    println!("the value of नमस्ते bytes is:{:?}", ctx)
}

pub fn string_hindi_char() {
    let s1 = String::from("नमस्ते");

    let mut ctx: Vec<char> = Vec::new();
    for x in s1.chars() {
        ctx.push(x)
    }

    println!("the value of नमस्ते chars is:{:?}", ctx)
}

pub fn string_string_slice() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("the value of s is:{:?}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
}
