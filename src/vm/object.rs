use std::mem::{size_of, transmute};


#[derive(Copy, Clone)]
pub struct ClassType(u32);
impl ClassType {
    const ID_MASK: u32 = u32::MAX >> 1;
    
    #[inline]
    pub fn get_id(self) -> u32 {
        self.0 & Self::ID_MASK
    }
    
    #[inline]
    pub fn is_wide(self) -> bool {
        self.0 > Self::ID_MASK
    }
    
    #[inline]
    pub fn hijack(value: u8) {
    
    }
    
    #[inline]
    pub fn new(id: u32, is_wide: bool) -> ClassType {
        ClassType(((is_wide as u32) << 31) | id)
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ByteCodeType {
    Unknown = 0,
    Float = 1,
    Int = 2,
    Short = 3,
    Byte = 4,
    Char = 5,
    Ref = 6,
}
impl From<u64> for ByteCodeType {
    #[inline]
    fn from(value: u64) -> Self {
        Self::from((value >> (64 - 8)) as u8)
    }
}
impl From<u16> for ByteCodeType {
    #[inline]
    fn from(value: u16) -> Self {
        unsafe {
            transmute(value as u8)
        }
    }
}
impl From<u8> for ByteCodeType {
    #[inline]
    fn from(value: u8) -> Self {
        unsafe {
            transmute(value)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pointer {
    metadata: ClassType,
    pointer_1: u16,
    pointer_2: u16,
    pointer_3: u16,
}

#[derive(Copy, Clone)]
pub union PointerMetadata {
    some: u16
}

#[derive(Copy, Clone)]
pub struct SingleWide {
    type_info: ClassType,
    value: u32,
}
impl From<SingleWide> for f32 {
    fn from(value: SingleWide) -> Self {
        unsafe { transmute(value.value) }
    }
}
impl From<SingleWide> for i32 {
    fn from(value: SingleWide) -> Self {
        unsafe { transmute(value.value) }
    }
}
impl From<SingleWide> for i16 {
    fn from(value: SingleWide) -> Self {
        unsafe { transmute(value.value as i16) }
    }
}
impl From<SingleWide> for i8 {
    fn from(value: SingleWide) -> Self {
        unsafe { transmute(value.value as i8) }
    }
}

#[derive(Copy, Clone)]
pub struct FatPointer {
    class_id: u16,
    pointer: Pointer,
}

#[derive(Copy, Clone)]
pub struct DoubleWide {
    value: u64
}

pub union JavaValue {
    fat_pointer: FatPointer,
    pair: (SingleWide, SingleWide),
}

pub trait JavaObject {
    fn hash_code(&self) -> i32;
    fn to_string(&self) -> String;
    fn get_ptr(&self) -> u32;
    
}

#[test]
pub fn test_size() {
    println!("{}", size_of::<PointerMetadata>())
}