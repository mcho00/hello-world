fn main() {
    println!("Hello, world!");
    // 최종적으로 보여주고 싶은거를 fn main()
}

//a
fn fib(k) {
    let mut k: u32 
    if k == 0 {
        println!("0")
    }
    if k == 1 {
        println!("1")
    else {
        fib(k-2)+fib(k-1)
    }
    }

}

for i in 0..=50 {
    k = i;
    println!(k);
    fib(k);


}

//------------------------------------------------

//b-a
let mut F: [u128: 101] = //;
for i in 0..=100 {
    F[i] = i;
    let mut i += 1;
}; 

//b-b
// we can't modify the array type, but how can we replace it?

//--------------------------

//c
use std::io;

let mut input = String::new()