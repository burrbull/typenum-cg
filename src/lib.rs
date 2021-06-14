#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

mod marker_traits;
pub use marker_traits::*;
mod type_operators;
pub use type_operators::*;
mod uint;
pub use uint::*;

use core::cmp::Ordering;

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Greater`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Greater;

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Less`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Less;

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Equal`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Equal;

/// Returns `core::cmp::Ordering::Greater`
impl Ord for Greater {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Greater
    }
}

/// Returns `core::cmp::Ordering::Less`
impl Ord for Less {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Less
    }
}

/// Returns `core::cmp::Ordering::Equal`
impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal
    }
}

pub type Diff<A, B> = <A as core::ops::Sub<B>>::Output;
pub type Sum<A, B> = <A as core::ops::Add<B>>::Output;

#[derive(Clone, Copy, Default)]
pub struct Bool<const B: bool>;
pub type True = Bool<true>;
pub type False = Bool<false>;

pub trait TrueExt {}
impl TrueExt for True {}

pub trait FalseExt {}
impl FalseExt for False {}

impl<const B: bool> Bit for Bool<B> {
    const U8: u8 = B as u8;
    const BOOL: bool = B;

    fn new() -> Self {
        Bool
    }
    fn to_u8() -> u8 {
        B as u8
    }
    fn to_bool() -> bool {
        B
    }
}
