use std::io::{self, Write};
use std::fs::{File,self};

pub struct Config{
    path:String,
    message:String,

}

impl Config{
    pub fn init(args:&Vec<String>)->Config{
        Config { path: args[1].clone(),message:args[2].clone() }
    }
    pub fn read_file(&self){
        let file = checkfile(&self.path,&self.message);
        println!("{}",file);
    }
}
//반복자 이용
/* impl Config{
    pub fn init(mut args:impl Iterator<Item=String>)->Result<Config,String>{
        args.next();

        //Config { path: args.next().unwrap(),message:args.next().unwrap() }
        let tpath = match args.next(){
            Some(args)=>args,
            None=>return Err("err parsing args".to_string())
        };
        let tmessage=match args.next(){
            Some(args)=>args,
            None=>return Err("err parsing args".to_string())
        };

        Ok(Config { path: tpath, message: tmessage })
    }
    pub fn read_file(&self){
        let file = checkfile(&self.path,&self.message);
        println!("{}",file);
    }
} */

pub fn checkfile(path:&String,message:&String)->String{
    let contents_res = fs::read_to_string(path);
    match contents_res{
        Ok(content)=>{
            println!("file exist finding message");
            let mut ret=String::new();
            for line in content.lines(){
                if line.contains(message){
                    ret.push_str(line);
                    ret.push_str("\n");
                }
            }
            if ret.is_empty(){
                String::from("can't find")
            }else{
                ret
            }
            
        },
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