
use std::thread;
use std::time::Duration;

fn main() {
    println!("Example of closures!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!("************* USE OF ITERATOR****************");
    iteratorusage();
}

fn generate_workout(intensity: u32, random_number: u32){

    let expensive_closure = |num| {
       println!("Calculating slowly ..");
       thread::sleep(Duration::from_secs(2));
       num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    }
    else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else{
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn iteratorusage() {
    let v1 = vec![1,5,10];
        
    for val in v1.iter() {
        println!("Values in the vectors: {}", val );

    }

    let v2 : Vec<_> = v1.iter().map(|x| x+1 ).collect(); //iterators are lazy , so had to use collect() to make another vector from closure map
    assert_eq!(v2 , vec![2,6,11]);
}

