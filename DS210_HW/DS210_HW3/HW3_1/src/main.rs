//HW3_1
//use std::f64::sqrt;
use std::f64::consts::PI;

fn main() {
    let triangle_ex = vec![3.0, 4.0, 3.0];
    let triangle_inval = vec![3.0, 4.0, 15.0];
    let rectangle_ex = vec![5.0, 9.0];
    let rectangle_inval = vec![-3.0, 5.0];
    let circle_ex = vec![3.0];
    let circle_inval = vec![-9.0];
    
    let triangle = input_shape(&triangle_ex).unwrap();
    let invalid_triangle = input_shape(&triangle_inval).unwrap();
    let rectangle = input_shape(&rectangle_ex).unwrap();
    let invalid_rectangle = input_shape(&rectangle_inval).unwrap();
    let circle = input_shape(&circle_ex).unwrap();
    let invalid_circle = input_shape(&circle_inval).unwrap();

    println!("Valid Triangle?: {}", triangle.validity());
    println!("Valid Triangle?: {}", invalid_triangle.validity());
    println!("Valid Rectangle?: {}", rectangle.validity());
    println!("Valid Rectangle?: {}", invalid_rectangle.validity());
    println!("Valid Circle?: {}", circle.validity());
    println!("Valid Circle?: {}", invalid_circle.validity());

    println!("The perimenter of Traingle?: {:.2}", triangle.perimeter());
    println!("The perimenter of Traingle?: {:.2}", invalid_triangle.perimeter());
    println!("The perimenter of Rectangle?: {:.2}", rectangle.perimeter());
    println!("The perimenter of Rectangle?: {:.2}", invalid_rectangle.perimeter());
    println!("The perimenter of Circle?: {:.2}", circle.perimeter());
    println!("The perimenter of Circle?: {:.2}", invalid_circle.perimeter());

    println!("The area of Traingle?: {:.2}", triangle.area());
    println!("The area of Traingle?: {:.2}", invalid_triangle.area());
    println!("The area of Rectangle?: {:.2}", rectangle.area());
    println!("The area of Rectangle?: {:.2}", invalid_rectangle.area());
    println!("The area of Circle?: {:.2}", circle.area());
    println!("The area of Circle?: {:.2}", invalid_circle.area());

    println!("Doubled perimeter of Triangle?: {:.2}", triangle.double_peri().perimeter());
    println!("Doubled perimeter of Triangle?: {:.2}", invalid_triangle.double_peri().perimeter());
    println!("Doubled perimeter of Rectangle?: {:.2}", rectangle.double_peri().perimeter());
    println!("Doubled perimeter of Rectangle?: {:.2}", invalid_rectangle.double_peri().perimeter());
    println!("Doubled perimeter of Circle?: {:.2}", circle.double_peri().perimeter());
    println!("Doubled perimeter of Circle?: {:.2}", invalid_circle.double_peri().perimeter());

}

fn input_shape(input: &[f64]) -> Result<Shape, String> {
    match input.len() {
        3 => Ok(Shape::Triangle(input[0], input[1], input[2])),
        2 => Ok(Shape::Rectangle(input[0], input[1])),
        1 => Ok(Shape::Circle(input[0])),
        _ => Err("Only accept maximum three sides.".to_string()),
    }
}

enum Shape {
    Triangle(f64, f64, f64),
    Rectangle(f64, f64), // len, wid
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Triangle(a, b, c) => {
                let semi_peri = (a + b + c) / 2.0;
                ((semi_peri)*(semi_peri - a)*(semi_peri - b)*(semi_peri - c)).sqrt()
            },
            Shape::Rectangle(len, wid) => len * wid,
            Shape::Circle(rad) => PI * rad.powf(2.0),
        }
    }
    fn perimeter(&self) -> f64 {
        match *self {
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Rectangle(len, wid) => 2.0 * (len + wid),
            Shape::Circle(rad) => 2.0 * PI * rad,
        }
    }
    fn double_peri(&self) -> Shape {
        match *self {
            Shape::Triangle(a, b, c) => Shape::Triangle(2.0 * a, 2.0 * b, 2.0 * c),
            Shape::Rectangle(len, wid) => Shape::Rectangle(2.0 * len, 2.0 * wid),
            Shape::Circle(rad) => Shape::Circle(2.0 * rad),
        }
    }
    fn validity(&self) -> bool {
        match *self {
            Shape::Triangle(a, b, c) => {
                ((a > 0.0) && (b > 0.0) && (c > 0.0)) && ((a + b > c) && (b + c > a) && (c + a > b))
            },
            Shape::Rectangle(len, wid) => {
                ((len > 0.0) && (wid > 0.0)) && (len != wid)
            },
            Shape::Circle(rad) => {
                ((rad > 0.0))
            },
        }
    }
}


//aux function: pass the function array of the argument
//arr: &[f64] // len of the array  //match len (if it's 3 --> triangle(array[0], array[1]); if it's 2 --> rect; if it's 1 --> circle)
//initialize -> Shape::Triangle(a, b, c) --> I need to index the lengths to the array input
/*enum Equation {
    Area,
    Perimeter,
    Double, 
    Correctness,
}*/
//single function with several methods
//put all the things in the main (or)

//OLD
/*fn Calculate(shape: &Shape, formula: &Equation) -> f64 {
    match (shape, formula) {
        (Shape::Triangle(a, b, c), Equation::Area) => {
            let semi_peri = (a + b + c) / 2.0;
            ((semi_peri)*(semi_peri - a)*(semi_peri - b)*(semi_peri - c)).sqrt()
        }
        (Shape::Triangle(a, b, c), Equation::Perimeter) => {
            a + b + c
        }
        (Shape::Triangle(a, b, c), Equation::)
        (Shape::Rectangle(len, wid), Equation::Area) => {
            len * wid
        }
        (Shape::Rectangle(len, wid), Equation::Perimeter) => {
            2.0 * (len + wid)
        }
        (Shape::Circle(rad), Equation::Area) => {
            let pi = std::f64::consts::PI;
            pi * rad.powf(2.0)
        }
        (Shape::Circle(rad), Equation::Perimeter) => {
            let pi = std::f64::consts::PI;
            2.0 * rad * pi
        }
    }
}*/

/*fn area -> f64 {
    match {
        Shape::Triangle(a, b, c) => {
            let semi_peri = (a + b + c) / 2.0;
            let tri_area = 
        }
        Shape::Rectangle(len, wid) => {
            ...
        }
        Shape::Circle(rad) => {
            ...
        }
    }
}*/

/*if (a < 0.0 || b < 0.0 || c < 0.0) && (a + b < c | b + c < a | c + a < b){
                    false
                },*/
                /*else {
                    true
                }
                if a + b < c | b + c < a | c + a < b {
                    false
                },
                else {
                    true
                }*/

/*if (len < 0.0 || wid < 0.0) && len == wid {
                    false
                }
                else {
                    true
                }*/
                /*if len = wid {
                    false
                }
                else {
                    true
                }*/


/*fn validity(self) -> bool {
        match self {
            Shape::Triangle(a, b, c) => {
                ((a > 0.0) && (b > 0.0) && (c > 0.0)) && ((a + b > c) && (b + c > a) && (c + a > b))
            }
            Shape::Rectangle(len, wid) => {
                ((len > 0.0) && (wid > 0.0)) && (len != wid)
            }
            Shape::Circle(rad) => {
                ((rad > 0.0))
                if rad < 0.0 {
                    false
                }
                else {
                    true
                }*/