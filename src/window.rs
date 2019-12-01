use std::vec::Vec;
use crate::event::Event;
use crate::video::Render;

/// 提供窗口的抽象
pub trait Window {
    fn update(&mut self);
}
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

impl Window for WindowSurface {
    fn update(&mut self){
        dbg!("WindowSurface更新");
    }
}
// 虚拟dom窗口
use crate::dom::VitrualDom;
pub struct VitrualDomSurface {
    root_dom: VitrualDom,
}
impl VitrualDomSurface {
    pub fn new(virtual_dom: VitrualDom) -> Self {
        VitrualDomSurface{
            root_dom: virtual_dom,
        }
    }
    fn update(&mut self){
        dbg!("VirtualDomSurface更新");
        self.root_dom.render();
    }
}

impl Window for VitrualDomSurface {
    fn update(&mut self){
        dbg!("VirtualDomSurface更新");
        self.root_dom.render();
    }
}