fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Altantic University";
    let addr:&str = "Km 52 lekki-Epe Expressway, Ibeju-Lekki";
    println!("Name: {}",name);
    println!("University: {}, \nAddress: {}", uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
