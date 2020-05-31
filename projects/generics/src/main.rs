
use std::fmt::Display;
use std::fmt;
use std::string;
//  Shout trait to display both x 
// and y of point
pub trait Shout {
    fn shouting(&self) -> String {
        return format!("No point description");
    }
}
// add traits copy,debug and
//  clone to our struct 
#[derive(Copy,Debug,Clone,)]
struct Point<T,U>
{
    x: T,
    y: U
}
// add method for a point to only 
// recreate itself if it uses the same types
impl <T,U> Point<T,U>{
    fn new(self,x:T , y:U) -> Self {
        Self {x ,y}
    }
}
//  implement Shout for points that
//  have types which implement Display
impl<T:Display,U:Display> Shout for Point<T,U> {
    fn shouting(&self) -> String
        {
        return format!("Point description :: x is {}, y is {}" , (self.x) , (self.y) );
        }
}
// implement the display behaviour for our
//  struct (Point) so that we can pass our 
//  struct to functions which require it
impl <T: Display,U:Display> Display for Point<T,U>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shouting() )
    }
}
// implement an associated function with 
//  Point via Point::find_important
//  as no use of &self is in function signature
impl <T: Display,U: Display> Point<T,U> {
    fn find_important(point: Point<T,U>) -> String{
        let compare_x = format!("{}",point.x);
        let compare_y =  format!("{}",point.y);
        if compare_x >= compare_y {
            format!("The most important part of Point is x :: {}",compare_x);
        }
        format!("The most important part of Point is y :: {}",compare_y)
    }
}
// implement getters for both x and y of 
//  Points
impl<T,U> Point<T,U> {
    fn x(self) -> T {
        self.x
    }
    
    fn y(self) -> U {
        self.y
    }
}
// implement a function that a point
//  may merge with another point
//  of different/same/whatever types
impl <T,U> Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W>{
        Point {
            x: self.x,
            y:other.y
        }
    }
}
// program
fn main() {
    // make some different points to 
    // show off
    // same type point, but mutable
    let mut same_point = Point {
        x:1,
        y:2
    };
    same_point = same_point.new(3,5);
    // point with two numeric types
    let simple_point = Point { 
        x: 1.0 , y: 1 
    };
    // point with negative and positive
    let complex_point = Point {
        x: 10.754 , y: -890 
    };
    // point with char and strings
    let string_point = Point {
        x:'c', y:"hello" 
    };
    // point that combines two 
    // previous points
    let combition_point = 
        string_point.mixup(complex_point);
    //  pass structs to println!
    println!(
    "A same point {}\n,{}",
    same_point,
    Point::find_important(same_point)
    );
    println!(
    "A simple point {}\n{}",
    simple_point,
    Point::find_important(simple_point)
    );
    println!(
    "A combo point {}\n{}",
    combition_point,
    Point::find_important(combition_point)
    );
    println!(
    "A string point {}\n{}",
    string_point,
    Point::find_important(string_point)
    );
    println!(
    "A complex point {}\n{}",
    complex_point,
    Point::find_important(complex_point)
    );
}



