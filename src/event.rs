use std::ops::Deref;

#[derive(Debug)]
pub enum  Event {
    WindowEvent {
        event:WindowEvent
    },
    MouseEvent,
    AppEvent{
        event:AppEvent
    },
}

#[derive(Debug)]
pub enum WindowEvent {
    /// 窗口关闭事件
    Close,
}

#[derive(Debug)]
pub enum AppEvent {
    /// 应用程序关闭
    ///
    Close,
    /// 应用程序挂起
    Suspend,
}





#[derive(Debug)]
pub struct EventsLoop(Vec<Event>);

impl EventsLoop{
    pub fn new()->Self{
        EventsLoop(Vec::new())
    }


    /// 从系统中请求获取事件队列
    fn get_events_from_system(&mut self){
        self.0.push(Event::MouseEvent);
    }

    /// 事件队列的每个事件调用回调函数

    pub fn poll_events<F>(&mut self, mut callback: F)
    where F:FnMut(Event){

        self.get_events_from_system();
        loop {
            match self.0.pop() {
                Some(event)=>{
                    callback(event);
                }
                None => {
                    break;
                }
            }
        }
    }
    pub fn push(&mut self, event:Event){
        self.0.push(event);
    }
}
#[derive(Default)]
pub struct EventEmitter{
    pub event_handle: Vec<Box<dyn Fn()>>
}
impl EventEmitter {
    pub fn emit(){}
}