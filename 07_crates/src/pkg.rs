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

/*

A crate can come in one of two forms: a binary crate or a library crate.
Binary crates are programs you can compile to an executable that you can run,
such as a command-line program or a server. Each must have a function called main that defines
what happens when the executable runs. All the crates we’ve created so far have been binary crates.

Library crates don’t have a main function, and they don’t compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.
For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers.
Most of the time when Rustaceans say “crate”,
they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

A package can contain as many binary crates as you like, but at most only one library crate.
A package must contain at least one crate, whether that’s a library or binary crate.

*/

/// ### Modules Cheat Sheet
///
/// Here we provide a quick reference on how modules, paths, the `use` keyword, and
/// the `pub` keyword work in the compiler, and how most developers organize their
/// code. We’ll be going through examples of each of these rules throughout this
/// chapter, but this is a great place to refer to as a reminder of how modules
/// work.
///
/// - **Start from the crate root**: When compiling a crate, the compiler first
///   looks in the crate root file (usually *src/lib.rs* for a library crate or
///   *src/main.rs* for a binary crate) for code to compile.
/// - **Declaring modules**: In the crate root file, you can declare new modules;
/// say, you declare a “garden” module with `mod garden;`. The compiler will look
/// for the module’s code in these places:
///   - Inline, within curly brackets that replace the semicolon following `mod
///     garden`
///   - In the file *src/garden.rs*
///   - In the file *src/garden/mod.rs*
/// - **Declaring submodules**: In any file other than the crate root, you can
///   declare submodules. For example, you might declare `mod vegetables;` in
///   *src/garden.rs*. The compiler will look for the submodule’s code within the
///   directory named for the parent module in these places:
///   - Inline, directly following `mod vegetables`, within curly brackets instead
///     of the semicolon
///   - In the file *src/garden/vegetables.rs*
///   - In the file *src/garden/vegetables/mod.rs*
/// - **Paths to code in modules**: Once a module is part of your crate, you can
///   refer to code in that module from anywhere else in that same crate, as long
///   as the privacy rules allow, using the path to the code. For example, an
///   `Asparagus` type in the garden vegetables module would be found at
///   `crate::garden::vegetables::Asparagus`.
/// - **Private vs public**: Code within a module is private from its parent
///   modules by default. To make a module public, declare it with `pub mod`
///   instead of `mod`. To make items within a public module public as well, use
///   `pub` before their declarations.
/// - **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
///   items to reduce repetition of long paths. In any scope that can refer to
///   `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
///   crate::garden::vegetables::Asparagus;` and from then on you only need to
///   write `Asparagus` to make use of that type in the scope.
pub fn pkg_fn() {}
