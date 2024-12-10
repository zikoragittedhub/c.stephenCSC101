use std::io;

fn main() {

    println!("Welcome to Ernst & Young Global Limited");
    println!("Please fill out this form");

    println!("how many developer results?");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed response");
    let dev_num:u32 = input1.trim().parse().expect("failed to response");

    let mut developer:Vec<(String,f32)> vec::new();
    for x in 0..dev_num{
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter name of developer:");
    io::stdin().read_line(&mut input2).expect("failed response");
    let name:String = input2.trim().parse().expect("failed to response");

    developer.push((name,years));
} 

    developer.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("");
    println!("developer list:");
    let mut list=1;
    for i in &developer{
        println!("{}. {} {}",list,i.0,i.1);
        list+=1
    } 

    println!("");
    println!("The developer with the highest years of experience is {}");
}      

