pub enum PixelType{
    RGBA,
}

pub struct ImageBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize, imageType: PixelType ) -> Self{
        ImageBuffer {
            width,
            height,
            buffer: vec![0; width * height * 4],
        }
    }

    pub fn get_pixel(&self, rth: usize, cth: usize) -> [u8; 4] {
        let start = ((rth - 1) * self.width + (cth - 1)) * 4;
        let buffer = &self.buffer;
        [buffer[start], buffer[start + 1], buffer[start + 2], buffer[start + 3]]
    }
    pub fn get_pixel_enumerate_position(&self, rth: usize, cth: usize) -> [u8; 4] {
        let start = (rth * self.width + cth) * 4;
        let buffer = &self.buffer;
        [buffer[start], buffer[start + 1], buffer[start + 2], buffer[start + 3]]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::{ImageBuffer, PixelType};
        let buffer = ImageBuffer::new(800, 600, PixelType::RGBA);
        assert_eq!(buffer.get_pixel(599, 799), [0, 0, 0, 0]);
        assert_eq!(buffer.get_pixel(600, 800), [0, 0, 0, 0]);
    }
}
