use std::ops::{Add, Sub, Neg, Mul, Div};
use rug::Rational;
use vector::V2;

use algebra::{Zero, One};
use billiards::{Params, ParamsTrait};
use singularity::{Singularity::{self, S0, S1}, Orientation, Pair};

pub struct BaseEdge<'a, R>
where
    R: ParamsTrait
{
  params: &'a mut Params<R>,
  coords: Pair<V2<R>>,
  // the vector from coords[S0] to coords[S1]
  //offset: V2<R>,
  orientation: Orientation,
}

impl<'b, 'a: 'b, K> BaseEdge<'a, K>
where
    K: ParamsTrait
{
  pub fn new(
      params: &'a mut Params<K>,
      coords: Pair<V2<K>>,
      orientation: Orientation) -> BaseEdge<'a, K> {
    BaseEdge{params, coords, orientation}
  }

  /// initialize a `BaseEdge` with the default starting state
  /// on the unit interval (0,0) -> (1,0).
  pub fn new_default(params: &'a mut Params<K>) -> BaseEdge<'a, K> {
    let origin = V2(K::zero(), K::zero());
    let one = V2(K::one(), K::zero());
    Self::new(params, Pair(origin, one), Orientation::Forward)
  }

  pub fn offset(&self) -> V2<K> {
    self.to_coords() - self.from_coords()
  }

  pub fn step(&'b mut self, turn: i32) {
    let degree = turn.abs() as u32;
    let turn_vec = self.params.turn_vec(self.to(), turn);
    let new_offset: V2<K> = turn_vec * (-self.offset());
    
    let new_to_coords: V2<K> = self.to_coords() + new_offset;
    self.orientation = self.orientation.reversed();
    let to = self.to();
    self.coords[to] = new_to_coords;
  }

  pub fn left_apex(&self) -> V2<K> {
    let mut apex = self.params.apex().clone();
    if self.orientation.from() == S1 {
      apex = apex.complex_conjugate();
    }
    let offset = self.coords[S1].clone() - &self.coords[S0];
    self.coords[S0].clone() + apex * offset
  }

  pub fn right_apex(&self) -> V2<K> {
    let mut apex = self.params.apex().clone();
    if self.orientation.from() == S0 {
      apex = apex.complex_conjugate();
    }
    let offset = self.coords[S1].clone() - &self.coords[S0];
    self.coords[S0].clone() + apex * offset
  }

  pub fn from(&self) -> Singularity {
    self.orientation.from()
  }

  pub fn from_coords(&self) -> V2<K> {
    self.coords[self.orientation.from()].clone()
  }

  pub fn to(&self) -> Singularity {
    self.orientation.to()
  }

  pub fn to_coords(&self) -> V2<K> {
    self.coords[self.orientation.to()].clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_base_edge() {
    let mut params = Params::new(V2(Rational::from((1, 2)), Rational::from((1, 2))));
    let mut edge = BaseEdge::new_default(&mut params);

    assert_eq!(edge.orientation, Orientation::Forward);
    assert_eq!(edge.from(), S0, "initial edge should point from S0");
    assert_eq!(edge.to(), S1, "initial edge should point to S1");
    assert_eq!(
      edge.offset(),
      V2(Rational::from(1), Rational::from(0)));

    assert_eq!(
      edge.params.turn_vec(S1, -1),
      V2(Rational::from(0), Rational::from(-1)),
      ""
    );

    edge.step(-1);


    assert_eq!(
      edge.offset(),
      V2(Rational::from(0), Rational::from(1)),
      "offset should be (0, 1)"
    );

    assert_eq!(
      edge.from_coords(),
      V2(Rational::from(1), Rational::from(0)),
      "from_coords should be (1, 0)"
    );

    assert_eq!(
      edge.to_coords(),
      V2(Rational::from(1), Rational::from(1)),
      "to_coords should be (1, 1)"
    );

    assert_eq!(
      edge.left_apex(),
      V2(Rational::from((1, 2)), Rational::from((1, 2))),
      "left apex should be (1/2, 1/2)"
    );

    assert_eq!(
      edge.right_apex(),
      V2(Rational::from((3, 2)), Rational::from((1, 2))),
      "right apex should be (3/2, 1/2)"
    );

  }
}