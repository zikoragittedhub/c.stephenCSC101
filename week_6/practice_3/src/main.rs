fn main() {
    let name1 = "Ayomide Adesakon";
    println!("My name is {}",name1);

    //find and replace
    let name2 = name1.replace("Ayomide", "Adebare");
    println!("you can also call me {}",name2);
    let faculty = "faculty of Science and Tecchnology";

    //find and replace
    let school = faculty.replace("faculty","school");
    println!("I am a student of the {}", school);
}
