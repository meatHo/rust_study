use std::io::{self, Write};
use std::fs::{File,self};

pub struct Config{
    path:String
}

impl Config{
    pub fn init(args:&Vec<String>)->Config{
        Config { path: args[1].clone() }
    }
    pub fn read_file(&self){
        let file = checkfile(&self.path);
        println!("{}",file);
    }
}

pub fn checkfile(path:&String)->String{
    let contents_res = fs::read_to_string(path);
    match contents_res{
        Ok(content)=>content,
        Err(e)=>match e.kind(){
            io::ErrorKind::NotFound=>{
                file_create(&path);
                String::from("new file created")
            },
            other_error=>panic!("Error")
        }
    }
}

fn file_create(path:&str){
    let mut created_file = File::create(path).expect("can't create file");
    println!("새 파일 내용 입력하세요");
    let mut content=String::new();
    io::stdin().read_line(&mut content).expect("error");
    created_file.write_all(&content.as_bytes()).expect("error writing file");
    
}