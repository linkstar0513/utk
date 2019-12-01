#![allow(non_snake_case)]
use utk::app::Application;
use utk::window::WindowSurface;
use utk::event::Event;
use utk::dom::VitrualDom;
use utk::component::{
    Button, Label,
};

fn main(){
    let mut app = Application::new();
    let surface = WindowSurface::new();
    app.add_window(surface);
    let mut root_dom = VitrualDom::new();
    root_dom.pushChild(VitrualDom::new());
    let mut button = Button::new(); button.vtype = "Button" ;button.virtualDomId = 900;
    root_dom.pushChild(button);
    app.render(root_dom, ());
    app.emit_event(Event::MouseEvent);
    app.exec();
}