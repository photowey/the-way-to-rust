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

///
/// ### Keywords Currently in Use
///
/// The following is a list of keywords currently in use, with their functionality
/// described.
///
/// * `as` - perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` statements
/// * `async` -  return a `Future` instead of blocking the current thread
/// * `await` - suspend execution until the result of a `Future` is ready
/// * `break` - exit a loop immediately
/// * `const` - define constant items or constant raw pointers
/// * `continue` - continue to the next loop iteration
/// * `crate` - in a module path, refers to the crate root
/// * `dyn` - dynamic dispatch to a trait object
/// * `else` - fallback for `if` and `if let` control flow constructs
/// * `enum` - define an enumeration
/// * `extern` - link an external function or variable
/// * `false` - Boolean false literal
/// * `fn` - define a function or the function pointer type
/// * `for` - loop over items from an iterator, implement a trait, or specify a
///   higher-ranked lifetime
/// * `if` - branch based on the result of a conditional expression
/// * `impl` - implement inherent or trait functionality
/// * `in` - part of `for` loop syntax
/// * `let` - bind a variable
/// * `loop` - loop unconditionally
/// * `match` - match a value to patterns
/// * `mod` - define a module
/// * `move` - make a closure take ownership of all its captures
/// * `mut` - denote mutability in references, raw pointers, or pattern bindings
/// * `pub` - denote public visibility in struct fields, `impl` blocks, or modules
/// * `ref` - bind by reference
/// * `return` - return from function
/// * `Self` - a type alias for the type we are defining or implementing
/// * `self` - method subject or current module
/// * `static` - global variable or lifetime lasting the entire program execution
/// * `struct` - define a structure
/// * `super` - parent module of the current module
/// * `trait` - define a trait
/// * `true` - Boolean true literal
/// * `type` - define a type alias or associated type
/// * `union` - define a [union][union]<!-- ignore -->; is only a keyword when used
///   in a union declaration
/// * `unsafe` - denote unsafe code, functions, traits, or implementations
/// * `use` - bring symbols into scope
/// * `where` - denote clauses that constrain a type
/// * `while` - loop conditionally based on the result of an expression
///
/// ### Keywords Reserved for Future Use
///
/// The following keywords do not yet have any functionality but are reserved by
/// Rust for potential future use.
///
/// * `abstract`
/// * `become`
/// * `box`
/// * `do`
/// * `final`
/// * `macro`
/// * `override`
/// * `priv`
/// * `try`
/// * `typeof`
/// * `unsized`
/// * `virtual`
/// * `yield`
///
#[warn(dead_code)]
pub fn rust_keywords() {}
