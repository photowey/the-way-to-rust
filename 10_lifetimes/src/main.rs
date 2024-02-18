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

mod lifetimes;

// Lifetimes are another kind of generic that we’ve already been using.
// Rather than ensuring that a type has the behavior we want,
// lifetimes ensure that references are valid as long as we need them to be.

fn main() {
    lifetimes();
}

fn lifetimes() {
    lifetimes::lifetimes();
}
