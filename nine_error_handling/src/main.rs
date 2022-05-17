//Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that 
//stops execution when the program encounters an unrecoverable error

// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. 
//But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. 
//Memory that the program was using will then need to be cleaned up by the operating system

use std::fs::{self,File};
use std::io::{self, ErrorKind , Read } ;

fn main() {
    //test_with_match();
    //test_with_unwrap_or_else(); // return ok value and need to handle error
    //learn_unwrap_exception(); //unwrap will panic directly , whereas expect will allow you to handle exception

    let filename = "Hello.txt".to_string();

    let error1 = test_with_propagating_error(&filename);
   // read_username_from_file(&filename);
   read_username_from_file_simplerform(&filename);

   read_sername_in_one_line(&filename);

   last_char_of_first_line("myfile.txt");

}

fn last_char_of_first_line(text : &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_sername_in_one_line(filename : &String) -> Result<String, io::Error>{
    fs::read_to_string(filename)
}

fn read_username_from_file_simplerform(filename :&String) ->Result<String , io::Error>{
    let mut mystr = String::new();
    File::open(filename)?.read_to_string(&mut mystr)?;
    Ok(mystr)
}
//this will do the same thing as test_with_propagating_error() but with ?
// ? will continue if OK , in case of err will return from whole program.. return may be Result/Option (?)
fn read_username_from_file(filenm : &String) -> Result<String , io::Error> {

    let mut f = File::open(filenm)?;
    let mut mystr = String::new();
    f.read_to_string(&mut mystr)?; // read the stream within file or return error // f need mutablle
    Ok(mystr)

}

// error propagation .. so this func will return error as result when file not created. 
// so this needs to be handled in calling function

fn test_with_propagating_error(filenm : &String) -> Result<String , io::Error> {
    let f = File::open(filenm);

    let mut retval = match f{
        Ok(fl) => fl,
        Err(e) => return Err(e)
    };
    
    let mut st = String::new();
    match retval.read_to_string(&mut st) {
        Ok(_) => Ok(st),               // returnning string(st) not length so _
        Err(stream_err) => Err(stream_err),
    }
}

fn learn_unwrap_exception() {
     //let f = File::open("myfile1.txt").unwrap(); //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value
     //let f = File::open("myfile1.txt").expect("Failed to open file myfile1.txt"); //thread 'main' panicked at 'Failed to open file myfile1.txt
    // let ipadd = "127.0.0.1".parse().unwrap(); // parse will be in formatted string .. check trim() also .. this line is not part of the code

}

fn test_with_unwrap_or_else() {
    let f = File::open("myfile.txt").unwrap_or_else( |error| {
                                                            if error.kind() == ErrorKind::NotFound {
                                                               File::create("myfile.txt").unwrap_or_else( |error2| {
                                                                panic!("Problem creating file {:?}",error2);
                                                               }                                                            
                                                               ) //create end
                                                            } //end of if create
                                                            else {
                                                                panic!("Problem opening file {}" , error);
                                                            }

                                                        }// end of unwrap_or_else
                                                  
                                                     );


}

fn test_with_match() {
    
    let fl = File::open("hello.txt"); //u can test  defining 'let fl: u32 ' this way for open retiurn type.. just to check 
    //n the case where File::open succeeds, the value in the variable f will be an instance of Ok that contains a file handle. 
    // In the case where it fails, the value in f will be an instance of Err that contains more information about the kind of error that happened.

    //match fl{
    //    Ok(_)      => println!("Good"),            //Ok(file) => file,
    //    Err(error) => println!("Bad"),             //println!("Error occured {:?}",error), //panic!("Problem opening the file: {:?}", error),
    //}

    let f = match fl {
        Ok(file) => {
            println!("File exist");
            file
        },
        Err(error) => match error.kind() {
           ErrorKind::NotFound => match File::create("hello.txt") {
               Ok(fc) => {
                   println!("Opened the new file"); 
                   fc
               }
               Err(e) => panic!("problem creating file {:?}",e),
            },
            other_error => {
                panic!("Problem opening file {:?}",other_error)
            }

        },
    };
}

fn test_panic() {
    println!("Panic test!");
    //panic!("crash and burn");
    
    let vect = vec![1,2,3];
    vect[99];
}
