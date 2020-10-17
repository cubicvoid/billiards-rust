pub mod base_edge;
pub mod embedding;
pub mod homotopy;
pub mod singularity;
pub mod turn_path;

use std::cmp::Ord;
use std::ops::{Add, Sub, Neg, Mul, Div};

use rug::Rational;

use crate::util::power_cache::PowerCache;
use crate::vector::V2;
use crate::algebra::{Zero, One, SquaredNorm};
use singularity::{BaseSingularity::{self, B0, B1}, BaseOrientation, BaseValues};

pub trait ParamsTrait:
    Ord +
    SquaredNorm<Output=Self> +
    Add<Output=Self> + Neg<Output=Self> +
    Mul<Output=Self> + Div<Output=Self> +
    Zero + One +
    Clone +
    std::fmt::Debug
{
}

impl<T> ParamsTrait for T
where
    T:
      Ord +
      SquaredNorm<Output=Self> +
      Add<Output=Self> + Neg<Output=Self> +
      Mul<Output=Self> + Div<Output=Self> +
      Zero + One +
      Clone +
      std::fmt::Debug
{

}

/// cached computed invariants for an explicit apex parameter over a ring `R`.
pub struct Params<K>
where
    K: ParamsTrait
{
  _apex: V2<K>,
  _rotations: BaseValues<PowerCache<V2<K>>>,
  _max_turns: BaseValues<u32>,
}

impl<K> Params<K>
where
    K: ParamsTrait
{
  pub fn new(apex: V2<K>) -> Params<K> {
    let origin = V2(K::zero(), K::zero());
    let one = V2(K::one(), K::zero());
  
    let bases = BaseValues(origin.clone(), one.clone());
    
    //let left_base = origin.clone();
    let left_edge = apex.clone();
    //let right_base = one.clone();
    let right_edge = bases[B1].clone() - apex.clone();
    let right_norm = right_edge.squared_norm();
    let left_norm = apex.squared_norm();
    let left_factor = left_edge.clone() * left_edge / V2::from_real(left_norm);
    let right_factor = (right_edge.clone() * right_edge / V2::from_real(right_norm)).complex_conjugate();

    let left_powers = PowerCache::new(left_factor);
    let right_powers = PowerCache::new(right_factor);
    let mut rotations = BaseValues(left_powers, right_powers);
    println!("apex: {:#?}", apex);
    let max_turns = BaseValues(
      Self::_max_turn(&mut rotations[B0]),
      Self::_max_turn(&mut rotations[B1]));
    println!("max_turns: {:#?}", max_turns);
    Params{
      _apex: apex,
      _rotations: rotations,
      _max_turns: max_turns,
    }
  }

  fn _max_turn(z: &mut PowerCache<V2<K>>) -> u32 {
    let mut turn = 1;
    let mut next_turn = turn + 1;
    while z.get(next_turn).1 > K::zero() {
      turn = next_turn;
      next_turn += 1;
    }
    turn
  }

  pub fn max_turn_around(&self, s: BaseSingularity) -> u32 {
    self._max_turns[s]
  }

  pub fn apex(&self) -> &V2<K> {
    &self._apex
  }

  pub fn turn_vec(&mut self, around: BaseSingularity, by: i32) -> V2<K> {
    if by < 0 {
      self._rotations[around].get((-by) as u32).clone().complex_conjugate()
    } else {
      self._rotations[around].get(by as u32).clone()
    }
  }
}

