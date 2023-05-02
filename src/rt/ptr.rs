//! Non-trivial pointer types.

pub trait Proxied {
  type View<'a>: ViewFor<'a, Self> + Copy;
  type Mut<'a>: MutFor<'a, Self>;
}

pub type View<'a, T> = <T as Proxied>::View<'a>;
pub type Mut<'a, T> = <T as Proxied>::Mut<'a>;

pub trait ViewFor<'a, T: Proxied + ?Sized> {
  fn as_view(&self) -> View<T>;
}

pub trait MutFor<'a, T: Proxied + ?Sized>: ViewFor<'a, T> {
  fn into_view(self) -> View<'a, T>;
  fn as_mut(&mut self) -> Mut<T>;
}
