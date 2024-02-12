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

mod datatype;
mod function;
mod keywords;
mod shadowing;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    keywords();

    immutable();
    mutable();

    constants();

    shadowing();

    datatype();

    function();
}

fn keywords() {
    keywords::rust_keywords();
}

fn immutable() {
    let x = 5;
    println!("the value of x is:{x}");

    // ^^^^^ cannot assign twice to immutable variable
    // x = 6;
    // println!("The value of x is: {x}");
}

fn mutable() {
    let mut x = 5;
    println!("the value of x is:{x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    println!("The const value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    shadowing::shadowing();
    shadowing::shadowing_spaces();
    shadowing::mut_spaces();
}

fn datatype() {
    datatype::guessing_game();
    datatype::floating_point();
    datatype::numeric_operation();
    datatype::boolean();
    datatype::character();
    datatype::tuple();
    datatype::array();
}

fn function() {
    function::function();
}
