use std::{env,process };
use mylib::Config;


fn main() {
    let args: Vec<String> = env::args().collect(); //anything iterable
    println!("{:?}", args);

   //let file_config = parse_config(&args);
   let file_config = Config::new(&args).unwrap_or_else( |err| {
       println!("Problem parsing arguments: {}" , err);
       process::exit(1);
   }   );
   
   if let Err(e) = mylib::run(file_config) {
       println!("Application Error : {}", e);
       process::exit(1);
   }

    
}

/* 
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // arg[0] is program name
    let filename = args[2].clone();
    Config { query,filename }
} */


