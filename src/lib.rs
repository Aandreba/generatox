#![cfg_attr(feature = "nightly", feature(waker_getters))]
use corelib::ops::DerefMut;
use std::pin::Pin;

pub mod prelude {
    pub use crate::{generator, Generator};
}

pub use generatox_proc::generator;
#[doc(hidden)]
pub mod wrapper;
#[doc(hidden)]
pub extern crate core as corelib;

pub enum State<Y, R> {
    Yield(Y),
    Return(R),
}

pub trait Generator {
    type Yield;
    type Return;

    fn resume(self: Pin<&mut Self>) -> State<Self::Yield, Self::Return>;

    fn yields(self: Pin<&mut Self>) -> Yields<&mut Self> {
        return Yields { inner: self };
    }
}

#[non_exhaustive]
pub struct Yields<T> {
    pub inner: Pin<T>,
}

impl<G: Generator> From<Box<G>> for Yields<Box<G>> {
    fn from(value: Box<G>) -> Self {
        return Self {
            inner: Box::into_pin(value),
        };
    }
}

// impl<T: DerefMut<Target = G> + Unpin, G: Generator> From<T> for Yields<T> {
//     fn from(value: T) -> Self {
//         Pin::new(pointer)
//     }
// }

impl<T: DerefMut<Target = G>, G: Generator> Iterator for Yields<T> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.as_mut().resume() {
            State::Yield(x) => Some(x),
            State::Return(_) => None,
        }
    }
}