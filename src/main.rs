use CommandLineProject::Config; //Library Crate.
use std::env;
use std::process;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{}",err);
        process::exit(1);
    });
    if let Err(err)=CommandLineProject::run(config){
        eprintln!("Error running application:{}",err);
        process::exit(1);
    }
}


