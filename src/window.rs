use std::vec::Vec;
pub struct WindowSurface {
    inner:Vec<u32>,
    width:u32,
    height:u32,
}

impl WindowSurface {
    pub fn new()-> Self{
        WindowSurface{
            inner:Vec::new(),
            width:600,
            height:800
        }
    }
    pub fn run(&mut self){
        println!("width{}height{}",self.width,self.height);
    }
}