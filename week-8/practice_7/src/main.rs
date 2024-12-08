fn main() {
    //intialization  of tuple with data type 
    let datatype_tuple: (&str, f32, u8) = ("rust",3.14, 100);
    println!("tuple contents = {:?}", datatype_tuple);

    //intilization of tuple without data type
    let no_datatype_tuple = ("rust", "fun", 100);
    println!("tuple contents = {:?}", no_datatype_tuple);

    //accessing tuple element at index 0 
    println!("value at index 0 = {}", no_datatype_tuple.0);

    //accessing tuple element at index 1
    println!("value at index 1 = {}", no_datatype_tuple.1);

    //accessing tuple element at index 2
    println!("value at index 2 = {}", no_datatype_tuple.2);
}
