use crate::{Bit, Bool, UInt};

/// A **type operator** that returns `True` if `Self < Rhs`, otherwise returns `False`.
pub trait IsLess<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_less(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsLess<UInt<RN>> for UInt<N>
where
    Bool<{ N < RN }>: Sized,
{
    type Output = Bool<{ N < RN }>;
    fn is_less(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}

/// A **type operator** that returns `True` if `Self == Rhs`, otherwise returns `False`.
pub trait IsEqual<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_equal(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsEqual<UInt<RN>> for UInt<N>
where
    Bool<{ N == RN }>: Sized,
{
    type Output = Bool<{ N == RN }>;
    fn is_equal(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}

/// A **type operator** that returns `True` if `Self > Rhs`, otherwise returns `False`.
pub trait IsGreater<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_greater(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsGreater<UInt<RN>> for UInt<N>
where
    Bool<{ N > RN }>: Sized,
{
    type Output = Bool<{ N > RN }>;
    fn is_greater(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}

/// A **type operator** that returns `True` if `Self <= Rhs`, otherwise returns `False`.
pub trait IsLessOrEqual<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_less_or_equal(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsLessOrEqual<UInt<RN>> for UInt<N>
where
    Bool<{ N <= RN }>: Sized,
{
    type Output = Bool<{ N <= RN }>;
    fn is_less_or_equal(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}

/// A **type operator** that returns `True` if `Self != Rhs`, otherwise returns `False`.
pub trait IsNotEqual<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_not_equal(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsNotEqual<UInt<RN>> for UInt<N>
where
    Bool<{ N != RN }>: Sized,
{
    type Output = Bool<{ N != RN }>;
    fn is_not_equal(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}

/// A **type operator** that returns `True` if `Self >= Rhs`, otherwise returns `False`.
pub trait IsGreaterOrEqual<Rhs = Self> {
    /// The type representing either `True` or `False`
    type Output: Bit;
    /// Method returning `True` or `False`.
    fn is_greater_or_equal(self, rhs: Rhs) -> Self::Output;
}

impl<const N: usize, const RN: usize> IsGreaterOrEqual<UInt<RN>> for UInt<N>
where
    Bool<{ N >= RN }>: Sized,
{
    type Output = Bool<{ N >= RN }>;
    fn is_greater_or_equal(self, _rhs: UInt<RN>) -> Self::Output {
        Bool
    }
}
