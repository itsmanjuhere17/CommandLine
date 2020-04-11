use std::env;
use std::fs;
//use std::error::Error;
use std::io;

#[derive(Debug,PartialEq)]
pub struct Config{
    pub query:String,
    pub filename:String,
    pub is_case_sensitive:bool
}

impl Config{
    pub fn new(args:&Vec<String>)->Result<Config,&'static str>{
        if args.len()<3{
            return Err("Not enough arguments");
        }
        let _query = args[1].clone();
        let file_name = args[2].clone();
        let case_sesitive = env::var("CASE_INSENSITIVE").is_err(); //is_err returns true, if env variable is not set.So, it does case sensitive search.
        let config = Config{
            query:_query,
            filename:file_name,
            is_case_sensitive:case_sesitive
        };
        Ok(config)
    }//end of new function
}

pub fn run(config:Config)->Result<(),io::Error> {
    let file_content = fs::read_to_string(config.filename)?;
    if config.is_case_sensitive{
        for hit in search(&config.query,&file_content){
            println!("{}",hit);
        }
    }
    else{
        for hit in search_case_not_sensitive(&config.query,&file_content){
            println!("{}",hit);
        }
    }

    Ok(())
}

//Here, if we don't specify lifetime, rust wont compile as it does not know till what lifetime the return value should be contained.
pub fn search<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    let mut res = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            res.push(line);
        }
    }
    res
}

pub fn search_case_not_sensitive<'a>(query:&str,content:&'a str)->Vec<&'a str>{
    let mut res = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){
        let query="duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,content));
    }
    #[test]
    fn test_config_new_error(){
        let empty = Vec::new();
        let res = Config::new(&empty);
        assert_eq!(Err("Not enough arguments"),res);
    }

    #[test]
    fn test_config_new_ok(){
        let args = vec!["CommandLineProject".to_string(),"hello".to_string(),"hello_world.txt".to_string()];
        assert_eq!(Ok(Config{
            query:args[1].to_string(),
            filename:args[2].to_string(),
            is_case_sensitive:false
        }), Config::new(&args));
    }
}