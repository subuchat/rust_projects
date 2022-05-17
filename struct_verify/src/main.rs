#[derive(Debug)]
struct Rectangle {
    height:u32,
    width :u32,
}

impl Rectangle {
    fn Area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct User {
    active : bool,
    username : String,
    email  : String,
    sign_in_count : u32,
}


fn main() {
    println!("Structure examples!");
   let rect1 = Rectangle {
       height : dbg!(30),
       width  : 50,
   };
   println!("Area of rect1 : {} " , areacal(&rect1));
   println!("Rectabgle values are : ");
   dbg!(&rect1);

   println!("Area of the rectangle using struct method : {}" , rect1.Area());

   let user1 = User {
       active : true,
       email : String::from("someone@gmail.com"),
       username : String::from("someone"),
       sign_in_count : 1,
   };

   println!("User1 is : {:?}" , user1);

   let rect2 = Rectangle {
    width: 40,
    height: 50,
   };
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect2));
}
 


fn areacal(rectangle : &Rectangle) -> u32 {
   rectangle.height * rectangle.width
}
