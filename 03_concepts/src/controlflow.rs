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

pub fn control_flow() {
    println!("----------------------------------------------------------------:control_flow");
    control_flow_if(3);
    control_flow_if_else_if();
    control_flow_let_if();

    // control_loop();

    control_flow_loop_break();
    control_flow_loop_label();

    control_flow_while();
    control_flow_while_trav();

    control_flow_for();
    control_flow_for_in();
    control_flow_for_in_eq();
    println!("----------------------------------------------------------------:control_flow");
}

fn control_flow_if(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    let number = 3;
    // ^^^^^^ expected `bool`, found integer
    if number {
        println!("number was three");
    }
    */

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn control_flow_if_else_if() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn control_flow_let_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    /*
    // ^^^^^ expected integer, found `&str`
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");
    */
}

#[warn(dead_code)]
fn control_flow_loop() {
    // dangerous
    loop {
        println!("again!");
    }
}

fn control_flow_loop_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn control_flow_loop_label() {
    let mut count = 0;

    'counting_up: loop {
        println!("outer.loop:count = {count}");
        let mut remaining = 10;

        loop {
            println!("inner.loop:remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn control_flow_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn control_flow_while_trav() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the while.item.value is: {}", a[index]);
        index += 1;
    }
}

fn control_flow_for() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the for.item.value is: {element}");
    }
}

fn control_flow_for_in() {
    // (1..4) == [1,4)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn control_flow_for_in_eq() {
    // (1..=4) == [1,4]
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
