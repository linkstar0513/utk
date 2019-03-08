use std::time::{
    Instant,
    Duration
};

use crate::window::WindowSurface;
use crate::event::{
    Event,
    EventsLoop,
    WindowEvent,
    AppEvent
};

pub mod cmd;
pub use self::cmd::Cmd;

pub struct Application {
    windows:Vec<WindowSurface>,
    event_loop:EventsLoop,

}

/**
new -> init -> exec-> shutdown
*/

impl Application {
    pub fn new() -> Self {
        Application {
            windows:Vec::new(),
            event_loop: EventsLoop::new(),
        }
    }
    pub fn init(&mut self){
        self.emit_event(Event::MouseEvent);
        self.emit_event(Event::WindowEvent{event:WindowEvent::Close});

        self.emit_event(Event::AppEvent{event:AppEvent::Close});


        println!("---------------------------------------");
        dbg!("init");
    }
    // 程序运行入口

    pub fn exec(&mut self){
        self.init();

        let mut is_continue = true;
        loop {
            self.event_loop.poll_events(|event|{

            });
            break;
        }

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

    pub fn exec_poll_event(&mut self){
        self.init();
        let mut is_continue = true;

        while is_continue {

            println!("---------------------------------------");
            dbg!("frame start");
            let pre = Instant::now();

            self.event_loop.poll_events(|event|{
                match event {
                    Event::AppEvent{event:AppEvent::Close} => {
                        is_continue = false;
                    },
                    Event::WindowEvent{..} => {
                        dbg!(event);
                    },
                    Event::MouseEvent => {
                        dbg!(event);
                    }
                    _ => {
                        dbg!(event);
                    }
                }
            });
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
    pub fn poll_event(&mut self) -> bool{
        self.event_loop.poll_events(|e|{
            dbg!(e);
        });
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