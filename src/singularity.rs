use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug)]
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
  /*pub fn new(a: T, b: T) -> Pair<T> {
    Pair(a, b)
  }*/

  pub fn with_value(&self, value: Value<T>) -> Pair<T> {
    match value.s {
      Singularity::S0 => { Pair(value.v, self.1.clone()) },
      Singularity::S1 => { Pair(self.0.clone(), value.v) },
    }
  } 

  /*pub fn map<U, V, F>(self, f: F) -> Pair<V>
  where
      T: Borrow<U>,
      F: Clone + Fn(U) -> V {
    return Pair(f(&self.0), f(&self.1))
  }*/

  /*pub fn map<F>(&self, f: F) -> Pair<F>
  where F: Clone + Fn(&T) -> F {
    return Pair(f(&self.0), f(&self.1))
  }*/
}

trait Collection {
  type Element;
}

impl<T> Collection for Pair<T> {
  type Element = T;
}

trait Map<U, V>: Collection
{
  type Output: Collection<Element=V>;

  fn map<F>(self, f: F) -> Self::Output where F: Fn(U) -> V;
}

/*impl<U, V> Map<U, V> for Pair<U> {
  type Output = Pair<V>;

  fn map<F>(self, f: F) -> Self::Output
      where F: Fn(U) -> V
  {
    Pair(f(self.0), f(self.1))
  }
}*/

impl<'a, T, U, V> Map<&'a U, V> for Pair<T>
where
    T: Borrow<&'a U> + 'a,
    U: 'a,
{
  type Output = Pair<V>;

  fn map<F>(self, f: F) -> Self::Output
      where F: Fn(&'a U) -> V
  {
    Pair(f(self.0.borrow()), f(self.1.borrow()))
  }
}

/*impl<'a, T> Map<U> for &'a Pair<T> {
  type Output = Pair<U>;
  fn map<F>(self, f: F) -> Pair<U>
  where
}*/

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

#[derive(PartialEq, Debug)]
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
