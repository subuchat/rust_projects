use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secretnum:u32 = rand::thread_rng().gen_range(1..101);
    //println!("The generated secret number is {}",secretnum);

    loop{
        println!("Your Guess please : ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    

        //let guessnew : u32 = guess.trim().parse().expect("Please provide a number");
        let guessnew : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue,
        };
        println!("You guessed: {}", guess);
        match guessnew.cmp(&secretnum){
            Ordering::Less => println!("Guess is less than the secret num"),
            Ordering::Greater => println!("Way too much."),
            Ordering::Equal =>  {
                println!("Perfeto !!");
                break;
            }            
        }
        println!("The generated secret number was {}",secretnum);
    }

}
