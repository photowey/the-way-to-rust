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

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
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
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
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
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
}

fn struct_generics_v2() {
    let _both_integer = MixedPoint { x: 5, y: 10 };
    let _both_float = MixedPoint { x: 1.0, y: 4.0 };
    let _integer_and_float = MixedPoint { x: 5, y: 4.0 };
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
