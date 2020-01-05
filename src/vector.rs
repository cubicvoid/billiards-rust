use std::borrow::{Borrow, ToOwned};
use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, MulAssign};

use algebra::{Zero, One, SquaredNorm};

#[derive(Clone, PartialEq, Debug)]
pub struct V2<R>(pub R, pub R);

//pub struct v2<R>(pub &R, pub &R)

impl<R> V2<R>
where
    R: Zero
{
  pub fn from_real(r: R) -> V2<R> {
    V2(r, R::zero())
  }
}

impl<R> SquaredNorm for V2<R>
where
    R: SquaredNorm + ToOwned,
    R::Output: Add
{
  type Output = <<R as SquaredNorm>::Output as Add>::Output;
  fn squared_norm(&self) -> Self::Output {
    self.0.squared_norm() + self.1.squared_norm()
  }
}
/*
impl<'a, R> V2<R>
where
    R: SquaredNorm<Output=R> + Add<R, Output=R> + 'a
{
  pub fn squared_norm(&'a self) -> R {
    self.0.clone() * self.0. + self.1.clone() * &self.1
  }
}
*/

impl<'a, R> V2<R>
where
    R: Add<R, Output=R> + Mul<&'a R, Output=R> + ToOwned<Owned=R> + 'a
{
  pub fn dot(&self, v: &'a V2<R>) -> R {
    self.0.to_owned() * &v.0 + self.1.to_owned() * &v.1
  }
}

impl<R> V2<R>
where
    R: Neg<Output=R>
{
  pub fn complex_conjugate(self) -> V2<R> {
    V2(self.0, -self.1)
  }
}

impl<R> Zero for V2<R>
where R: Zero
{
  fn zero() -> V2<R> {
    V2(R::zero(), R::zero())
  }
}

impl<T> One for V2<T>
where
    T: Zero + One
{
  fn one() -> V2<T> {
    V2(T::one(), T::zero())
  }
}

impl<R> Add<V2<R>> for V2<R>
where
    R: Add<R, Output=R>
{
  type Output = V2<R>;

  fn add(self, v: V2<R>) -> V2<R> {
    V2(self.0 + v.0, self.1 + v.1)
  }
}

impl<'a, R> Add<&'a V2<R>> for V2<R>
where
    R: Add<&'a R, Output=R>
{
  type Output = V2<R>;

  fn add(self, v: &'a V2<R>) -> V2<R> {
    //V2(self.0 + &v.0, self.1 + &v.1)
    V2(self.0 + &v.0, self.1 + &v.1)
  }
}

impl<R> AddAssign<V2<R>> for V2<R>
where
    R: AddAssign<R> + Clone
{
  fn add_assign(&mut self, v: V2<R>) {
    self.0 += v.0;
    self.1 += v.1;
  }
}

impl<'a, R> AddAssign<&'a V2<R>> for V2<R>
where
    R: AddAssign<&'a R> + Clone
{
  fn add_assign(&mut self, v: &'a V2<R>) {
    self.0 += &v.0;
    self.1 += &v.1;
  }
}

impl<R> Sub<V2<R>> for V2<R>
where
    R: Add<Output=R> + Neg<Output=R> + Clone
{
  type Output = V2<R>;

  fn sub(self, v: V2<R>) -> V2<R> {
    V2(self.0 + (-v.0), self.1 + (-v.1))
  }
}

impl<'a, R> Sub<&'a V2<R>> for V2<R>
where
    R: Add<R, Output=R> + Neg<Output=R> + Clone
{
  type Output = V2<R>;

  fn sub(self, v: &'a V2<R>) -> V2<R> {
    V2(self.0 + (-v.0.clone()), self.1 + (-v.1.clone()))
  }
}

impl<R> Neg for V2<R>
where
    R: Neg<Output=R> + Clone
{
  type Output = V2<R>;

  fn neg(self) -> V2<R> {
    V2(self.0.neg(), self.1.neg())
  }
}


