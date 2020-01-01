use std::ops::{Add, Sub, Neg, Mul, Div};

use rug::Rational;

use vector::V2;


pub trait Ring: Zero + One + Neg<Output=Self> + Clone {
    
}

impl<T> Ring for T
where
    T: Zero + One + Neg<Output=T> + Clone
{

}

pub trait Zero: Add<Output=Self> + Sized {
  fn zero() -> Self;
}

impl Zero for Rational {
  fn zero() -> Rational { Rational::from(0) }
}

impl Zero for f64 {
  fn zero() -> f64 { 0.0 }
}

impl<T> Zero for V2<T>
where T: Zero + Clone
{
  fn zero() -> V2<T> {
    V2(T::zero(), T::zero())
  }
}

pub trait One: Mul<Output=Self> + Sized {
  fn one() -> Self;
}

impl One for Rational {
  fn one() -> Rational { Rational::from(1) }
}

impl One for f64 {
  fn one() -> f64 { 1.0 }
}

impl<T> One for V2<T>
where
    T: Zero + One + Neg + Clone,
    V2<T>: Mul<Output=Self>,
{
  fn one() -> V2<T> {
    V2(T::one(), T::zero())
  }
}