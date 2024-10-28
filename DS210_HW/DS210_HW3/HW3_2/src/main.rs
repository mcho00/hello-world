use std::f64::consts::PI;

fn main() {
    let hexagon = Polygon{
        len: 15.0,
        sides: 6,
    };
    println!("Hexagon:");
    println!("Perimeter: {:.2}", hexagon.perimeter());
    println!("Area: {:.2}", hexagon.area());
    println!("Radius: {:.4}", hexagon.radius());
    println!("Apothem: {:.4}", hexagon.apothem());
    println!("Circle Area: {:.4}", hexagon.circle_area());

    let _12 = Polygon{
        len: 15.0,
        sides: 12,
    };
    println!("\nSides of 12:");
    println!("Perimeter: {:.2}", _12.perimeter());
    println!("Area: {:.2}", _12.area());
    println!("Radius: {:.4}", _12.radius());
    println!("Apothem: {:.4}", _12.apothem());
    println!("Circle Area: {:.4}", _12.circle_area());

    let _24 = Polygon{
        len: 15.0,
        sides: 24,
    };
    println!("\nSides of 24:");
    println!("Perimeter: {:.2}", _24.perimeter());
    println!("Area: {:.2}", _24.area());
    println!("Radius: {:.4}", _24.radius());
    println!("Apothem: {:.4}", _24.apothem());
    println!("Circle Area: {:.4}", _24.circle_area());

    let _128 = Polygon{
        len: 15.0,
        sides: 128,
    };
    println!("\nSides of 128:");
    println!("Perimeter: {:.2}", _128.perimeter());
    println!("Area: {:.2}", _128.area());
    println!("Radius: {:.4}", _128.radius());
    println!("Apothem: {:.4}", _128.apothem());
    println!("Circle Area: {:.4}", _128.circle_area());

    let _256 = Polygon{
        len: 15.0,
        sides: 256,
    };
    println!("\nSides of 256:");
    println!("Perimeter: {:.2}", _256.perimeter());
    println!("Area: {:.2}", _256.area());
    println!("Radius: {:.4}", _256.radius());
    println!("Apothem: {:.4}", _256.apothem());
    println!("Circle Area: {:.4}", _256.circle_area());

    let _512 = Polygon{
        len: 15.0,
        sides: 512,
    };
    println!("\nSides of 512:");
    println!("Perimeter: {:.2}", _512.perimeter());
    println!("Area: {:.2}", _512.area());
    println!("Radius: {:.4}", _512.radius());
    println!("Apothem: {:.4}", _512.apothem());
    println!("Circle Area: {:.4}", _512.circle_area());

    let _1024 = Polygon{
        len: 15.0,
        sides: 1024,
    };
    println!("\nSides of 1024:");
    println!("Perimeter: {:.2}", _1024.perimeter());
    println!("Area: {:.2}", _1024.area());
    println!("Radius: {:.4}", _1024.radius());
    println!("Apothem: {:.4}", _1024.apothem());
    println!("Circle Area: {:.4}", _1024.circle_area());

    let _2048 = Polygon{
        len: 15.0,
        sides: 2048,
    };
    println!("\nSides of 2048:");
    println!("Perimeter: {:.2}", _2048.perimeter());
    println!("Area: {:.2}", _2048.area());
    println!("Radius: {:.4}", _2048.radius());
    println!("Apothem: {:.4}", _2048.apothem());
    println!("Circle Area: {:.4}", _2048.circle_area());

    let _65536 = Polygon{
        len: 15.0,
        sides: 65536,
    };
    println!("\nSides of 65536:");
    println!("Perimeter: {:.2}", _65536.perimeter());
    println!("Area: {:.2}", _65536.area());
    println!("Radius: {:.4}", _65536.radius());
    println!("Apothem: {:.4}", _65536.apothem());
    println!("Circle Area: {:.4}", _65536.circle_area());
}

struct Polygon {
    len: f64,
    sides: u32,
}

trait Equations {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64; 
    fn apothem(&self) -> f64;
    fn circle_area(&self) -> f64; 
}

impl Equations for Polygon {
    fn perimeter(&self) -> f64 {
        (self.sides as f64) * self.len
    }
    fn area(&self) -> f64 {
        self.perimeter() * self.apothem() / 2.0
    }
    fn radius(&self) -> f64 {
        let angle = PI / (self.sides as f64);
        self.len / (2.0 * (angle.sin()))
    }
    fn apothem(&self) -> f64 {
        //let angle = PI / (self.sides as f64);
        self.len / (2.0 * ((PI/(self.sides as f64)).tan()))
    }
    fn circle_area(&self) -> f64 {
        self.radius().powf(2.0) * PI
    }
}

/*impl Equations for Polygon {
    fn perimeter(&self) -> f64 {
        let self.peri = (sides as f64) * len;
        self.peri
    }
    fn area(&self) -> f64 {
        let self.area = ((sides as f64) * len * apothem) / 2;
        self.area
    }
    fn radius(&self) -> f64 {
        let angle = PI / (sides as f64);
        let self.radius = len / (2 * angle.sin());
        self.radius
    }
    fn apothem(&self) -> f64 {
        let self.apothem = len / 2 * (180/((sides as f64).tan()));
        self.apothem
    }
}*/

/*trait Equations {
    let sides as f64;
    fn perimeter(&self) -> f64 {
        (sides as f64) * len
    } //sum all lengths on side
    fn area(&self) -> f64 {
        /*if sides == 3 {
            let mut semi_per = (len * 3) / 2;
            (semi_peri)*(semi_peri - len).powf(3).sqrt()
        }
        if sides == 4 {
            len * len*/
        }
    }
    fn radius(&self) -> f64 {

    }
    fn apothem(&self) -> f64 {
        //Apothem = [(length of one side)/{2 ×(tan(180/number of sides))}]
        
    }
 let square = Polygon{
        len: 15.0,
        sides: 4,
    };
    println!("Square:");
    println!("Perimeter: {:.2}", square.perimeter());
    println!("Area: {:.2}", square.area());
    println!("Radius: {:.2}", square.radius());
    println!("Apothem: {:.2}", square.apothem());
}*/