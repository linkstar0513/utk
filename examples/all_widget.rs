use utk::app::Application;
use utk::window::WindowSurface;
use utk::event::Event;

fn main(){
    let mut app = Application::new();
    let surface = WindowSurface::new();
    app.add_window(surface);
    app.emit_event(Event::MouseEvent);
    app.exec_poll_event();
}