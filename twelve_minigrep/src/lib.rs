use std::{fs, error::Error , env,};

pub struct Config{
    pub query : String,
    pub filename : String,
    pub case_sesitive :bool,
}

//Constructor way
impl Config {
    pub fn new(args :&[String]) -> Result<Config,&str> {
        if args.len() == 3{
            let query = args[1].clone();
            let filename=args[2].clone();

            let case_sesitive = env::var("CASE_INSENSITIVE").is_err();
            Ok( Config{query,filename,case_sesitive } )
        }
        else{
            Err("Invalid number of argument")
        }
    }
}


pub fn run(file_config: Config) -> Result<(),Box<dyn Error>>{
    println!("Searching for {}", file_config.query);
    println!("In file {}", file_config.filename);

    let contents = fs::read_to_string(file_config.filename)?;

    let results = if file_config.case_sesitive{
        search(&file_config.query, &contents)
    } else {
        search_case_insensitive(&file_config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    
    //println!("With text:\n{}", contents);
    
   Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str >{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query : &str , contents : & 'a str)-> Vec<&'a str>{

    let query = query.to_lowercase();
    let mut result = Vec::new();

    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query){
            result.push(lines);
        }
    }


    result
}