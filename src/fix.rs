use std::{fmt::{Display, LowerHex}, num::Wrapping, ops::{Add, AddAssign, Div, Mul, MulAssign, Shl, Shr, Sub, SubAssign}};

#[macro_export]
macro_rules! fx {
    ($x:expr) => {$crate::fix::Fix::new($x)};
}

#[macro_export]
macro_rules! fr {
    ($x:expr) => {$crate::fix::Fix::new_raw($x)};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fix(Wrapping<i32>);

impl Fix {
    const PRECISION: i32 = 12;

    pub fn new(val: f64) -> Self {
        Self(Wrapping((val * (1 << Self::PRECISION) as f64 + 0.5).floor() as i32))
    }

    pub const fn new_raw(val: i32) -> Self {
        Self(Wrapping(val))
    }

    pub fn min(self, rhs: Self) -> Self {
        Fix(self.0.min(rhs.0))
    }

    pub fn max(self, rhs: Self) -> Self {
        Fix(self.0.max(rhs.0))
    }

    pub fn val(self) -> i32 {
        self.0.0
    }
}

impl AddAssign for Fix {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add for Fix {
    type Output = Fix;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Fix {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sub for Fix {
    type Output = Fix;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl MulAssign for Fix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul for Fix {
    type Output = Fix;

    fn mul(self, rhs: Self) -> Self::Output {
        Fix(Wrapping(((self.0.0 as i64).wrapping_mul(rhs.0.0 as i64).wrapping_add(1 << (Self::PRECISION - 1))).div_euclid(1 << Self::PRECISION) as i32))
    }
}

impl Mul<i32> for Fix {
    type Output = Fix;

    fn mul(self, rhs: i32) -> Self::Output {
        Fix(self.0 * Wrapping(rhs))
    }
}

impl Mul<Fix> for i32 {
    type Output = Fix;

    fn mul(self, rhs: Fix) -> Self::Output {
        rhs * self
    }
}

impl Div<i32> for Fix {
    type Output = Fix;

    fn div(self, rhs: i32) -> Self::Output {
        Fix(self.0 / Wrapping(rhs))
    }
}

impl Shl<usize> for Fix {
    type Output = Fix;

    fn shl(self, rhs: usize) -> Self::Output {
        Fix(self.0 << rhs)
    }
}

impl Shr<usize> for Fix {
    type Output = Fix;

    fn shr(self, rhs: usize) -> Self::Output {
        Fix(self.0 >> rhs)
    }
}

impl Display for Fix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&(self.0.0 as f64 / (1 << Self::PRECISION) as f64), f)
    }
}

impl LowerHex for Fix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&(self.0.0 as u32), f)
    }
}