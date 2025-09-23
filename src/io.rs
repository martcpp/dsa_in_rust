use std::io::{self, BufRead};
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::env;
pub fn readuserinput()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn commandline()->(String,String){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3  {
        ("No command line argument found".to_string(),String::new())
    } else {
        (args[1].to_string(),args[2].to_string())
    }

}

pub fn readfile(path:&str)->Result<String, io::Error>{
    let contents = fs::read_to_string(path);
    contents
}

pub fn readfilebuffer(path:&str)->Result<Vec<u8>, io::Error>{
    let contents = fs::read(path);
    contents
}

pub fn myread(path:&str){
    let contents = fs::File::open(path).unwrap();
    let reader = io::BufReader::with_capacity(1024, contents);
    for line in reader.lines(){
        sleep(Duration::from_secs(2));
        println!("{}",line.unwrap());
        sleep(Duration::from_secs(2));
    }
}