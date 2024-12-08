fn main() {

    //mutable array
    let mut colours = ["red","green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colours);

    //mutable slice
    let sliced_colours = &mut colours [1..3];

    println!("first slice = {:?}",sliced_colours);

    //change the value of the original slice at the first index
    sliced_colours[1] = "purple";

    println!("changed slice = {:?}", sliced_colours);
}
