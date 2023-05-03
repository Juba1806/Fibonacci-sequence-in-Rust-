use std::io;

fn main() {
    println!("Enter the number of terms in the Fibonacci sequence to generate: ");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse()
        .expect("Please enter a valid number");

    let mut a = 0;
    let mut b = 1;

    for i in 0..n {
        println!("{}: {}", i, a);
        let sum = a + b;
        a = b;
        b = sum;
    }
}

