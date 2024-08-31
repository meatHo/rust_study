use std::env;
use minigrep::Config;
fn main(){
    //asdf
    let args:Vec<String>=env::args().collect();
    let file_name : Config = Config::init(&args);
    file_name.read_file();

    //No clone 반복자 통해서 값 이동한 코드
    /* let args=env::args();
    let file_name : Config = Config::init(args).unwrap_or_else(|err|{
        panic!("{}",err);
    });
    file_name.read_file(); */
}