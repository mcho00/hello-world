use std::ops::Neg;

fn main() {
    println!("Hello, world!");
    let point_f64 = Point { x: 2.0, y: 4.0 };
    let point_i32 = Point { x: 3, y: 6 };

    let rotated_clockwise_f64 = point_f64.clockwise();
    println!("Clockwise - point f64: ({}, {})", rotated_clockwise_f64.x, rotated_clockwise_f64.y);

    let rotated_counterclockwise_i32 = point_i32.counterclockwise();
    println!("Counterclockwise - point i32: ({}, {})", rotated_counterclockwise_i32.x, rotated_counterclockwise_i32.y);
    
}

struct Point<T> {
    x: T,
    y: T, 
}

/*trait Rotation {
    fn clockwise(&self) -> Self;
    fn counterclockwise(&self) -> Self;
}*/
//Should I have to use Vector for this question's input?
//ex. (3,1) clockwise 90deg will be 1, -3 // 180deg will be -3, -1 // 270deg will be -1, 3 // 360deg will come back
// (3, 1) counterclockwise 90deg will be -1, 1 // 180deg will be -1, -1 // 270deg will be 1, -1 // 360deg will come back
//Which one should I have to use?


impl<T:Copy + Neg<Output = T>> Point<T> {
    fn clockwise(&self) -> Point<T> {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
    fn counterclockwise(&self) -> Point<T> {
        Point{
            x: -self.y,
            y: self.x,
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        println!("This is a test for clockwise.");
        let point_f64 = Point { x: 2.0, y: 4.0 };
        let rotated_clockwise = point_f64.clockwise();
        println!("Clockwise rotation of f64: {}, {}",
        rotated_clockwise.x, rotated_clockwise.y);
        assert_eq!(rotated_clockwise.x, 4.0);
        assert_eq!(rotated_clockwise.y, -2.0);

        println!("This is a test for counterclockwise.");
        let point_i32 = Point { x: 3, y: 6 };
        let rotated_counterclockwise = point_i32.counterclockwise();
        println!("Counterclockwise rotation of i32 point: {}, {}",
        rotated_counterclockwise.x, rotated_counterclockwise.y);
        assert_eq!(rotated_counterclockwise.x, -6);
        assert_eq!(rotated_counterclockwise.y, 3);
    }
}



/*impl Rotation for Point<T> {
    fn clockwise(&self) {
        //
    }
    fn counterclockwise(&self) {
        //
    }
}*/

/*impl Rotation for Point {
    fn clockwise(&self) -> {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
    fn counterclockwise(&self) -> {
        Point{
            x: -self.y
            y: self.x
        }
    }
}*/