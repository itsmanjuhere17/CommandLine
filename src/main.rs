use CommandLineProject::Config; //Library Crate.
use std::env;
use std::process;
fn main() {
    println!("Hello, world!");
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    if let Err(err)=CommandLineProject::run(&config){
        println!("Error running application:{}",err);
        process::exit(1);
    }
}

