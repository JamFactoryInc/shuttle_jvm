use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use crate::vm::native::string::JString;
use crate::vm::nullability::{Exception, Fallible, Nullable};

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct JInt(i32);

impl JInt {

    //pub fn to_string(i: JInt, radix: JInt) ->

    pub fn value_of(i: JInt) -> Nullable<JInt> {

    }

    pub fn parse_int_radix(str: Nullable<JString>, radix: JInt) -> Fallible<JInt> {
        if let Nullable::Present(str) = str {
            match i32::from_str_radix(str.as_str(), radix.0 as u32) {
                Ok(ok) => Fallible::Value(JInt(ok)),
                Err(_) => Fallible::Exception(Exception::NUMBER_FORMAT)
            }
        } else {
            Fallible::Exception(Exception::NUMBER_FORMAT)
        }
    }

    pub fn parse_int(str: Nullable<JString>) -> Fallible<JInt> {
        if let Nullable::Present(str) = str {
            match i32::from_str(str.as_str()) {
                Ok(ok) => Fallible::Value(JInt(ok)),
                Err(_) => Fallible::Exception(Exception::NUMBER_FORMAT)
            }
        } else {
            Fallible::Exception(Exception::NUMBER_FORMAT)
        }

    }
}

impl Deref for JInt {
    type Target = i32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}