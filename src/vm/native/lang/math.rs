use std::intrinsics::roundf32;

pub struct Math;
impl Math {
    pub fn round() {
        roundf32()
    }
}