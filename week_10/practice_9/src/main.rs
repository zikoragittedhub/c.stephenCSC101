//define dimensions of a rectangle
struct Rectangle {
    width:u32,
    height:u32
}
//logic to calculate area of a rectangle 
impl Rectangle{
    fn area(&self)->u32 {
        //use the . operator ro fetch the value of a field via the sllf keynote
        self.width * self.height
    }
}

fn main() {
    //instanatiate the structure 
    let small = Rectangle {
        width:10,
        height:20
    };
    //print the rectangke"s area
    println!("width is {} /n height is {} \n area of Rectangle is {}",small.width,small.height,small.area());
}
