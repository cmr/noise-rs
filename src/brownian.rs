// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::Float;

use {math, Seed};

pub fn brownian2<T, F>(seed: &Seed, point: &::Point2<T>, noise_func: F, wavelength: T, octaves: u32) -> T
    where T: Float, F: Fn(&Seed, &::Point2<T>) -> T
{
    let mut frequency: T = wavelength.recip();
    let mut amplitude: T = Float::one();
    let mut result: T = Float::zero();
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * frequency,
                            point[1] * frequency];
        result = result + (noise_func(seed, &scaled_point) * amplitude);
        amplitude = amplitude * math::cast(0.5f32);
        frequency = frequency * math::cast(2.0f32);
    }
    result
}

pub fn brownian3<T, F>(seed: &Seed, point: &::Point3<T>, noise_func: F, wavelength: T, octaves: u32) -> T
    where T: Float, F: Fn(&Seed, &::Point3<T>) -> T
{
    let mut frequency: T = wavelength.recip();
    let mut amplitude: T = Float::one();
    let mut result: T = Float::zero();
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * frequency,
                            point[1] * frequency,
                            point[2] * frequency];
        result = result + (noise_func(seed, &scaled_point) * amplitude);
        amplitude = amplitude * math::cast(0.5f32);
        frequency = frequency * math::cast(2.0f32);
    }
    result
}

pub fn brownian4<T, F>(seed: &Seed, point: &::Point4<T>, noise_func: F, wavelength: T, octaves: u32) -> T
    where T: Float, F: Fn(&Seed, &::Point4<T>) -> T
{
    let mut frequency: T = wavelength.recip();
    let mut amplitude: T = Float::one();
    let mut result: T = Float::zero();
    for _ in range(0, octaves) {
        let scaled_point = [point[0] * frequency,
                            point[1] * frequency,
                            point[2] * frequency,
                            point[3] * frequency];
        result = result + (noise_func(seed, &scaled_point) * amplitude);
        amplitude = amplitude * math::cast(0.5f32);
        frequency = frequency * math::cast(2.0f32);
    }
    result
}
