fn main() {

    let b: (i32, bool,f64) = (110,true,10.0);
    print(b);
}

//pass the tuple as a parameter

fn print(x:(i32,bool,f64)){

    println!("inside print method");
    println!("{:?}",x);
}