/*impl<R> Mul<R> for V2<R>
where
    R: Mul<Output=R> + Clone
{
  type Output = V2<R>;
  fn mul(self, r: R) -> V2<R> {
    V2(self.0 * r.clone(), self.1 * r)
  }
}*/

/*
impl<R> Div<R> for V2<R>
where
    R: Div<Output=R> + Clone
{
  type Output = V2<R>;
  fn div(self, r: R) -> V2<R> {
    V2(self.0 / r.clone(), self.1 / r)
  }
}*/


/*impl<'a, R, T> Mul<&'a T> for V2<R>
where
    T: Borrow<V2<R>>,
    R: Add<Output=R> + Mul<Output=R> + Neg<Output=R> + Clone + 'a
{
  type Output = V2<R>;
  fn mul(&self, v: &'a T) -> V2<R> {
    let v: &'a V2<R> = v.borrow();
    V2(
      self.0.clone() * v.0.clone() + (-self.1.clone() * v.1.clone()),
      self.0.clone() * v.1.clone() + self.1.clone() * v.0.clone()
    )
  }
}*/

impl<R> Mul<V2<R>> for V2<R>
where
    R: Add<Output=R> + Mul<Output=R> + Neg<Output=R> + Clone
{
  type Output = V2<R>;
  fn mul(self, v: V2<R>) -> V2<R> {
    V2(
      self.0.clone() * v.0.clone() + (-self.1.clone() * v.1.clone()),
      self.0.clone() * v.1.clone() + self.1.clone() * v.0.clone()
    )
  }
}

impl<R> Div<V2<R>> for V2<R>
where
    V2<R>: Mul<V2<R>> + SquaredNorm<Output=R>,
    R: Add<Output=R> + Mul<Output=R> + Div<Output=R> + Neg<Output=R> + Clone,
{
  type Output = <V2<R> as Mul>::Output;
  fn div(self, v: V2<R>) -> Self::Output {
    let conj = v.clone().complex_conjugate();
    let norm = v.squared_norm();
    let v_inv = V2(conj.0 / norm.clone(), conj.1 / norm);
    self * v_inv
  }

}


/*impl<'a, 'b, R, T> Mul<&'b T> for &'a V2<R>
where
    T: Borrow<V2<R>>,
    R: Add<Output=R> + Mul<Output=R> + Neg<Output=R> + Clone
{
  type Output = V2<R>;
  fn mul(&'a self, v: &'b V2<R>) -> V2<R> {
    let v: &'b V2<R> = v.borrow();
    V2(
      self.0.clone() * v.0.clone() + (-self.1.clone() * v.1.clone()),
      self.0.clone() * v.1.clone() + self.1.clone() * v.0.clone()
    )
  }
}*/

/*
impl<R> MulAssign<V2<R>> for V2<R>
where
    R: Ring
{
  fn mul_assign(&mut self, v: V2<R>) {
    *self = *self * v;
  }
}
*/

/*
impl<'a, R> MulAssign<&'a V2<R>> for V2<R>
where
    R: Mul<R, Output=R> + Add<R, Output=R> + Neg<Output=R> + Clone + 'a
{
  fn mul_assign(self, rhs: V2<R>) {
    self = rhs;
  }
}
*/
/*impl<'a, R, V> Mul<&'a V> for V2<R>
where
    V: AsRef<V2<R>> + 'a,
    R: Mul<&'a R, Output=R> + Add<&'a R, Output=R> + Neg<Output=R> + Clone + 'a
{
  type Output = V2<R>;

  fn mul(self, v: &'a V) -> V2<R> {
    let v = v.as_ref();
    V2(self.0 + &v.0, self.1 + &v.1)
  }
}*/


impl<R: Clone> From<(R,R)> for V2<R> {
  fn from(v: (R, R)) -> V2<R> {
    V2(v.0, v.1)
  }
}

impl<R> From<&(R,R)> for V2<R> where R: Clone {
  fn from(v: &(R, R)) -> V2<R> {
    V2(v.0.clone(), v.1.clone())
  }
}
