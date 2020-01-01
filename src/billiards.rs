pub mod base_edge;

use std::ops::{Add, Sub, Neg, Mul, Div};

use rug::Rational;

use vector::V2;
use algebra::Ring;
use singularity::{Singularity::{self, S0, S1}, Orientation, Pair};
use self::power_cache::PowerCache;

/// cached computed invariants for an explicit apex parameter over a field `K`.
pub struct Params<R>
where
    R: Ring
{
  _apex: V2<R>,
  _rotations: Pair<PowerCache<V2<R>>>,
}

impl<K> Params<K>
where
    K: Ring + Div<Output=K>
{
  pub fn new(apex: V2<K>) -> Params<K> {
    let origin = V2(K::zero(), K::zero());
    let one = V2(K::one(), K::zero());
  
    let left_base = origin.clone();
    let left_edge = apex.clone();
    let left_norm = apex.squared_norm();
    let right_base = one.clone();
    let right_edge = right_base.clone() - apex.clone();
    let right_norm = right_edge.squared_norm();

    let left_powers = PowerCache::new(left_edge.clone() * left_edge / left_norm);
    let right_powers = PowerCache::new(right_edge.clone() * right_edge / right_norm);
    Params{
      _apex: apex,
      _rotations: Pair(left_powers, right_powers),
    }
  }
}

impl<R> Params<R> where R: Ring {
  pub fn apex(&self) -> &V2<R> {
    &self._apex
  }

  pub fn turn_vec(&mut self, around: Singularity, by: i32) -> V2<R> {
    if by < 0 {
      self._rotations[around].get((-by) as u32).clone().complex_conjugate()
    } else {
      self._rotations[around].get(by as u32).clone()
    }
  }
}

mod power_cache {
  use std::ops::{Add, Sub, Neg, Mul, Div};
  use rug::Rational;
  use vector::V2;
  use algebra::{Zero, One};

  pub struct PowerCache<T>
  where
      T: One + Clone
  {
    base: T,
    powers: Vec<T>
  }

  impl<T> PowerCache<T>
  where
      T: One + Clone
  {
    pub fn new(base: T) -> PowerCache<T> {
      let powers = vec![T::one(), base.clone()];
      PowerCache{base, powers}
    }

    pub fn get(&mut self, degree: u32) -> &T {
      let degree = degree as usize;
      while self.powers.len() <= degree {
        self.powers.push(self.powers.last().unwrap().clone() * self.base.clone());
      }
      &self.powers[degree]
    }
  }
}
