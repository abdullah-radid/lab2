
//Create a triangle struct. 
struct Rectangle {
    width: f64,
    height: f64,
    
}   

//implementation of rust
impl Rectangle {
    //basically the rust equivalent of a constructor, initializes the Rectangle struct
    //called associated functions
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
        }

        //method to calculate area, returns as f64 type. uses self to initialize the current struct
        fn area(&self) -> f64 {
            return self.width *self.height;
        }

        //same as area method, but for perimeter
        fn perimeter(&self) -> f64 {
            return 2.0 * (self.width + self.height);
        }

        //same as previous, uses if statements to check, returns boolean
        fn is_square(&self) -> bool {
            if self.width == self.height {
                true
            } else {
                false
            }
        }
    }


//Create a circle struct with radius as its field being a f64 type (f64 used for floating nums)
struct Circle {
    radius: f64, // assume radius > 0
}

//implementation of Circle
impl Circle {

    //assorted function, constructor to initialize the circle struct with radius as a param
    pub fn new(radius: f64) -> Circle {
        Circle {
            radius: radius,
        }
    }
 
    //func to calculate circle area. uses &self to point to memory of current instance
    pub fn circle_area(&self) -> f64 {
        return 3.14 * (self.radius * self.radius);
    }

    //same as circle area, but for circumference.
    //i casted 2 as an f64 since it was an integer before, but i could have just done 2.0
    pub fn circle_circumference(&self) -> f64 {
        return (2 as f64) * 3.14 * self.radius;
    }

}

fn main() {
    //initializes a rect struct with correct parameters
    let rect: Rectangle = Rectangle::new(10.0, 5.0);
    //These 3 tests call on our rect instance and call the specific methods
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());
    //test conditions at runtime
    //since program runs, we should be correct on this tests
    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());


    //instance of circle struct
    let circ: Circle = Circle::new(9.3);

    //prints out area and circum of current instance of circle called circ
    println!("Circle Area: {}", circ.circle_area());
    println!("Circle Circumference: {}", circ.circle_circumference());

}