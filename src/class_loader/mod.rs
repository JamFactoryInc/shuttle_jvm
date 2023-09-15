mod linker;
pub mod in_memory_class;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use cafebabe::{ClassFile, ParseError};

pub struct ClassLoader;

pub struct SucceededClassLoader<'a> {
    bytes: Vec<u8>,
    slice: &'a [u8],
    class: Option<ClassFile<'a>>
}
impl<'a> SucceededClassLoader<'a> {
    
    pub fn get_class(self) -> ClassFile<'a> {
        return self.class.unwrap()
    }
    
    pub fn parse_class<'b>(self) -> Result<SucceededClassLoader<'b>, FailedClassLoader>
        where 'a: 'b {
        let SucceededClassLoader { bytes, slice, .. } = self;
        Ok(SucceededClassLoader::<'b> {
            bytes,
            slice: &*slice,
            class: match cafebabe::parse_class(slice) {
                Ok(res) => {
                    Some(res)
                },
                Err(err) => return Err(FailedClassLoader { message: err.to_string() })
            },
        })
    }
}

pub struct FailedClassLoader<> {
    message: String
}
impl FailedClassLoader {
    pub fn get_msg(&self) -> &str {
        self.message.as_str()
    }
}
impl<'a> Display for FailedClassLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_msg())
    }
}

impl ClassLoader {
    pub fn parse(bytes: &[u8]) -> Result<ClassFile, ParseError> {
        cafebabe::parse_class(bytes)
    }
    
    pub fn read(dest: &str) -> Result<Vec<u8>, Error> {
        let path = Path::new(dest);
        
        let file = File::open(path);
        
        Ok(
            file?.bytes()
                .map(Result::unwrap)
                .collect::<Vec<u8>>()
        )
    }
}