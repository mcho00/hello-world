
use std::time::SystemTime; 

fn main() {
    //1-b
    for k in 0..=50 {
        println!("k is {}", k);
        println!("fib(k) is {}", fib(k));
        let before = SystemTime::now();
        let after = SystemTime::now(); 
        let difference = after.duration_since(before); 
        let difference = difference.expect("Did the clock go back?"); 
        println!("Time it took: {:?}", difference); 
        }
}
//1-a: make function of fib()
fn fib(k: u32) -> u128 { 
    if k == 0 {
        return 0;
    }
    else if k == 1 {
        return 1;
    } 
    else {
        return fib(k-2) + fib(k-1);
    }
}
//2-a)// 최종적으로 보여주고 싶은거를 fn main(); function can be made outside
    



//1-b


//------------------------------------------------

//2-a


//b-b

//--------------------------

//3

//let mut input = String::new()

