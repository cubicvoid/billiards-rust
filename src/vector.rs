
use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, MulAssign};

#[derive(Clone, PartialEq)]
pub struct V2<R: Clone>(pub R, pub R);

impl<'a, R> V2<R>
where
    R: Mul<Output=R> + Add<Output=R> + Clone + 'a
{
  pub fn squared_norm(&'a self) -> R {
    self.0.clone() * self.0.clone() + self.1.clone() * self.1.clone()
  }

  pub fn dot(&self, v: &V2<R>) -> R {
    self.0.clone() * v.0.clone() + self.1.clone() * v.1.clone()
  }
}

impl<R> V2<R>
where
    R: Neg<Output=R> + Clone
{
  pub fn complex_conjugate(self) -> V2<R> {
    V2(self.0, -self.1)
  }
}

impl<R> Add<V2<R>> for V2<R>
where
    R: Add<R, Output=R> + Clone
{
  type Output = V2<R>;

  fn add(self, v: V2<R>) -> V2<R> {
    V2(self.0 + v.0, self.1 + v.1)
  }
}

impl<'a, R> Add<&'a V2<R>> for V2<R>
where
    R: Add<&'a R, Output=R> + Clone + 'a
{
  type Output = V2<R>;

  fn add(self, v: &'a V2<R>) -> V2<R> {
    //V2(self.0 + &v.0, self.1 + &v.1)
    V2(self.0 + &v.0, self.1 + &v.1)
  }
}

impl<'a, R> AddAssign<&'a V2<R>> for V2<R>
where
    R: AddAssign<&'a R> + Clone + 'a
{
  fn add_assign(&mut self, v: &'a V2<R>) {
    self.0 += &v.0;
    self.1 += &v.1;
  }
}

impl<R> Sub<V2<R>> for V2<R>
where
    R: Add<R, Output=R> + Neg<Output=R> + Clone
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

impl<R> Neg for V2<R> where R: Neg<Output=R> + Clone {
  type Output = V2<R>;

  fn neg(self) -> V2<R> {
    V2(self.0.neg(), self.1.neg())
  }
}

impl<R> Mul<R> for V2<R>
where
    R: Mul<R, Output=R> + Clone
{
  type Output = V2<R>;
  fn mul(self, r: R) -> V2<R> {
    V2(self.0 * r.clone(), self.1 * r)
  }
}

impl<R> Div<R> for V2<R>
where
    R: Div<Output=R> + Clone
{
  type Output = V2<R>;
  fn div(self, r: R) -> V2<R> {
    V2(self.0 / r.clone(), self.1 / r)
  }
}

impl<'a, R> Mul<V2<R>> for V2<R>
where
    R: Mul<R, Output=R> + Add<R, Output=R> + Neg<Output=R> + Clone
{
  type Output = V2<R>;
  fn mul(self, v: V2<R>) -> V2<R> {
    V2(
      self.0.clone() * v.0.clone() + (-self.1.clone() * v.1.clone()),
      self.0 * v.1 + self.1 * v.0
    )
  }
}

impl<'a, R> Mul<&'a V2<R>> for V2<R>
where
    R: Mul<R, Output=R> + Add<R, Output=R> + Neg<Output=R> + Clone + 'a
{
  type Output = V2<R>;
  fn mul(self, v: &'a V2<R>) -> V2<R> {
    V2(
      self.0.clone() * v.0.clone() + (-self.1.clone() * v.1.clone()),
      self.0 * v.1.clone() + self.1 * v.0.clone()
    )
  }
}
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
