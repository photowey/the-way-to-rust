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

/// ownership
///
/// * Each value in Rust has an *owner*.
/// * There can only be one owner at a time.
/// * When the owner goes out of scope, the value will be dropped.
///
/// - borrowing
/// - slice
mod scope;

fn main() {
    scope();
}

fn scope() {
    scope::scope();
    scope::type_string();
    scope::ownership_function();
}
