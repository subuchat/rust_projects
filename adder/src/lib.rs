#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn do_not_call(&self) {
        panic!("Ohh my God!!");
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration() {
        assert_eq!(2+3 , 5);
    }
    #[test]
    #[should_panic]
    fn larger_can_hold_smaller_test() {
        let larger = Rectangle {
            width : 10,
            height : 6,
        };
        let smaller = Rectangle {
            width : 9,
            height : 4,
        };
        //assert!(larger.can_hold(&smaller));
        assert!(smaller.can_hold(&larger) , "This is expected to FAIL!!!!!!!!!!!"); // this will see as pass as testing -ve case under should panic
    }

    #[test]
    #[should_panic(expected = "Ohh my God")]

    fn test_do_not_call()
    {
        let testrect = Rectangle{
            width : 5,
            height :3,
        };
        testrect.do_not_call();
    }

    #[test]
    #[ignore]
    fn test_without_assert() -> Result<(), String> {  //this test is based on Result, if return Err will be fail
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
 
}
