fn main() {

    let fullname = "Chibudem John Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "school of science".to_string();
    //push string
    school.push_str("and technology");

    println!("My name is: {}", fullname);
    //check length
    println!("the length my fullname is : {}",fullname.len());
    println!("i am a student of {} department", department);
    println!("{}",school);
    println!("{}", uni);
}
