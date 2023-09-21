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

#![feature(test)]

extern crate test;

use kdtree::distance::squared_euclidean;


#[inline(always)]
pub fn justify_ascii(byte: u8) -> usize {
    (byte as usize % 96 % 64) * 2
}

#[inline(always)]
pub fn to_coord(str: &str) -> ([f32; 128], &str) {
    let mut coords = [0f32; 128];
    let bytes = str.bytes();
    let len = bytes.len() as f32;
    for (i, byte) in bytes.enumerate() {
        let index = justify_ascii(byte) * 2;
        coords[index.clone()] += 1.0;
        coords[index.clone() + 1] += (i + 1) as f32;
    }
    for i in (0..128).step_by(2) {
        if coords[i] != 0.0 {
            coords[i.clone() + 1] = len.clone() - coords[i.clone() + 1].clone() / coords[i.clone()].clone()
        }
    }
    (coords, str)
}

pub fn to_coord_lookup(str: &str) -> ([f32; 128], &str) {
    let mut coords = [0f32; 128];
    let bytes = str.bytes();
    let len = bytes.len() as f32;
    for (i, byte) in bytes.enumerate() {
        let index = justify_ascii(byte) * 2;
        coords[index.clone()] += 1.0;
        //coords[index.clone() + 1] += (i + 1) as f32;
    }
    // for i in (0..128).step_by(2) {
    //     if coords[i] != 0.0 {
    //         coords[i.clone() + 1] = coords[i.clone() + 1].clone() / coords[i.clone()].clone()
    //     }
    // }
    (coords, str)
}

#[allow(soft_unstable)]
#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use kdtree::KdTree;
    use super::*;

    #[test]
    fn it_works2() {

        let a: ([f32; 128], &str) = to_coord("acbd");
        let b: ([f32; 128], &str) = to_coord("dcba");
        let c: ([f32; 128], &str) = to_coord("abcc");
        let d: ([f32; 128], &str) = to_coord("abbd");
        let e: ([f32; 128], &str) = to_coord("abc");
        let f: ([f32; 128], &str) = to_coord("abcd");

        let dimensions = 128;
        let mut kdtree = KdTree::new(dimensions);

        kdtree.add(&a.0, a.1).unwrap();
        kdtree.add(&b.0, b.1).unwrap();
        kdtree.add(&c.0, c.1).unwrap();
        kdtree.add(&d.0, d.1).unwrap();
        kdtree.add(&e.0, e.1).unwrap();
        kdtree.add(&f.0, f.1).unwrap();

        let search_word = to_coord_lookup("abcd").0;
        let res = kdtree.nearest(&search_word, 7, &squared_euclidean).unwrap();
        println!("abcd {:?}", res);
    }

    #[bench]
    fn it_works(bencher: &mut Bencher) {

        let a: ([f32; 128], &str) = to_coord("somlongword");
        let b: ([f32; 128], &str) = to_coord("someongword");
        let c: ([f32; 128], &str) = to_coord("drowgnolemos");

        let dimensions = 128;
        let mut kdtree = KdTree::new(dimensions);

        kdtree.add(&a.0, a.1).unwrap();
        kdtree.add(&b.0, b.1).unwrap();
        kdtree.add(&c.0, c.1).unwrap();

        for i in 0..3000 {
            kdtree.add(&[0f32; 128], &"somelongord"[0..(i % 10 + 1)]).unwrap();
        }

        let search_word = to_coord("somelongword").0;
        bencher.iter(|| {
            black_box({
                let res = black_box(kdtree.nearest(&search_word, 2, &squared_euclidean).unwrap());
                //println!("mat {:?}", res);
            });
        })
    }
}
