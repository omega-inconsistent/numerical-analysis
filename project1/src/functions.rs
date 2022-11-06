use std::ops::{Sub};
use crate::matrix::Matrix;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

fn F1(p: Point) -> f64 {    // 5 * x^2 - y^2
    5.0 * p.x * p.x - p.y * p.y
}

fn F2(p: Point) -> f64 {    // 4 * y - (sin x + cos y)
    4.0 * p.y - (p.x.sin() + p.y.cos())
}

pub fn F(p : Point) -> Point {
    Point { x: F1(p), y: F2(p) }
}

fn F1_x(p: Point) -> f64 {  // 10x
    10.0 * p.x
}

fn F1_y(p: Point) -> f64 {  // -2y
    - 2.0 * p.y
}

fn F2_x(p: Point) -> f64 {  // -cos x
    - p.x.cos()
}

fn F2_y(p: Point) -> f64 { // 4 + sin y
    4.0 + p.y.sin()
}

pub fn J(p: Point) -> Matrix {  // calculate Jacobian matrix
    Matrix { 
        a_11: F1_x(p), 
        a_12: F1_y(p),
        a_21: F2_x(p), 
        a_22: F2_y(p) 
    }
}