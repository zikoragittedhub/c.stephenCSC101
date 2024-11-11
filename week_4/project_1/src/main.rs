use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a).expect("failed to read");
    let a:f32 = a.trim().parse().expect("failed to read");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b).expect("failed to read");
    let b:f32 = b.trim().parse().expect("failed to read");

    println!("Enter the value for c: ");
    io::stdin().read_line(&mut c).expect("failed to read");
    let c:f32 = c.trim().parse().expect("failed to read");

    let d: f32 = (b * b) - (4.0 * a * c);

    if d > 0.0 {
        let x1 = (-b + (d.sqrt())) / (2.0 * a);
        let x2 = (-b - (d.sqrt())) / (2.0 * a);
        println!("The roots are {} and {}", x1, x2);
    } 
    else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("the root is {}", x);
    }
    else {
        println!("the equation has no real roots");
    }
}
