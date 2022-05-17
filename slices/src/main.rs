fn main() {
    let mystr = String::from("Hello world!");

    let len = mystr.len();
    let slice1 = &mystr[0..5]; // or &mystr[..5] 
    let slice2 = &mystr[5..len]; // or &mystr[6..]
    println!("Slice1 : {} , Slice2 : {} " , slice1 , slice2);
    
    let word = first_word_of_str(&mystr);
    println!("Word : {}",word);
    
}
 // String is mutable , str is immutable
fn first_word_of_str(s :&String) -> &str {
  let bytes = s.as_bytes();
  for (i , &itemvalue) in bytes.iter().enumerate() {
      if itemvalue == b' '{
         return &s[..i];
      }
  }
  &s[..]
}

//A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.
//fn first_word(s: &str) -> &str {