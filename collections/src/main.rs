fn main() {

    println!("Learn Vector and other collection!");
    let mut v : Vec<u32> = Vec::new();
    v.push(4);
    println!("Value of vector 0th position : {}" , &v[0]); // to get direct value. get() will return in option form
    //let sec_vec = vec!['a','b','c'];
    //let temp_val = &sec_vec[100];

    let mut another_vector = vec![1,2,3,4,5];
    //another_vector.pop();
    match another_vector.get(10) {
        Some(num) => println!("Value in get way for 3rd position {}", num), // deref using enum Option
        None      => println!("Invalid value")
    }
    //iterating over vector values
    for retval in &mut another_vector {
        println!("Value in vec is {} ", retval);
        (*retval) += 20;
        println!("New Value after modificaiton  in vec is {} ", retval);

    }
    // *** Using an Enum to Store Multiple Types ***
    #[derive(Debug)]
    enum SpreaSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![SpreaSheetCell::Int(4),SpreaSheetCell::Float(5.6),SpreaSheetCell::Text(String::from("Subrata"))];
    for val in &row {
        println! ("Values printing in mized vector {:?}" , val);
    }

    test_string();
    test_hashmap();
}

fn test_string(){
    let mut teststr = String::new();
    teststr.push_str("Subrata");        // string add
    teststr.push('c');                  // only charater add
    println!("Value of new string : {}" , teststr.to_string());
    
    let data = "Subrata Chattopadhyay";
    let s = data.to_string();
    println!("{}",s);
    let hello = String::from("नमस्ते"); // UTF-8
    //We can also use the function String::from to create a String from a string literal.
    println!("{}",hello.to_string());

    //concatenation with + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used [s2 is ref so will be intact]
    println!("{} : {}",s3, s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");

    let final_str = format!("{}-{}-{}",s1,s2,s3); //SAme as - let s = s1 + "-" + &s2 + "-" + &s3; - with difference that nothing moved, all var intact.so format! behaving line prinln!
    println!("FInal string {}",final_str.to_string());

    println!("s1 : {}",s1);
    println!("s2 : {}",s2);
    println!("s3 : {}",s3);

    //let take_slice = s1[0]; // string cant be indecexed like integer bcoz A String is a wrapper over a Vec<u8>
    //in UTF-8, because each Unicode scalar value in that string takes variable(?) bytes of storage

    let take_slice = &s1[0..2]; //problematic in case of other language due to char boundary ?
    println!("take_slice of s1 {}",take_slice);

    //****************************right way to iterating over String ****************************************
    
    let namste = "नमस्ते".to_string();
    dbg!(&namste);
    println!("Printing in CHARS");
    for lettr in namste.chars() {
        println!("{}",lettr);
    }
    //The bytes method returns each raw byte, which might be appropriate for your domain:
    println!("Printing in BYTES");
    for indiv_byte in namste.bytes() {
        println!("{}",indiv_byte);
    }

}

fn test_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Amitesh"), 50);
    scores.insert(String::from("Subrata"), 55); //Key - string , value -> i32
    
    //zip method to create an iterator of tuples where “Blue” is paired with 10, and so forth. Then we could use the collect method to turn that iterator of tuples into a hash map
    // zip return tupple , collect will make that tuple and make hashmap
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scores2 : HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //accessing data

    let blue = "Blue".to_string();
    let yellow = "Yellow".to_string();
    let mut score3 = HashMap::new();
    score3.insert(&blue, 10);
    score3.insert(&yellow, 50);
    
    //accessing values from hashmap - using get() methiod
    let scoreofblue = score3.get(&blue);
    //println!("Score of blue team {}",&scoreofblue); as get() return enum we can use if let to deref the value
    if let Some(val) = scoreofblue {
        println!("The score of blue is {}",val);
    }

    //another example of reading by tuple

    for (key,val) in score3 {
        println!("{} , {}", key,val);
    }

    //Only Inserting a Value If the Key Has No Value : Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
    //The return value of the entry method is an enum called Entry that represents a value that might or might not exist
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, 
    // inserts the parameter as the new value for this key and returns a MUUTABLE REFERENCE to the new value. 

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

