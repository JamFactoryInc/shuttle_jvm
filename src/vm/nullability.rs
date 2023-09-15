use std::convert::Infallible;
use std::error::Error;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut, FromResidual};

#[derive(Copy, Clone)]
pub enum Fallible<T> {
    Value(T),
    Exception(Exception),
}
// impl<T: Copy + Clone, Err: Error> FromResidual<Result<Infallible, Err>> for Fallible<T> {
//     fn from_residual(residual: Result<Infallible, Err>) -> Self {
//         match residual {
//             Ok(success) => Fallible::Exception(),
//             Err(err) =>
//         }
//     }
// }

#[derive(Copy, Clone)]
pub struct Exception(NonZeroU32);
impl Exception {
    pub const NULL_PTR: Exception = unsafe { Exception(NonZeroU32::new_unchecked(1)) };
    pub const NUMBER_FORMAT: Exception = unsafe { Exception(NonZeroU32::new_unchecked(2)) };
}

#[derive(Copy, Clone)]
pub enum Nullable<T> {
    Null,
    Present(T)
}
// impl<T: Copy + Clone> Deref for Nullable<T> {
//     type Target = Fallible<T>;
//
//     fn deref(&self) -> &Fallible<T> {
//         &match self {
//             Nullable::Null => Fallible::Exception(Exception::NULL_PTR),
//             Nullable::Present(value) => Fallible::Value(*value)
//         }
//     }
// }
//
// impl<T: Deref + Copy> DerefMut for Nullable<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut match self {
//             Nullable::Null => Fallible::Exception(Exception::NULL_PTR),
//             Nullable::Present(value) => Fallible::Value(*value)
//         }
//     }
// }
