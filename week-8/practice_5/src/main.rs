fn main() {
    //create an empty vector "city"
    let mut city : Vec<String> = Vec::new();
    println!("the city vector has element {}",city.len());
    //push new elements into 
    let mut input1 = String::new();
    println!("how many cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("failed to read line");
    let city_num:i32 = input1.trim().parse().expect("invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("enter city {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("failed to read input");
        let new_city:String = input2.trim().parse().expect("invalid input");
        city.push(new_city); 
    }
    print!("your preferred cities are:\n");
    let mut count=1;
    //loop to iterare elements in vector 
    for i in city
    {
        //iterating through 1 on the vector 
        println!("{} {}",count, i);
        count+=1;
    }
}
