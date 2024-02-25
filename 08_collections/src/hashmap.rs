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

use std::collections::HashMap;

pub fn hashmap() {
    println!("----------------------------------------------------------------:hashmap");
    hashmap_simple();
    hashmap_accessing();
    hashmap_iterating();

    hashmap_ownership();

    hashmap_overwriting();

    hashmap_entry_or_insert();
    hashmap_entry_update_value();
    println!("----------------------------------------------------------------:hashmap");
}

fn hashmap_simple() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn hashmap_accessing() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The value of score is: {score}")
}

#[allow(dead_code)]
fn hashmap_iterating() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // ---------- value moved here
    map.insert(field_name, field_value);
    // map.insert(field_name.clone(), field_value);

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // ^^^^^^^^^^^^ value borrowed here after move
    // println!("The value of field_name is: {field_name}")
}

fn hashmap_overwriting() {
    println!(
        "----------------------------------------------------------------:hashmap:overwriting"
    );
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    // Output:
    // {"Blue": 25}
}

fn hashmap_entry_or_insert() {
    println!(
        "----------------------------------------------------------------:hashmap:entry_or_insert"
    );

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // Output:
    // {"Yellow": 50, "Blue": 10}
}

fn hashmap_entry_update_value() {
    println!("----------------------------------------------------------------:hashmap:entry_update_value");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // Output:
    // {"world": 2, "wonderful": 1, "hello": 1}
}

// hasher: https://en.wikipedia.org/wiki/SipHash
