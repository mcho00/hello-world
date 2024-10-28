fn main() {
    let mut F: [u8; 101] = [0; 101]; 
    for i in 0..=100 {
        if i == 0 {
            F[i] = 0;
        }
        else if i == 1 {
            F[i] = 1; 
        }
        else {
            F[i] = F[(i) - 1] + F[(i) - 2];
        }
        println!("{}th number= {}", i, F[i]);
    }
}
    

// we can't modify the array type, but how can we replace it?
// use the same function, but try change to u8; it will cause error

//let mut F: [u128; 101] = //;
//for i in 0..=100 {
//    F[i] = i;
//    println!("ith number=", F[i]);
//    let mut i = i + 1;
//}; 

/*    let mut F: [u128; 101] = [0; 101];
    for i in 0..=100 {
        F[i] = fib(i as u32);
        println!("{}th number= {}", i, F[i]);
        let i = i + 1; // do we need this???
    }; */

// how to use previous indixed to utilize 
//arr = [F0, F1, ...]
//arr[3] = arr[2]+arr[1]


/*    for i in 0..=100 {
        F[i] = fib(i as u8, &mut F);
        // u32 as i, and fib(i) will be converted to u8 as F[i] only allows u8
        println!("{}th number= {}", i, F[i]); */

/*if F[i as usize] != 0 {
        F[i as usize] = F[(i-2) as usize]+F[(i-1) as usize];
    } */

/*if i == 0 {
        F[i as usize] = 0;
        return 0;
    }
    else if i == 1 {
        F[i as usize] = 1;
        return 1;
    } 
    if F[i as usize] != 0 {
        return F[i as usize];
    }
    F[i as usize] = fib(i - 1, &mut F) + fib(i - 2, &mut F);
    return F[i as usize]; 
    */


/*//let mut i: u128; 
    for i in 0..=100 {
        F[i] = fib(i as u8, &mut F);
        // u32 as i, and fib(i) will be converted to u8 as F[i] only allows u8
        println!("{}th number= {}", i, F[i]);
    }
}

//F[i] - F[]
//instead of calling the funciton, 
fn fib(i: u8, F: &mut [u8; 101]) -> u8 {
    let mut F: [u8; 101] = [0; 101]; 
    for i in 0..=100 {
        if i == 0 {
            F[i as usize] = 0;
        }
        else if i == 1 {
            F[i as usize] = 1; 
        }
        else {
            F[i as usize] = F[i as usize - 1] + F[i as usize - 2];
        }
        println!("{}th number= {}", i, F[i]);
    }
} */