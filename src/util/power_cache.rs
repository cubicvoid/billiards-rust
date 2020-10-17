use std::ops::{Add, Sub, Neg, Mul, Div, MulAssign};
use rug::Rational;

use crate::vector::V2;
use crate::algebra::One;

pub struct PowerCache<T>
where
    T: Mul<Output=T> + One + Clone
{
  base: T,
  powers: Vec<T>
}

pub struct TurnCache<T>
{
  base: V2<T>,
  powers: Vec<Option<V2<T>>>
}

impl<T> PowerCache<T>
where
    T: Mul<Output=T> + One + Clone
{
  pub fn new(base: T) -> PowerCache<T> {
    let powers: Vec<T> = vec![T::one(), base.clone()];
    PowerCache{base, powers}
  }

 pub fn get(&mut self, degree: u32) -> &T {
    let degree = degree as usize;
    while self.powers.len() <= degree {
      let last: T = self.powers.last().unwrap().clone();
      let base: T = self.base.clone();
      self.powers.push(last * base);
    }
    &self.powers[degree]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_with_rational() {
    let mut cache = PowerCache::new(Rational::from((1, 2)));
    assert_eq!(
      *cache.get(2),
      Rational::from((1, 4)),
      "(1/2)^2 should equal 1/4"
    );
  }

  #[test]
  fn test_with_v2() {
    let base = V2(Rational::from((1, 2)), Rational::from((1, 2)));
    let mut cache = PowerCache::new(base);
    assert_eq!(
      *cache.get(2),
      V2(Rational::from(0), Rational::from((1, 2))),
      "(1/2 + i/2)^2 should equal i/2"
    );
  }
}