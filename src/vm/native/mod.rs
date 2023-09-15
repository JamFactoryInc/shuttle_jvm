use std::collections::HashMap;
use once_cell::unsync::Lazy;
use crate::vm::native::lang::integer::JInt;

pub mod math;
pub mod lang;
pub mod util;
pub mod string;

static mut STATIC_OVERRIDES: Lazy<HashMap<&str, usize>> = Lazy::new(HashMap::new) ;
static mut METHOD_OVERRIDES: Lazy<HashMap<&str, usize>> = Lazy::new(HashMap::new);

pub struct NativeOverrides;
impl NativeOverrides {
    pub fn init() {
        unsafe {
            STATIC_OVERRIDES.insert("java/lang/Integer.valueOf:(Ljava/lang/String;)Ljava/lang/Integer;", JInt::parse_int as usize);
            STATIC_OVERRIDES.insert("java/lang/Integer.valueOf:(Ljava/lang/String;I)Ljava/lang/Integer;", JInt::parse_int_radix as usize);
            STATIC_OVERRIDES.insert("java/lang/Integer.parseInt:(Ljava/lang/String;)Ljava/lang/Integer;", JInt::parse_int as usize);
            STATIC_OVERRIDES.insert("java/lang/Integer.parseInt:(Ljava/lang/String;I)Ljava/lang/Integer;", JInt::parse_int_radix as usize);
            METHOD_OVERRIDES.insert("", 0);
        };
    }
}