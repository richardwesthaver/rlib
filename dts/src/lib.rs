#![feature(generic_associated_types)]
//#![feature(arbitrary_enum_discriminant)]
#![allow(incomplete_features, dead_code)]

pub trait Functor {
  type Unwrapped;
  type Wrapped<B>: Functor;

  fn map<F, B>(self, f: F) -> Self::Wrapped<B>
  where
    F: FnMut(Self::Unwrapped) -> B;
}

pub trait Pointed: Functor {
  fn wrap<T>(t: T) -> Self::Wrapped<T>;
}

pub trait Applicative: Pointed {
  fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
  where
    F: FnMut(Self::Unwrapped, B) -> C;
}

pub trait Monad: Applicative {
  fn bind<B, F>(self, f: F) -> Self::Wrapped<B>
  where
    F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>;
}

pub trait MonadTrans {
  type Base: Monad;

  fn lift(base: Self::Base) -> Self;
}

struct IdentityT<M>(M);

impl<M: Functor> Functor for IdentityT<M> {
  type Unwrapped = M::Unwrapped;
  type Wrapped<A> = IdentityT<M::Wrapped<A>>;

  fn map<F, B>(self, f: F) -> Self::Wrapped<B>
  where
    F: FnMut(M::Unwrapped) -> B,
  {
    IdentityT(self.0.map(f))
  }
}

impl<M: Pointed> Pointed for IdentityT<M> {
  fn wrap<T>(t: T) -> IdentityT<M::Wrapped<T>> {
    IdentityT(M::wrap(t))
  }
}

impl<M: Applicative> Applicative for IdentityT<M> {
  fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
  where
    F: FnMut(Self::Unwrapped, B) -> C,
  {
    IdentityT(self.0.lift_a2(b.0, f))
  }
}

impl<M: Monad> Monad for IdentityT<M> {
  fn bind<B, F>(self, mut f: F) -> Self::Wrapped<B>
  where
    F: FnMut(Self::Unwrapped) -> Self::Wrapped<B>,
  {
    IdentityT(self.0.bind(|x| f(x).0))
  }
}

impl<M: Monad> MonadTrans for IdentityT<M> {
  type Base = M;

  fn lift(base: M) -> Self {
    IdentityT(base)
  }
}

struct Unlimited;

trait Limit {
  fn within_limit(self, n: usize) -> bool;
}

impl Limit for usize {
  fn within_limit(self, n: usize) -> bool {
    n < self
  }
}

impl Limit for Unlimited {
  fn within_limit(self, _: usize) -> bool {
    true
  }
}

#[derive(Debug, PartialEq)]
pub enum Opt<A> {
  Some(A),
  None,
}

impl<A> Opt<A> {
  pub fn map<F: FnOnce(A) -> B, B>(self, f: F) -> Opt<B> {
    match self {
      Opt::Some(a) => Opt::Some(f(a)),
      Opt::None => Opt::None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Res<A, E> {
  Ok(A),
  Err(E),
}

impl<A, E> Res<A, E> {
  pub fn map<F: FnOnce(A) -> B, B>(self, f: F) -> Res<B, E> {
    match self {
      Res::Ok(a) => Res::Ok(f(a)),
      Res::Err(e) => Res::Err(e),
    }
  }
}

#[derive(PartialEq, Debug)]
enum Validation<A, E> {
  Ok(A),
  Err(E),
}

impl<A> Functor for Option<A> {
  type Unwrapped = A;
  type Wrapped<B> = Option<B>;

  fn map<F: FnMut(A) -> B, B>(self, mut f: F) -> Option<B> {
    match self {
      Some(x) => Some(f(x)),
      None => None,
    }
  }
}

impl<A> Pointed for Option<A> {
  fn wrap<T>(t: T) -> Option<T> {
    Some(t)
  }
}

impl<A> Applicative for Option<A> {
  fn lift_a2<F, B, C>(self, b: Self::Wrapped<B>, mut f: F) -> Self::Wrapped<C>
  where
    F: FnMut(Self::Unwrapped, B) -> C,
  {
    let a = self?;
    let b = b?;
    Some(f(a, b))
  }
}

impl<A> Monad for Option<A> {
  fn bind<B, F>(self, f: F) -> Option<B>
  where
    F: FnMut(A) -> Option<B>,
  {
    self.and_then(f)
  }
}

#[cfg(test)]
mod tests;
