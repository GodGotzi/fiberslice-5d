use std::ops::AddAssign;

use three_d::Vector3;

use super::{FlipYZ, Reverse};

pub trait DirectMul {
    fn direct_mul(&self, other: &Self) -> Self;
}

impl<S: std::ops::Mul<S, Output = S> + Copy> DirectMul for Vector3<S> {
    fn direct_mul(&self, other: &Self) -> Self {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl FlipYZ for Vector3<f32> {
    fn flip(&mut self) {
        std::mem::swap(&mut self.y, &mut self.z);
    }
}

impl FlipYZ for (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    fn flip(&mut self) {
        self.0.flip();
        self.1.flip();
        self.2.flip();
    }
}

impl Reverse for (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    fn reverse(&mut self) {
        std::mem::swap(&mut self.0, &mut self.2);
    }
}

#[derive(Debug)]
pub struct Average<T: std::ops::Div<f32>> {
    pub value: Option<T>,
    pub count: usize,
}

impl<T: std::ops::Div<f32>> Default for Average<T> {
    fn default() -> Self {
        Self {
            value: None,
            count: 0,
        }
    }
}

impl<T: std::ops::Div<f32, Output = T>> Average<T> {
    pub fn divide_average(self) -> Option<T> {
        if let Some(value) = self.value {
            Some(value / (self.count as f32))
        } else {
            None
        }
    }
}

impl<T: std::ops::Div<f32, Output = T> + AddAssign> AddAssign for Average<T> {
    fn add_assign(&mut self, other: Self) {
        if let Some(average) = other.divide_average() {
            self.add(average);
        }
    }
}

impl<T: std::ops::Div<f32> + AddAssign> Average<T> {
    pub fn add(&mut self, value: T) {
        if let Some(current) = self.value.as_mut() {
            *current += value;
        } else {
            self.value = Some(value);
        }

        self.count += 1;
    }
}
