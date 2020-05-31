
use std::fmt::Display;
use std::fmt;
use std::string;

pub trait Shout {
    fn shouting(&self) -> String;
}
#[derive(Copy,Debug,Clone)]
struct Point<T,U>
{
    x: T,
    y: U
}

impl <T,U> Point<T,U>{
    fn new(self,x:T , y:U) -> Self {
        Self {x ,y}
    }
}

impl<T: std::fmt::Display,U: std::fmt::Display> Shout for Point<T,U> {
    fn shouting(&self) -> String
        {
        return format!("Point description :: x is {}, y is {}" , (self.x) , (self.y) );
    }
}

impl <T: std::fmt::Display,U: std::fmt::Display> Display for Point<T,U>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shouting() )
    }
}

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

impl<T,U> Point<T,U> {
    fn x(self) -> T {
        self.x
    }
    
    fn y(self) -> U {
        self.y
    }
}

impl <T,U> Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W>{
        Point {
            x: self.x,
            y:other.y
        }
    }
}

fn main() {

    let mut same_point = Point {
        x:1,
        y:2
    };
    same_point = same_point.new(3,5);

    let simple_point = Point { 
        x: 1.0 , y: 1 
    };
    let complex_point = Point {
        x: 10.754 , y: -890 
    };
    let string_point = Point {
        x:'c', y:"hello" 
    };
    let combition_point = 
        string_point.mixup(complex_point);

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



