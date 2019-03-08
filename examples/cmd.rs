use utk::app::Cmd;

fn main(){
    let mut cmd = Cmd::new();
    println!("{}",cmd.match_str("target\\debug\\examples\\cmd.exe"));
    match cmd.match_str("--title") {
        true => {
            println!("Title");
        },
        false => {
            println!("Notitle");
        },
    }
    cmd.exec();
}