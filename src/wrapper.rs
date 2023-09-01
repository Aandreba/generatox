use crate::{Generator, State};
use core::ptr::null;
use core::task::{RawWaker, RawWakerVTable, Waker};
use pin_project::pin_project;
use std::cell::Cell;
use std::future::Future;
use std::marker::PhantomData;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::ptr::addr_of_mut;
use std::task::{Context, Poll};

#[cfg(not(feature = "nightly"))]
thread_local! {
    static CURRENT_CELL: Cell<*const ()> = Cell::new(core::ptr::null());
}

#[repr(transparent)]
#[pin_project]
pub struct Wrapper<Fut, Y> {
    #[pin]
    pub fut: Fut,
    phantom: Ghostly<Y>,
}

impl<Fut, Y> Wrapper<Fut, Y> {
    pub const fn new(fut: Fut) -> Self {
        return Self {
            fut,
            phantom: Ghostly(PhantomData),
        };
    }
}

impl<Fut: Future, Y> Generator for Wrapper<Fut, Y> {
    type Yield = Y;
    type Return = Fut::Output;

    fn resume(self: std::pin::Pin<&mut Self>) -> crate::State<Self::Yield, Self::Return> {
        let this = self.project();
        let mut cell: Option<Y> = None;

        let mut context: Context;
        cfg_if::cfg_if! {
            if #[cfg(feature = "nightly")] {
                let waker = unsafe {
                    Waker::from_raw(RawWaker::new(
                        addr_of_mut!(cell) as *mut Option<Y> as *const (),
                        &NOOP_WAKER_VTABLE,
                    ))
                };
                context = Context::from_waker(&waker);
            } else {
                CURRENT_CELL.with(|x| x.set(addr_of_mut!(cell) as *mut Option<Y> as *const ()));
                context = Context::from_waker(noop_waker_ref());
            }
        }

        return match this.fut.poll(&mut context) {
            std::task::Poll::Ready(ret) => State::Return(ret),
            std::task::Poll::Pending => unsafe { State::Yield(cell.take().unwrap_unchecked()) },
        };
    }
}

pub fn r#yield<T>(value: T) -> PendingOnce<T> {
    return PendingOnce(Some(value));
}

#[repr(transparent)]
#[pin_project]
pub struct PendingOnce<T>(Option<T>);

impl<T> Future for PendingOnce<T> {
    type Output = ();

    fn poll(
        self: Pin<&mut Self>,
        #[allow(unused_variables)] cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.project();
        match this.0.take() {
            Some(x) => unsafe {
                let cell: *const ();
                cfg_if::cfg_if! {
                    if #[cfg(feature = "nightly")] {
                        cell = cx.waker().as_raw().data();
                    } else {
                        cell = CURRENT_CELL.with(|cell| cell.get());
                    }
                };

                let cell = &mut *(cell as *mut Option<T>);
                *cell = Some(x);
                Poll::Pending
            },
            None => Poll::Ready(()),
        }
    }
}

const NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);

unsafe fn noop(_data: *const ()) {}

unsafe fn noop_clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &NOOP_WAKER_VTABLE)
}

const fn noop_raw_waker() -> RawWaker {
    RawWaker::new(null(), &NOOP_WAKER_VTABLE)
}

#[inline]
pub fn noop_waker() -> Waker {
    // FIXME: Since 1.46.0 we can use transmute in consts, allowing this function to be const.
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

#[inline]
pub fn noop_waker_ref() -> &'static Waker {
    struct SyncRawWaker(RawWaker);
    unsafe impl Sync for SyncRawWaker {}

    static NOOP_WAKER_INSTANCE: SyncRawWaker = SyncRawWaker(noop_raw_waker());

    // SAFETY: `Waker` is #[repr(transparent)] over its `RawWaker`.
    unsafe { &*(&NOOP_WAKER_INSTANCE.0 as *const RawWaker as *const Waker) }
}

#[derive(Default)]
pub struct Ghostly<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> RefUnwindSafe for Ghostly<T> {}
unsafe impl<T: ?Sized> Send for Ghostly<T> {}
unsafe impl<T: ?Sized> Sync for Ghostly<T> {}
impl<T: ?Sized> Unpin for Ghostly<T> {}
impl<T: ?Sized> UnwindSafe for Ghostly<T> {}
