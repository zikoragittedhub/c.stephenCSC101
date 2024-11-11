//Rust program that reads the expeience 
//and age of an employee

use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Enter the experience (1 experience, 0 for inexperienced):");
    io::stdin().read_line(&mut experience).expect("failed to read_line");
    let experience: u32 = experience.trim().parse().expect("failed to read");

    println!("Enter your age");
    io::stdin().read_line(&mut age).expect("failed to read");
    let age:u32 = age.trim().parse().expect("faied to read");

    let incentive: u32;

    if experience == 1 {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 {
            incentive = 1_480_000;
        } else {
            incentive = 1_300_000;
        }
    }else {
            incentive = 1_000_000;
    }

    println!("The annual incentive is N{} per month", incentive );

}
