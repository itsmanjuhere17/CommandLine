use std::env;
use std::io;
use std::fs;
use std::process;
#[derive(Debug)]
struct Config{
    query:String,
    filename:String
}

impl Config{
    fn new(args:&Vec<String>)->Result<Config,&'static str>{
        if args.len()<3{
           return Err("Not enough arguments");
        }
        let _query = args[1].clone();
        let file_name = args[2].clone();
        let config = Config{
            query:_query,
            filename:file_name,
        };
        Ok(config)
    }//end of new function
}
fn main() {
    println!("Hello, world!");
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    let file_content = fs::read_to_string(config.filename).expect("Could not read from file");
    println!("File content is:{}",file_content);
}
