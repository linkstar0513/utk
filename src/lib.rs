
pub fn translate(){
    println!("translate");
}


pub mod core;
pub mod app;
pub mod window;
pub mod event;
pub mod audio;
pub mod video;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
