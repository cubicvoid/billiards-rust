use std::ops::{Add, Sub, Neg, Mul, Div};
use rug::Rational;
use vector::V2;
use singularity::{Singularity::{self, S0, S1}, Orientation, Pair};
use algebra::Ring;
use billiards::Params;

pub struct BaseEdge<'a, R>
where
    R: Ring
{
  params: &'a mut Params<R>,
  coords: Pair<V2<R>>,
  // the vector from coords[S0] to coords[S1]
  offset: V2<R>,
  orientation: Orientation,
}

impl<'b, 'a: 'b, K> BaseEdge<'a, K>
where
    K: Ring + 'a
{
  pub fn new(
      params: &'a mut Params<K>,
      coords: Pair<V2<K>>,
      orientation: Orientation) -> BaseEdge<'a, K> {
    let offset = coords[S1].clone() - &coords[S0];
    BaseEdge{params, coords, offset, orientation}
  }

  pub fn step(&'b mut self, turn: i32) {
    let degree = turn.abs() as u32;
    let turn_vec = self.params.turn_vec(self.orientation.to(), turn);
    let new_offset: V2<K> = turn_vec * self.offset.clone();
    let from_coords: V2<K> = self.coords[self.orientation.to()].clone();
    let new_coords: V2<K> = from_coords + self.offset.clone();
    self.offset = new_offset;
    self.coords[self.orientation.from()] = new_coords;
    self.orientation = self.orientation.reversed();
  }

  pub fn left_apex(&self) -> V2<K> {
    let mut apex = self.params.apex().clone();
    if self.orientation.from() == S1 {
      apex = apex.complex_conjugate();
    }
    apex * &self.offset
  }

  pub fn right_apex(&self) -> V2<K> {
    let mut apex = self.params.apex().clone();
    if self.orientation.from() == S0 {
      apex = apex.complex_conjugate();
    }
    apex * &self.offset
  }

  pub fn from(&self) -> Singularity {
    self.orientation.from()
  }

  pub fn from_coords(&self) -> V2<K> {
    self.coords[self.orientation.from()].clone()
  }
}

