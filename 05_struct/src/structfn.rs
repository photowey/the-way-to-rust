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

use structee::User;

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[allow(dead_code)]
struct AlwaysEqual;

pub fn strut_fn() {
    build_user(String::from("superman"), String::from("someone@exmple.com"));

    simple_struct();
    update_struct();

    tuple_struct();
    unit_struct();
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

pub fn simple_struct() {
    let simple = User {
        username: String::from("photowey"),
        email: String::from("photowey@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("The value of simple.User is: {:#?}", simple)
}

pub fn update_struct() {
    let image = User {
        username: String::from("rust"),
        email: String::from("photowey@rust.org"),
        active: true,
        sign_in_count: 1,
    };
    println!("The value of image.User is: {:#?}", image);

    let byset = User {
        username: image.username,
        email: String::from("byset@example.com"),
        active: image.active,
        sign_in_count: image.sign_in_count,
    };
    println!("The value of byset.User is: {:#?}", byset);

    let newest = User {
        email: String::from("newest@example.com"),
        ..byset
    };

    println!("The value of newest.User is: {:#?}", newest)
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!(
        "The value of black.Color is: ({}, {}, {})",
        black.0, black.1, black.2
    );
    println!(
        "The value of origin.Point is: ({}, {}, {})",
        origin.0, origin.1, origin.2
    );
}

fn unit_struct() {
    // let subject = AlwaysEqual;
}
