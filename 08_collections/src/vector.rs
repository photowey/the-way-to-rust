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

pub fn vector() {
    println!("----------------------------------------------------------------:vector");
    vector_define();

    vector_mut();

    vector_reading();
    vector_reading_immutable();

    vector_iterating();
    vector_enum_multi();

    vector_drop();
    println!("----------------------------------------------------------------:vector");
}

pub fn vector_define() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    println!("The value of v1:v2 is:{:?} : {:?}", v1, v2);
}

fn vector_mut() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("The value of v1:v2 is:{:?}", v);
}

fn vector_reading() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn vector_reading_immutable() {
    // let mut v = vec![1, 2, 3, 4, 5];

    // - immutable borrow occurs here
    // let first = &v[0];

    // ^^^^^^^^^ mutable borrow occurs here
    // v.push(6);

    // ------- immutable borrow later used here
    // println!("The first element is: {first}")
}

fn vector_iterating() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 {
        *i += 50;
    }

    for i in &v1 {
        println!("{i}");
    }
}

fn vector_enum_multi() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn vector_drop() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    /*
    When the vector gets dropped, all of its contents are also dropped,
    meaning the integers it holds will be cleaned up.
    The borrow checker ensures that any references to contents of a vector
    are only used while the vector itself is valid.
    */
}
