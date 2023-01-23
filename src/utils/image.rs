use crate::utils::pixel::Pixel;

pub struct Image {
    pub height: u32,
    pub width: u32,
    pub pixels: Vec<Pixel<u8>>,
}

impl Image { // Assumes top left corner is (0,0)
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel<u8> {
        let offset = y * self.width + x;
        return self.pixels[offset as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel<u8>) {
        let offset = y * self.width + x;
        self.pixels[offset as usize] = pixel;
    }


}
