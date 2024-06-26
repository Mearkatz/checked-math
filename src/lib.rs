#[allow(dead_code)]
/// A wrapper type where all operations that would normally be unchecked are checked.
/// # Examples
/// ```rust
/// use checked_math::Checked;
/// let x: Checked<u8> = 1u8.into();
/// let y: Checked<u8> = 1u8.into();
/// let z: Checked<u8> = 255u8.into();
///
/// assert_eq!((x + y + z), Checked::new(None)); // Overflow prevented :)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Checked<T> {
    pub data: Option<T>,
}

impl<T> Checked<T> {
    /// Constructs a new `Checked` with some initial value
    #[must_use]
    pub const fn new(data: Option<T>) -> Self {
        Self { data }
    }

    /// Calls an infix function on each of the inner values of two `Checked<T>`s
    #[must_use]
    pub fn fallible_inner_infix<F>(self, rhs: Self, f: F) -> Self
    where
        F: Fn(T, T) -> Option<T>,
    {
        if let (Some(x), Some(y)) = (self.data, rhs.data) {
            Self::new(f(x, y))
        } else {
            Self::new(None)
        }
    }
}

impl<T> From<T> for Checked<T> {
    fn from(value: T) -> Self {
        Self::new(Some(value))
    }
}

impl<T> PartialEq for Checked<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Eq for Checked<T> where T: PartialEq {}

impl<T> PartialOrd for Checked<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T> std::ops::Add for Checked<T>
where
    T: num::CheckedAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_add(&b))
    }
}

impl<T> std::ops::Sub for Checked<T>
where
    T: num::CheckedSub,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_sub(&b))
    }
}

impl<T> std::ops::Mul for Checked<T>
where
    T: num::CheckedMul,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_mul(&b))
    }
}

impl<T> std::ops::Div for Checked<T>
where
    T: num::CheckedDiv,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_div(&b))
    }
}

impl<T> std::ops::Rem for Checked<T>
where
    T: num::traits::CheckedRem,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.fallible_inner_infix(rhs, |a: T, b: T| a.checked_rem(&b))
    }
}
