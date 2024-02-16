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

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<T, U>(self, other: MixedPoint<T, U>) -> MixedPoint<T, U> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// ----------------------------------------------------------------

pub fn struct_generics() {
    println!("----------------------------------------------------------------:generics:struct");
    struct_generics_v1();
    struct_generics_v2();
    struct_generics_v3();

    struct_generics_mixup();
    println!("----------------------------------------------------------------:generics:struct");
}

fn struct_generics_v1() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

fn struct_generics_v2() {
    let both_integer = MixedPoint { x: 5, y: 10 };
    let both_float = MixedPoint { x: 1.0, y: 4.0 };
    let integer_and_float = MixedPoint { x: 5, y: 4.0 };
}

fn struct_generics_v3() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

fn struct_generics_mixup() {
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
