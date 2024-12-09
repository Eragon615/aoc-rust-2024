use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(isize, isize);

impl Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        return Point(self.0 + rhs.0, self.1 + rhs.1);
    }
    
    type Output = Point;
}

impl Mul for Point {
    fn mul(self, rhs: Self) -> Self::Output {
        return Point(self.0 * rhs.0, self.1 * rhs.1);
    }

    type Output = Point;
}

impl Sub for Point {
    fn sub(self, rhs: Self) -> Self::Output {
        return Point(self.0 - rhs.0, self.1 - rhs.1);
    }

    type Output = Point;
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point(x, y)
    }

    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}