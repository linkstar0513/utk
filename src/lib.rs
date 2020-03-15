#[allow(non_snake_case)]
pub fn translate(){
    println!("translate");
}

pub mod device;
pub mod platform;
pub mod core;
pub mod app;
pub mod window;
pub mod event;
pub mod audio;
pub mod video;
pub mod net;
pub mod dom;
pub mod component;
pub mod image;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
