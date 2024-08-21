use std::env;
use minigrep::Config;
fn main(){
    //asdf
    let args:Vec<String>=env::args().collect();
    let file_name : Config = Config::init(&args);
    file_name.read_file();
}