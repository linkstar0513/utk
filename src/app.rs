use std::time::{
    Instant,
    Duration
};

use crate::window::{WindowSurface, VitrualDomSurface};
use crate::window::Window;
use crate::event::{
    Event,
    EventsLoop,
    WindowEvent,
    AppEvent
};

// use winit::{
//     event::{Event, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     window::WindowBuilder,
// };

pub mod cmd;
pub use self::cmd::Cmd;

// type WEventLoop = winit::event_loop::EventLoop;
pub struct EventStore{
    init: Vec<Box<Fn()>>,
    // mouseEventHandle: Vec<Box<Fn()>>,
    // customEventHandle: Vec<Box<Fn()>>,
}
impl EventStore {
    pub fn new() -> Self{
        EventStore {
            init: Vec::new(),
        }
    }
}

pub struct Application {
    windows: Vec<Box<dyn Window>>,
    event_loop:EventsLoop,
    event_store: EventStore,
    // winit_event_loop: winit::event_loop::EventLoop<()>,
}

/**
new -> init -> exec-> shutdown
*/

impl Application {
    pub fn new() -> Self {
        // let winit_event_loop = winit::event_loop::EventLoop::new();
        Application {
            windows:Vec::new(),
            event_loop: EventsLoop::new(),
            event_store: EventStore::new(),
            // winit_event_loop,
        }
    }
    pub fn on_init(&mut self, init: Box<Fn()>) {
        self.event_store.init.push(init);
    }
    pub fn init(&mut self){
        self.emit_event(Event::MouseEvent);
        self.emit_event(Event::WindowEvent{event:WindowEvent::Close});

        self.emit_event(Event::AppEvent{event:AppEvent::Close});


        // println!("---------------------------------------");
        dbg!("init");
    }
    // 程序运行入口

    // 分发event
    pub fn dispatch(&mut self, event: Event){

    }

    pub fn exec(&mut self){
        for init_all in &self.event_store.init {
            init_all();
        }
        self.init();

        let mut is_continue = true;
        loop {
            self.event_loop.poll_events(|event|{

            });
            break;
        }
        let eventloop = winit::event_loop::EventLoop::new();
        eventloop.run(move |event, _, control_flow| {
            match event {
                _ => {
                    // selfRef.dispatch(Event::MouseEvent);
                    // println!("{:?}",event);
                },
            }
        });
        // eventloop.run(move |event, _, control_flow| {
        //     match event {
        //         _ => {
        //             // selfRef.dispatch(Event::MouseEvent);
        //             // println!("{:?}",event);
        //         },
        //     }
        // });
        // while is_continue {
        //     dbg!("frame start");


        //     self.drain_event();
        //     is_continue = self.poll_event();

        //     let pre = Instant::now();
        //     self.update();
        //     dbg!("frame end");
        //     // println!("---------------------------------------");

        //     self.sycn_fps(&pre);


        // }


        self.shutdown();
    }

    pub fn exec_poll_event(&mut self){
        self.init();
        let mut is_continue = true;

        while is_continue {

            // println!("---------------------------------------");
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
            // println!("---------------------------------------");


            self.sycn_fps(&pre);

        }







        self.shutdown();
    }

    //更新视图
    pub fn update(&mut self){
        for window in self.windows.iter_mut(){
            window.update();
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
    pub fn shutdown(&self){
        dbg!("shutdown");
        // println!("---------------------------------------");
    }
    // 同步帧率
    pub fn sycn_fps(&self, pre: &Instant){
        let cal = pre.elapsed();
        let fps_delay = Duration::new(1,0)/1;
        let sleep_time = fps_delay - cal;
        ::std::thread::sleep(sleep_time);
    }
}
impl Application {
    pub fn add_window(&mut self, window: impl Window + 'static){
        self.windows.push(Box::new(window));
    }
}

impl Application {
    pub fn on_close(&self) {
        // println!("close");
    }
}
use crate::dom::VitrualDom;
impl Application {
    pub fn render(&mut self,mut rootDom: VitrualDom, VWindow: ()){
        println!("{:#?}", &rootDom);
        self.windows.push(Box::new(VitrualDomSurface::new(rootDom)));
    }
}