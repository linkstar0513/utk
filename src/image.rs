pub enum PixelType{
    RGBA,
}

pub trait Canvas{
    fn draw_line(&mut self, start: (usize, usize), end: (usize, usize));
}

pub struct ImageBuffer {
    width: usize,
    height: usize,
    pixel_type: PixelType,
    buffer: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize, pixel_type: PixelType ) -> Self{
        ImageBuffer {
            width,
            height,
            pixel_type,
            buffer: vec![0; width * height * 4],
        }
    }

    pub fn get_pixel(&self, rth: usize, cth: usize) -> [u8; 4] {
        let start = ((rth - 1) * self.width + (cth - 1)) * 4;
        let buffer = &self.buffer;
        [buffer[start], buffer[start + 1], buffer[start + 2], buffer[start + 3]]
    }
    pub fn set_pixel(&mut self, rth: usize, cth: usize, pixel: [u8;4]){
        let start = ((rth - 1) * self.width + (cth - 1)) * 4;
        let mut buffer = &mut self.buffer;
        buffer[start + 0] = pixel[0];
        buffer[start + 1] = pixel[1];
        buffer[start + 2] = pixel[2];
        buffer[start + 3] = pixel[3];
    }
    pub fn get_pixel_enumerate_position(&self, rth: usize, cth: usize) -> [u8; 4] {
        let start = (rth * self.width + cth) * 4;
        let buffer = &self.buffer;
        [buffer[start], buffer[start + 1], buffer[start + 2], buffer[start + 3]]
    }
}

impl Canvas for ImageBuffer {
    fn draw_line(&mut self, start: (usize, usize), end: (usize, usize)){
        for pos_x in start.0..end.0{
            self.set_pixel(pos_x, start.1, [255, 255, 255, 255]);
        }

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
