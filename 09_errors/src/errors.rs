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

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};

/// Rust groups errors into two major categories: recoverable and unrecoverable errors
pub fn errors() {
    println!("----------------------------------------------------------------:errors");
    errors_panic_call_panic_macro();
    errors_panic_code_bug();

    errors_result();
    errors_result_handle_err();
    errors_result_unwrap_or_else();

    errors_result_shortcut_unwrap();
    errors_result_shortcut_expect();

    errors_result_shortcut_propagating();
    println!("----------------------------------------------------------------:errors");
}

fn errors_panic_call_panic_macro() {
    // dangerous
    // panic!("crash and burn");

    /*
    thread 'main' panicked at 09_errors\src\errors.rs:26:5:
    crash and burn
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\errors.exe` (exit code: 101)
    */
}

fn errors_panic_code_bug() {
    // let v = vec![1, 2, 3];
    // v[99];

    /*
    thread 'main' panicked at 09_errors\src\errors.rs:39:6:
    index out of bounds: the len is 3 but the index is 99
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\errors.exe` (exit code: 101)
    */
}

fn errors_result() {
    let greeting_file_result = File::open("hello.txt");

    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    */
}

fn errors_result_handle_err() {
    let greeting_file_result = File::open("hello.txt");

    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    */
}

fn errors_result_unwrap_or_else() {
    /*
     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
         if error.kind() == ErrorKind::NotFound {
             File::create("hello.txt").unwrap_or_else(|error| {
                 panic!("Problem creating the file: {:?}", error);
             })
         } else {
             panic!("Problem opening the file: {:?}", error);
         }
     });
    */
}

fn errors_result_shortcut_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn errors_result_shortcut_expect() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn errors_result_shortcut_propagating() {
    read_username_from_file();

    read_username_from_file_shortcut();
    read_username_from_file_shortcut_chain();

    read_username_from_file_fs();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortcut_chain() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ----------------------------------------------------------------

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn option_shortcut() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
