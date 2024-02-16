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

/// Rust groups errors into two major categories: recoverable and unrecoverable errors
pub fn errors() {
    println!("----------------------------------------------------------------:errors");
    errors_call_panic_macro();
    errors_code_bug();
    println!("----------------------------------------------------------------:errors");
}

pub fn errors_call_panic_macro() {
    // dangerous
    // panic!("crash and burn");

    /*
    thread 'main' panicked at 09_errors\src\errors.rs:26:5:
    crash and burn
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\errors.exe` (exit code: 101)
    */
}

pub fn errors_code_bug() {
    // let v = vec![1, 2, 3];
    // v[99];

    /*
    thread 'main' panicked at 09_errors\src\errors.rs:39:6:
    index out of bounds: the len is 3 but the index is 99
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\errors.exe` (exit code: 101)
    */
}
