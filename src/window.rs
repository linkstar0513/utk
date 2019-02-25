use std::vec::Vec;
use crate::event::Event;
use crate::video::Render;
pub struct WindowSurface {
    inner:Vec<u32>,
    width:u32,
    height:u32,
    events_loop:Vec<Event>,
    render:Render,
}

impl WindowSurface {
    pub fn new()-> Self {
        WindowSurface{
            inner:Vec::new(),
            width:600,
            height:800,
            events_loop:Vec::new(),
            render:Render::new(),
        }
    }
    pub fn run(&mut self){
        self.render();
    }
    // 窗口event处理
    pub fn poll_event(&mut self){
        for event in self.events_loop.iter(){
            println!("event");
        }

    }
    //窗口渲染
    pub fn render(&mut self) {
        self.render.render()
    }
}