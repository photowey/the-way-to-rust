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

/// `ternary_operator`: A macro for ternary operation([wiki](https://en.wikipedia.org/wiki/Ternary_conditional_operator)) in rust.
///
/// # Examples:
///
///```rust
/// use macros::ternary_operator;
///
/// let result = ternary_operator!(true, "yes", "no");
/// assert_eq!(result, "yes");
/// ```
#[macro_export]
macro_rules! ternary_operator {
    ($condition:expr, $value_true:expr, $value_false:expr) => {
        if $condition {
            $value_true
        } else {
            $value_false
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ternary_operator() {
        let seed = 10;

        let is_even = ternary_operator!(seed % 2 == 0, true, false);
        assert!(is_even);

        let result = ternary_operator!(seed > 5, "greater than 5", "less than or equal to 5");
        assert_eq!("greater than 5", result);
    }
}
