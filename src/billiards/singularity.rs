use std::borrow::Borrow;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug)]
pub enum BaseSingularity {
	B0,
	B1,
}

impl BaseSingularity {
	/*fn Vals<T>(v0: T, v1: T) {

	}*/
}

#[derive(Debug)]
pub struct BaseValues<T>(pub T, pub T);

#[derive(PartialEq, Debug)]
pub enum ApexSingularity {
	A0,
	A1,
}

// Singularity represents the four distinct singularities induced by the
// vertices of the fundamental quadrilateral. For obtuse triangular billiards,
// we label the base vertices B0 and B1, and the apex A0, then reflect through
// the base to produce the conjugate apex A1. Thus the oriented (widdershins)
// quadrilateral vertices are B0-A1-B1-A0, while the original triangle is
// B0-B1-A0.
#[derive(PartialEq, Debug)]
pub enum Singularity {
	B0,
	B1,
	A0,
	A1,
}

impl From<BaseSingularity> for Singularity {
	fn from(s: BaseSingularity) -> Singularity {
		match s {
			BaseSingularity::B0 => Singularity::B0,
			BaseSingularity::B1 => Singularity::B1,
		}
	}
}

impl From<ApexSingularity> for Singularity {
	fn from(s: ApexSingularity) -> Singularity {
		match s {
			ApexSingularity::A0 => Singularity::A0,
			ApexSingularity::A1 => Singularity::A1,
		}
	}
}

//#[derive(Debug)]
//pub struct S2<T>(pub T, pub T);

/*impl<T> Pair<T> {
  pub fn from_fn<F>(f: F) -> Pair<T>
  where
      F: Fn(Singularity) -> T
  {
    Pair(
      f(Singularity::S0), f(Singularity::S1)
    )
  }
}*/

/*impl<T> S2<T> 
where T: Clone
{

  pub fn with_value(&self, value: Value<T>) -> S2<T> {
    match value.s {
      Singularity::S0 => { S2(value.v, self.1.clone()) },
      Singularity::S1 => { S2(self.0.clone(), value.v) },
    }
  } 

  pub fn map<U, F>(&self, f: F) -> S2<U>
  where
      F: Fn(&T) -> U
  {
    S2(f(&self.0), f(&self.1))
  }*/

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
//}
/*
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
}*/



/*impl<U, V> Map<U, V> for Pair<U> {
  type Output = Pair<V>;

  fn map<F>(self, f: F) -> Self::Output
      where F: Fn(U) -> V
  {
    Pair(f(self.0), f(self.1))
  }
}*/

/*impl<'a, T, U, V> Map<&'a U, V> for Pair<T>
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
*/
/*impl<'a, T> Map<U> for &'a Pair<T> {
  type Output = Pair<U>;
  fn map<F>(self, f: F) -> Pair<U>
  where
}*/

impl<T> Index<BaseSingularity> for BaseValues<T> {
  type Output = T;
  fn index(&self, s: BaseSingularity) -> &T {
    match s {
      BaseSingularity::B0 => { &self.0 },
      BaseSingularity::B1 => { &self.1 }
    }
  }
}

impl<T> IndexMut<BaseSingularity> for BaseValues<T> {
  fn index_mut(&mut self, s: BaseSingularity) -> &mut T {
    match s {
      BaseSingularity::B0 => { &mut self.0 },
      BaseSingularity::B1 => { &mut self.1 }
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum BaseOrientation {
  // "forward" means from S0 to S1
  Forward,
  Backward
}

impl BaseOrientation {
  pub fn to(&self) -> BaseSingularity {
    match self {
      BaseOrientation::Forward => BaseSingularity::B1,
      BaseOrientation::Backward => BaseSingularity::B0,
    }
  }

  pub fn from(&self) -> BaseSingularity {
    match self {
      BaseOrientation::Forward => BaseSingularity::B0,
      BaseOrientation::Backward => BaseSingularity::B1,
    }
  }

  pub fn reversed(&self) -> BaseOrientation {
    match self {
      BaseOrientation::Forward => BaseOrientation::Backward,
      BaseOrientation::Backward => BaseOrientation::Forward,
    }
  }
}
