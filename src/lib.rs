#[allow(dead_code)]
pub struct Checked<T>(Option<T>);

impl<T> Checked<T> {
    /// Calls an infix function on each of the inner values of two `Checked<T>`s
    pub fn fallible_inner_infix<F>(self, rhs: Self, f: F) -> Option<T>
    where
        F: Fn(T, T) -> Option<T>,
    {
        if let (Some(x), Some(y)) = (self.0, rhs.0) {
            f(x, y)
        } else {
            None
        }
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

impl<T> std::ops::Add for Checked<T>
where
    T: num::CheckedAdd,
{
    type Output = Option<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_add(&b))
    }
}

impl<T> std::ops::Sub for Checked<T>
where
    T: num::CheckedSub,
{
    type Output = Option<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_sub(&b))
    }
}

impl<T> std::ops::Mul for Checked<T>
where
    T: num::CheckedMul,
{
    type Output = Option<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_mul(&b))
    }
}

impl<T> std::ops::Div for Checked<T>
where
    T: num::CheckedDiv,
{
    type Output = Option<T>;

    fn div(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_div(&b))
    }
}

impl<T> std::ops::Rem for Checked<T>
where
    T: num::traits::CheckedRem,
{
    type Output = Option<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_rem(&b))
    }
}
