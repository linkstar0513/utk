
pub struct Cmd {

}
impl Cmd{
    pub fn new() -> Self{
        Cmd {

        }
    }
    ///! 命令行参数是否匹配字符串
    pub fn match_str(&mut self, pattern: &str) -> bool {
        for argument in ::std::env::args().skip(1){
            if argument.as_str() == pattern {
                return true;
            }
        }
        false
    }
    ///！ 运行程序
    pub fn exec(&mut self){
        for (key, value) in ::std::env::vars() {
            println!("{},{}", key, value);
        }
        for argument in ::std::env::args(){
            println!("{}",argument);
        }
    }
}