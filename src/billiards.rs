pub mod base_edge;

use std::ops::{Add, Sub, Neg, Mul, Div};

use rug::Rational;

use util::power_cache::PowerCache;
use vector::V2;
use algebra::{Zero, One, SquaredNorm};
use singularity::{Singularity::{self, S0, S1}, Orientation, Pair};

pub trait ParamsTrait:
    SquaredNorm<Output=Self> +
    Add<Output=Self> + Neg<Output=Self> +
    Mul<Output=Self> + Div<Output=Self> +
    Zero + One +
    Clone
{
}

impl<T> ParamsTrait for T
where
    T:
      SquaredNorm<Output=Self> +
      Add<Output=Self> + Neg<Output=Self> +
      Mul<Output=Self> + Div<Output=Self> +
      Zero + One +
      Clone
{

}

/// cached computed invariants for an explicit apex parameter over a ring `R`.
pub struct Params<K>
where
    K: ParamsTrait
{
  _apex: V2<K>,
  _rotations: Pair<PowerCache<V2<K>>>,
  _max_turns: Pair<u32>,
}

impl<K> Params<K>
where
    K: ParamsTrait
{
  pub fn new(apex: V2<K>) -> Params<K> {
    let origin = V2(K::zero(), K::zero());
    let one = V2(K::one(), K::zero());
  
    let bases = Pair(origin.clone(), one.clone());
    
    //let left_base = origin.clone();
    let left_edge = apex.clone();
    //let right_base = one.clone();
    let right_edge = bases[S1].clone() - apex.clone();
    let right_norm = right_edge.squared_norm();
    let left_norm = apex.squared_norm();
    let left_factor = left_edge.clone() * left_edge / V2::from_real(left_norm);
    let right_factor = (right_edge.clone() * right_edge / V2::from_real(right_norm)).complex_conjugate();

    let left_powers = PowerCache::new(left_factor);
    let right_powers = PowerCache::new(right_factor);
    //for 
    Params{
      _apex: apex,
      _rotations: Pair(left_powers, right_powers),
      _max_turns: Pair(5, 5),
    }
  }

  pub fn apex(&self) -> &V2<K> {
    &self._apex
  }

  pub fn turn_vec(&mut self, around: Singularity, by: i32) -> V2<K> {
    if by < 0 {
      self._rotations[around].get((-by) as u32).clone().complex_conjugate()
    } else {
      self._rotations[around].get(by as u32).clone()
    }
  }
}

