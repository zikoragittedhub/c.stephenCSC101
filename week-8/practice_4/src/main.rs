fn main() {

    //name vecto
    let name = vec!["mary","sam","sally","greg","ade","mark","june","ife"];

    //age vector
    let age = vec![16,17,18,19,22,20,21,18,23];

    print!("\nAge allocation:\n");

    //loop to iterate elements in vector 
    for i in 0..age.len()
    {
        //iterating through i on the vector
        print!("{} is {} years old\n",name[i],age[i]); 
    }

}
