
fn largest ( mylist : &[i32]) -> i32 {
//fn largest ( mylist : &Vec<i32>) -> i32 { // just to check that vector converting to array in above func!! suprise!!
    let mut larg = mylist[0];

    for &item in mylist {
        if item > larg {
            larg = item;
        }
    }

    larg
}
// Generic implementation
struct Point<T> {
    x_coordinate : T,
    y_coordinate : T,
}

impl<T> Point<T> {
    fn x_val(&self) -> &T{
        &self.x_coordinate
    }
}
//similar to templete specialization of Cpp

impl Point<f32>{
    fn distance_from_origin(&self) ->f32{
        (self.x_coordinate.powi(2) + self.y_coordinate.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34,50,12,44,99,11];
    
    let result = largest(&number_list);
    println!("Largest number of the vectore is : {}" , result);

    let mut myarr = [1,2,3,4,5,6];
    let reslt = largest(&myarr);
    println!("Largest number of the array is : {}" , reslt);

    let float_coordinate = Point {x_coordinate: 5.0, y_coordinate: 60.0};
    println!("Value of x_coordinate of the point is {} : ", float_coordinate.x_val());
    println!("Distance from origin : {}" , float_coordinate.distance_from_origin());


}
