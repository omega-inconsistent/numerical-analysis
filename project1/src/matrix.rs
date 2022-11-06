use std::ops::{Add, Mul};
use std::fmt::{Display, Formatter, Result};

use crate::functions::Point;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub a_11 : f64,
    pub a_12 : f64,
    pub a_21 : f64,
    pub a_22 : f64,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix { a_11: 0.0, a_12: 0.0, a_21: 0.0, a_22: 0.0 }
    }

    pub fn determinant(&self) -> f64 {
        self.a_11 * self.a_22 - self.a_12 * self.a_21
    }

    pub fn inverse(self) -> Option<Matrix> {
        let d = self.determinant();

        if d == 0.0 {
            None
        } else {
            Some(
             Matrix {
                    a_11 : self.a_22,
                    a_12 : -self.a_12,
                    a_21 : -self.a_21,
                    a_22 : self.a_11,
                } 
                * (1.0 / d)
            )
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}\n{}, {}\n", 
            self.a_11,
            self.a_12,
            self.a_21,
            self.a_22,
        )
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            a_11 : self.a_11 + rhs.a_11,
            a_12 : self.a_12 + rhs.a_12,
            a_21 : self.a_21 + rhs.a_21,
            a_22 : self.a_22 + rhs.a_21,
        }
    }
}

impl Mul for Matrix {   // implementation of matrix multiplication
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            a_11 : self.a_11 * rhs.a_11 + self.a_12 * rhs.a_21,
            a_12 : self.a_11 * rhs.a_12 + self.a_12 * rhs.a_22,
            a_21 : self.a_21 * rhs.a_11 + self.a_22 * rhs.a_21,
            a_22 : self.a_21 * rhs.a_12 + self.a_22 * rhs.a_22,
        }
    }
}

impl Mul<f64> for Matrix {  // implementation of scalar multiplication(require scalar on the right hand side)
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            a_11 : self.a_11 * rhs,
            a_12 : self.a_12 * rhs,
            a_21 : self.a_21 * rhs,
            a_22 : self.a_22 * rhs,
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x : self.a_11 * rhs.x + self.a_12 * rhs.y,
            y : self.a_21 * rhs.x + self.a_22 * rhs.y
        }
    }
}