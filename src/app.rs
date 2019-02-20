use std::time::{
    Instant,
    Duration
};

use crate::window::WindowSurface;
use crate::event::Event;

pub struct Application {
    windows:Vec<WindowSurface>,
    event_loop:Vec<Event>,

}

/**
new -> init -> exec-> shutdown
*/

impl Application {
    pub fn new() -> Self {
        Application {
            windows:Vec::new(),
            event_loop: Vec::new(),
        }
    }
    pub fn init(&self){
        println!("---------------------------------------");
        dbg!("init");
    }
    // 程序运行入口

    pub fn exec(&mut self){
        self.init();

        let mut is_continue = true;

        while is_continue {
            dbg!("frame start");


            self.drain_event();
            is_continue = self.poll_event();

            let pre = Instant::now();
            self.update();
            dbg!("frame end");
            println!("---------------------------------------");

            self.sycn_fps(&pre);


        }


        self.shutdown();
    }
    //更新视图
    pub fn update(&mut self){
        for window in self.windows.iter_mut(){
            window.run();
        }
    }
    // 应用event
    pub fn emit_event(&mut self, event:Event){
        self.event_loop.push(event);
    }

    // 系统event转换为utk event
    pub fn drain_event(&mut self){

    }
    //事件处理
    pub fn poll_event(&self) -> bool{
        for event in self.event_loop.iter(){
            match event {
                Event::WindowEvent => {
                    dbg!(3);
                    return false;
                },
                _ => {
                    dbg!(0);
                    return true;
                }
            }
        }
        return true;

    }
    //增加窗口
    pub fn add_window(&mut self,window:WindowSurface){
        self.windows.push(window);
    }
    pub fn shutdown(&self){
        dbg!("shutdown");
        println!("---------------------------------------");
;
    }
    // 同步帧率
    pub fn sycn_fps(&self, pre: &Instant){
        let cal = pre.elapsed();
        let fps_delay = Duration::new(1,0)/1;
        let sleep_time = fps_delay - cal;
        ::std::thread::sleep(sleep_time);
    }
}