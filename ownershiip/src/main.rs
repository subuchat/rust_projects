fn main() {
    let mut str1 = String::from("Hello");
    str1.push_str(" , world!");
    println!("Str1 value : {} ", str1);
    let str2 = str1;
    println!("Str2 value : {} ", str2);
    //println!("Str1 value after move: {} ", str1); error //std::move
    //but deep copy now using clone

    let str3 = str2.clone();
    println!("Str2 value : {}  , str3 value {}", str2, str3);
   
    let s1 = gives_ownership_copyelission();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let mut s3 = takes_and_gives_back(s2);  // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
    println!("S3 value : {}" , s3);

    let length = calculate_length(&s3) ; // pass by reference (borrowing) , no move
    println!("Length of s3 string {}" , length);

    {
        let r1 = &mut s3;        
    } // r1 goes out of scope here, so we can make a new reference with no problems.else due to data race avoidance , rust dont allow 2 borrowing
    let r2 = &mut s3;

    //let borrow_frm_outofscope = dangle();
    let borrow_frm_outofscope = no_dangle();
    print!("borrow_frm_outofscope : {}" , borrow_frm_outofscope);
}
/* fn dangle() -> &String {
    let str5 = String::from("another");
    &str5
} */
fn no_dangle() -> String {
    let str5 = String::from("another");
    str5
}

fn calculate_length(strtemp : &String) ->usize {
    strtemp.len()
}
fn gives_ownership_copyelission() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function

}

fn takes_and_gives_back(a_str : String) -> String {
   // a_str.push_str(" subu")  // a_string is returned and moves out to the calling function
   a_str
}