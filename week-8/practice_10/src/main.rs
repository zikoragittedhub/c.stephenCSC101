fn main() {
    let b:(i32,bool,f64) = (30,true,4.9);
    print(b);
}
 fn print(x:(i32,bool,f64)) {

    println!("inside print method");
    //assign a tuple to distinct varibles
    let (age,is_male,cgpa) = x;
    println!("Age is {}, is male? {}, cgpa {}",age,is_male,cgpa);
 }
