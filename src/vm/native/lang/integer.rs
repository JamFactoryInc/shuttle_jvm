use std::ops::{Deref, DerefMut};
use crate::vm::native::string::JString;
use crate::vm::nullability::Fallible;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct JInt(i32);

impl JInt {
    pub fn value_of__ljava_lang_string__i__out_i__throws_number_format_exception(str: JString, radix: JInt) -> Fallible<JInt> {
        Self::parse_int__ljava_lang_string__i__out_i__throws_number_format_exception(str, radix)
    }
    
    pub fn value_of__ljava_lang_string__out_i__throws_number_format_exception(str: JString) -> Fallible<JInt> {
        Self::parse_int__ljava_lang_string__i__out_i__throws_number_format_exception(str, JInt(10))
    }
    
    pub fn parse_int__ljava_lang_string__i__out_i__throws_number_format_exception(str: JString, radix: JInt) -> Fallible<JInt> {
        Fallible::Value(JInt(i32::from_str_radix(str.as_str(), radix.0 as u32)?))
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