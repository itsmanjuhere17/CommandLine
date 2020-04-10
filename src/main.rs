use std::env;
use std::io;
fn main() {
    println!("Hello, world!");
    let args:Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("Query to search:{}",query);
    println!("In File Name:{}",file_name);
}
