use crate::dom::VitrualDom;
pub type Button = VitrualDom;
pub type Label = VitrualDom;

pub struct ImageBuffer {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

pub type Image = ();

// 组件widget
pub trait Widget {
    fn inner_image() -> Image {
        ()
    }
}