use std::{any, borrow::Borrow};

use crate::{
    res::{Res, ResMut},
    World,
};

pub trait BorrowWorld<'w> {
    unsafe fn borrow(w: &'w World) -> Self;
}

impl<'w, T: 'static> BorrowWorld<'w> for Res<'w, T> {
    unsafe fn borrow(w: &'w World) -> Self {
        w.res.borrow().unwrap_or_else(|| {
            panic!(
                "Tried to borrow resource of type {} for a system",
                any::type_name::<T>()
            )
        })
    }
}

impl<'w, T: 'static> BorrowWorld<'w> for ResMut<'w, T> {
    unsafe fn borrow(w: &'w World) -> Self {
        w.res.borrow_mut().unwrap_or_else(|| {
            panic!(
                "Tried to borrow resource of type {} for a system",
                any::type_name::<T>()
            )
        })
    }
}

pub unsafe trait System<'w, Param> {
    unsafe fn run(&mut self, w: &'w World);
}

macro_rules! impl_run {
    ($($xs:ident),+ $(,)?) => {
        unsafe impl<'w, $($xs), +, F> System<'w, ($($xs,) +)> for F
        where
            F: FnMut($($xs), +),
            $($xs: BorrowWorld<'w>), +
        {
            unsafe fn run(&mut self, w: &'w World) {
                (self)(
                    $(<$xs as BorrowWorld>::borrow(w),)+
                )
            }
        }
    };
}

macro_rules! recursive {
    ($macro:ident, $first:ident) => {
        $macro!($first);
    };
    ($macro:ident, $first:ident, $($rest:ident),* $(,)?) => {
        $macro!($first, $($rest), *);
        recursive!($macro, $($rest), *);
    };
}

recursive!(impl_run, P15, P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0,);
