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

pub fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

pub fn shadowing_spaces() {
    // 5
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}

#[allow(unused_mut)]
pub fn mut_spaces() {
    // 5
    let mut spaces = "     ";

    // ^^^^^^^^^^^^ expected `&str`, found `usize`
    // spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
