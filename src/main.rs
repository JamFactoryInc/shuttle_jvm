#![feature(core_intrinsics)]
#![feature(try_trait_v2)]

extern crate core;

use crate::class_loader::ClassLoader;

pub mod class_loader;
mod vm;

fn main() {
    let bytes = ClassLoader::read(r#"D:\Codin\Java\shuttle_jvm_testing\out\production\shuttle_jvm_testing\Main.class"#).unwrap();
    let class = ClassLoader::parse(bytes.as_slice());
    
    println!("{class:?}");
    
    match &class.unwrap().methods.get(1).unwrap().attributes.first().unwrap().data {
        cafebabe::attributes::AttributeData::Code(a) => {
            for op in &a.bytecode.as_ref().unwrap().opcodes {
                println!("{op:?}")
            }
        },
        _ => (),
    }
}
