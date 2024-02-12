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

/// #### Function
/// - parameters
/// - arguments

pub fn function() {
    println!("----------------------------------------------------------------:function");
    function_define();

    function_statement();
    function_expression();
    function_statement_and_expression(1024);

    function_return();
    println!("----------------------------------------------------------------:function");
}

fn function_define() {
    another_function();
    // arguments: 5
    another_function_parameter(5);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.")
}

fn another_function_parameter(x: i32) {
    // parameter: x
    println!("The value of function.parameter.x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    // Output:
    // The measurement is: 5h
}

fn function_statement() {
    let y = 6;

    println!("The value of y is: {y}");
}

fn function_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn function_statement_and_expression(x: i32) -> i32 {
    println!("The value of function_statement.parameter.x is: {x}");

    // statements
    let y = x * 2;

    println!("The value of y is: {y}");

    // expressions
    x
}

fn function_return() {
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of plus_one.x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    //  ^^^ expected `i32`, found `()`
    // x + 1;

    x + 1
}
