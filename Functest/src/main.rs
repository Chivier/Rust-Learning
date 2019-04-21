#![allow(non_snake_case)]
fn main() {
    let mut x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    if x==5 {
        x = x + 1;
    }

    x = x+1;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}