use core::fmt;

#[derive(Copy, Clone)]
pub struct Pixel<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl fmt::Display for Pixel<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r:{}, g:{}, b:{}", self.r, self.g, self.b)
    }
}
