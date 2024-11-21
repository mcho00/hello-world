use std::io;

fn main() {
    let mut input = String::new();
    println!("What's your k?");
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    //let mut k: u32;
    let input = input.trim();
    let number: u32 = input.parse().expect("Not a good number!"); 
    let k: u32 = number;
    let mut sum: u32 = 0;
    for i in 1..=k {
        sum += (i as u32).pow(2);
        println!("{}th sum= {}", i, sum);  
    }
}

/*  let mut input = String::new();
    let mut k: u32;
    let mut sum = 0 as u32;
    for i in 1..k {
        sum += i.pow(2);
        println!("{}th sum=", sum); 
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    let input = input.trim(); 
    let number: u32 = input.parse().expect("Not a good number!");     
    } */