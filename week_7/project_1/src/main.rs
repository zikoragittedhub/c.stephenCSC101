use std::io;

fn main() {

    println!("Welcome to Zikorella's Mathematics Solutions Program!");
    println!("What would you like find the solution to?");
    println!("
       1) Area of trapezium
       2) Area of Rhombus 
       3) Area of Parallelogram
       4) Area of Cube
       5) Volume of Cylinder ");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("failed to read line");
    let choice: u32 = choice.trim().parse().expect("invalid input");

    if choice == 1 {
        area_of_trapezium();
    } else if choice == 2 {
        area_of_rhombus();
    } else if choice == 3{
        area_of_parallelogram();
    } else if choice == 4{
        area_of_cube();
    } else if choice == 5{
        volume_of_cylinder();
    } else {
        println!("please choose 1-5");
    }
}
                                 
fn area_of_trapezium() {
    // ask measurements
    // calc
    // print result 
    println!("what is the height? ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("failed to read line");
    let height: f32 = height.trim().parse().expect("invalid input"); 

    println!("What is the base 1?");
    let mut base_1 = String::new();
    io::stdin().read_line(&mut base_1).expect("failed to read line");
    let base_1: f32 = base_1.trim().parse().expect("invalid input"); 

    println!("What is the base 2?");
    let mut base_2 = String::new();
    io::stdin().read_line(&mut base_2).expect("failed to read line");
    let base_2: f32 = base_2.trim().parse().expect("invalid input");

    let area = (height * 0.5) *(base_1 * base_2);
    println!("Area of trapezium: {}", area); 
}

fn area_of_rhombus() {
    println!("What is diagonal 1");
    let mut diagonal_1 = String::new();
    io::stdin().read_line(&mut diagonal_1).expect("failed to read line");
    let diagonal_1: f32 = diagonal_1.trim().parse().expect("invalid input"); 

    println!("What is diagonal_2");
    let mut diagonal_2 = String::new();
    io::stdin().read_line(&mut diagonal_2).expect("failed to read line");
    let diagonal_2: f32 = diagonal_2.trim().parse().expect("invalid input");

    let area = 0.5 *(diagonal_1 * diagonal_2);
    println!("area_of_rhombus: {}", area); 
}

fn area_of_parallelogram() {
    println!("What is the base? ");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("failed to read line");
    let base: f32 = base .trim().parse().expect("invalid input"); 

    println!("What is the altitude?");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("failed to read line");
    let altitude: f32 = altitude.trim().parse().expect("invalid input");

    let area = base * altitude;
    println!("Area of Parallelogram: {}", area);
}

fn area_of_cube() {
    println!("what is the length of a side? ");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("failed to read line");
    let length: f32 = length.trim().parse().expect("invalid input"); 

    let area = 6.0 * (length * length);
    println!("Area of cube: {}", area);
}

fn volume_of_cylinder() {
    println!("What is the radius?");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("failed to read line");
    let radius: f32 = radius.trim().parse().expect("invalid input");

    println!("What is the height of the cylinder?"); 
    let mut height_of_cylinder = String::new();
    io::stdin().read_line(&mut height_of_cylinder).expect("failed to read line");
    let height_of_cylinder: f32 = height_of_cylinder.trim().parse().expect("invalid input");

    let volume = (22.0/7.0) * (radius * radius) * height_of_cylinder;
    println!("Volume of Cylinder: {}", volume);
}