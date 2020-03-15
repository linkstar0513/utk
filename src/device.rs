use winit_blit::{PixelBufferTyped, NativeFormat};
pub trait Canvas{
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, painter: ());
}


pub struct CanvasContext{}
pub struct CanvasContainer<'ctx, 'ctype, T>{
    ctx: &'ctx CanvasContext,
    ctype: &'ctype mut T,
}
impl <'ctx, 'ctype, T> CanvasContainer<'ctx, 'ctype, T>{
    pub fn new(ctx: &'ctx CanvasContext, ctype: &'ctype mut T) -> Self {
        CanvasContainer{
            ctx,
            ctype,
        }
    }
}
impl <'ctx, 'ctype> Canvas for CanvasContainer<'ctx, 'ctype, PixelBufferTyped<NativeFormat>>
{
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, painter: ()) {
        for row in x..x+5{
            let mut row_arr_ref = self.ctype.row_mut(row as u32).unwrap();
            for pixel_ref in row_arr_ref.iter_mut(){
                *pixel_ref = NativeFormat::from_rgb(
                     255u8,
                    255u8,
                    255u8,
                );
            }
        }
    }

}