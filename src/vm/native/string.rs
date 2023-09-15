
pub struct JString {

}
impl JString {
    pub fn as_str(&self) -> &str {
        todo!()
    }
}

pub fn hash(str: &str) -> i32 {
    let mut total = 0i32;
    let mut poly = 1i32;
    for byte in str.bytes().rev() {
        total += byte as i32 * poly;
        poly *= 31;
    }
    total
}