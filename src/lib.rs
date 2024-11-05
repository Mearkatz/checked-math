use std::ops::{Add, Div, Mul, Rem, Sub};

use num::{traits::CheckedRem, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

#[allow(dead_code)]
/// A wrapper type where all operations that would normally be unchecked are checked.
/// # Examples
/// ```
/// # use checked_math::Checked;
/// assert_eq!(Checked::from(255u8) + Checked::from(1u8), Checked(None));
/// assert_eq!(Checked::from(255u8) - Checked::from(1u8), Checked(Some(254u8)));
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Checked<T>(pub Option<T>);

impl<T> Checked<T> {
    /// Calls an infix function on each of the inner values of two `Checked<T>`s        
    fn infix<F, U>(self, rhs: Self, f: F) -> Checked<U>
    where
        F: Fn(T, T) -> Option<U>,
    {
        match (self.0, rhs.0) {
            (Some(x), Some(y)) => Checked(f(x, y)),
            _ => Checked(None),
        }
    }
}

impl<T> From<T> for Checked<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}
impl<T> From<Option<T>> for Checked<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> PartialEq for Checked<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Checked<T> where T: PartialEq {}

impl<T> PartialOrd for Checked<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Add for Checked<T>
where
    T: CheckedAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.infix(rhs, |a: T, b: T| a.checked_add(&b))
    }
}

impl<T> Sub for Checked<T>
where
    T: CheckedSub,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.infix(rhs, |a: T, b: T| a.checked_sub(&b))
    }
}

impl<T> Mul for Checked<T>
where
    T: CheckedMul,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.infix(rhs, |a: T, b: T| a.checked_mul(&b))
    }
}

impl<T> Div for Checked<T>
where
    T: CheckedDiv,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.infix(rhs, |a: T, b: T| a.checked_div(&b))
    }
}

impl<T> Rem for Checked<T>
where
    T: CheckedRem,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.infix(rhs, |a: T, b: T| a.checked_rem(&b))
    }
}
