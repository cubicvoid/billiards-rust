use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
pub enum Singularity {
  S0,
  S1,
}

// map from disjoint union -> T
pub struct Value<T>{
  s: Singularity,
  v: T
}

pub struct Pair<T>(pub T, pub T);

impl<T> Pair<T> 
where T: Clone
{
  pub fn new(a: T, b: T) -> Pair<T> {
    Pair(a, b)
  }

  pub fn with_value(&self, value: Value<T>) -> Pair<T> {
    match value.s {
      Singularity::S0 => { Pair(value.v, self.1.clone()) },
      Singularity::S1 => { Pair(self.0.clone(), value.v) },
    }
  } 
}

impl<T> Index<Singularity> for Pair<T> {
  type Output = T;
  fn index(&self, s: Singularity) -> &T {
    match s {
      Singularity::S0 => { &self.0 },
      Singularity::S1 => { &self.1 }
    }
  }
}

impl<T> IndexMut<Singularity> for Pair<T> {
  fn index_mut(&mut self, s: Singularity) -> &mut T {
    match s {
      Singularity::S0 => { &mut self.0 },
      Singularity::S1 => { &mut self.1 }
    }
  }
}

#[derive(PartialEq)]
pub enum Orientation {
  // "forward" means from S0 to S1
  Forward,
  Backward
}

impl Orientation {
  pub fn to(&self) -> Singularity {
    match self {
      Orientation::Forward => Singularity::S1,
      Orientation::Backward => Singularity::S0,
    }
  }

  pub fn from(&self) -> Singularity {
    match self {
      Orientation::Forward => Singularity::S0,
      Orientation::Backward => Singularity::S1,
    }
  }

  pub fn reversed(&self) -> Orientation {
    match self {
      Orientation::Forward => Orientation::Backward,
      Orientation::Backward => Orientation::Forward,
    }
  }
}
