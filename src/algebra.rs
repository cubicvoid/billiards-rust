use std::ops::{Add, AddAssign, Sub, Neg, Mul, MulAssign, Div};

use rug::Rational;

use vector::V2;

/// required invariants:
///   `zero() + zero() == zero()` (identity)
///   `a + (-a) == zero()` (inverse)
pub trait Zero
{
  fn zero() -> Self;
}

impl Zero for Rational {
  fn zero() -> Rational { Rational::from(0) }
}

impl Zero for f64 {
  fn zero() -> f64 { 0.0 }
}

/// required invariant:
///   `one() * one() == one()` (identity)
pub trait One {
  fn one() -> Self;
}

impl One for Rational {
  fn one() -> Rational { Rational::from(1) }
}

impl One for f64 {
  fn one() -> f64 { 1.0 }
}

pub trait SquaredNorm {
  type Output;
  fn squared_norm(&self) -> Self::Output;
}

impl SquaredNorm for Rational {
  type Output = Rational;
  fn squared_norm(&self) -> Rational {
    Rational::from(self * self)
  }
}

impl SquaredNorm for f64 {
  type Output = f64;
  fn squared_norm(&self) -> f64 {
    self * self
  }
}