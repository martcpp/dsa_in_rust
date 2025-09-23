use std::io;
use std::fs;
use std::env;
fn readuserinput()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn commandline()->(String,String){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3  {
        ("No command line argument found".to_string(),String::new())
    } else {
        (args[1].to_string(),args[2].to_string())
    }

}

fn readfile(path:&str)->Result<String, io::Error>{
    let contents = fs::read_to_string(path);
    contents
}