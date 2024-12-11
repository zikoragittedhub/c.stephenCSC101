use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Nigerian Berwery Limited Online Cellar ");

    let mut lager = Vec::new();
    let mut stout = Vec::new();
    let mut non_alcoholic = Vec::new();
    let num_drinks: u32 = read_line("how many drinks do you want?")
    .parse().expect("failed to parse drinks");
    
    for x in 0..num_drinks {

        let selection: u32 = read_line("
        Chose from 1-3
        1. Lager
        2. Stout
        3. Non-Alcoholic").parse().expect("failed to parse");

        let drink_name = read_line("what drink do you want?");

        if selection == 1{
            lager.push(drink_name + "\n");
        }else if selection == 2{
            stout.push(drink_name + "\n");
        }else if selection == 3{
            non_alcoholic.push(drink_name + "\n");
        }else {
            println!("Invalid selection, please try again!");
        }

        
    }

    let mut lager_file = std::fs::File::create("lager.txt").expect("create failed");   
    for drink in lager.iter() {
        lager_file.write_all(drink.as_bytes()).expect("failed to write file");
    }

    let mut stout_file = std::fs::File::create("stout.txt").expect("create failed");
    for drink in stout.iter(){
        stout_file.write_all(drink.as_bytes()).expect("failed to write file");
    }

    let mut non_alcoholic_file = std::fs::File::create("non_alcoholic.txt").expect("create failed");
    for drink in non_alcoholic.iter(){
        non_alcoholic_file.write_all(drink.as_bytes()).expect("failed to write file");
    }
}

fn read_line(hint: &str) -> String {
    println!("{}", hint);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}
