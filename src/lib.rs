use std::env;
use std::fs;
use std::process;
//use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct Config{
    pub query:String,
    pub filename:String
}

impl Config{
    pub fn new(args:&Vec<String>)->Result<Config,&'static str>{
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

pub fn run(config:&Config)->Result<(),io::Error> {
    let file_content = fs::read_to_string(&config.filename)?;
    println!("File content is:{}",file_content);
    Ok(())
}