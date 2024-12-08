fn main() {
    // initalize a mutable tuple 
    let mut mountian_heights = ("everest",8848, "fishtail", 6993);

    println!("orginal tuple = {:?}", mountian_heights);

    //change 3rd and 4th element of a mutable tuple 
    mountian_heights.2 = "Lhotse";
    mountian_heights.3 = 8516;

    println!("Changed tuple = {:?}", mountian_heights);
}
