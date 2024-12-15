pub mod template;

use std::ops::{Add, Mul, Rem};

// Use this file to add helper functions and additional modules.
pub fn vectranspose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Rem<Point> for Point {
    type Output = Self;

    fn rem(self, rhs: Point) -> Self {
        Self {
            x: self.x.rem_euclid(rhs.x) % rhs.x,
            y: self.y.rem_euclid(rhs.y) % rhs.y,
        }
    }
}
