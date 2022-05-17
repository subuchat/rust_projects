use std::io;

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("Value of x is {}", x);
    {
        const x :i16 = -10;
        println!("THe value of const x is {}",x);
    }

    let x = 6;
    println!("Value of x is {}", x);
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("all char output c {} z {} heart_eyed_cat {}", c ,z,heart_eyed_cat);

    let tup : (i32, f64, u8) = (500 , 6.4 , 1);
    let (x , y , z ) = tup;
    println!("Value of x {} ",x);
    let abc = 5;
    array_function(abc);
    
   expression_within_statement();

   let retval  = function_with_returnvalue();
   println!("Return value {}", retval);

   controlflow_test("subrata");

   test_loop();
   test_forloop();
   test_whileloop();


}
fn test_whileloop() {
    let arr = [1,2,3,4,5];
    let mut index = 0;
    while index < 5 {
        println!("The value in while loop {}" , arr[index]);
        index = index+1;
    }
}
fn test_forloop() {
  for num in (1..5).rev(){
      println!("Number in for loop reverse order {}" , num);
  }
}

fn test_loop() {
   let mut count = 0;
   'my_extern_loop : loop{ //outer loop lebel
       println!("Count {}",count);
       let mut remaining = 3;

       let result =loop{
                    println!("Remaining in inner loop {}" , remaining);
                    if remaining == 1{
                        break remaining*5;
                    }
                    if count == 1{
                        break 'my_extern_loop;
                    }
                    remaining -= 1;
            };
            count +=1;
            println!("Result value = {}", result);
   }
   println!("End count = {}", count);
}

fn controlflow_test( input1 : &str) {
 let num = 3;
 //let _num2 = 5;
 if (num < 4) && (input1 == "hel") {
     println!("COndition is true");
 }
 else if num == 2 {
    println!("Condition is false");
 }
 else
 {
     println!("Just a test");
 }

}

fn function_with_returnvalue() -> u32 {
    5
}

// array_function(&x : &i32) pass by reference

fn array_function(x : i32) {
    println!("value in function {}",x);
    let month : [&str ; 2] = ["Jan", "Feb"];
    println!("First month : {}" , month[0]);
    //other way of declaring 
    let _a = [1, 2, 3, 4, 5]; //unused so _a

    println!("PLease enter the index of array a you want to see :  ");
    let mut inputstr = String::new();

    io::stdin().read_line(&mut inputstr).expect("Unable to read");

    let mut index : usize = inputstr.trim().parse().expect("Entered index is not a number");
    let element = _a[index];

    println!("The value of the element at index {} is: {}", element,index);
}
//expresseion return a value (here no ned to return value only no semicolon).statement does not retrun statement
fn expression_within_statement() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("Value of y {} :" , y);
}
