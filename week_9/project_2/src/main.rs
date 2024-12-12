use std::io;
use std::io::Read;

fn main() {
    println!("PAU Student Management Information System")

    let name = Vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matrix_num = Vec!["ACC1011111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = Vec!["Accounting","Economics","Computer Science","Electrical Engineering","Mechanical Engineering"];
    let level = Vec!["300","100","200","200","100"];

    let mut file = std::fs::File::create("SMIS.txt").expect("create failed");

    for x in 0..name.len() {
        println!("");
        let line = format!("{},{},{},{}\n",name[1],matrix_num[1],department[1],level[1]);
        file.write_all(line.as_bytes()).expect("failed to write");
    }
    println!("Sucessfully done!");
}
