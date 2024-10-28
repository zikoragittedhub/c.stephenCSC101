fn main(){
	let p:f64 = 210000.0;
	let r:f64 = 5.
	let t:f64 = 3.0;

	//compound interest
	let a = p * (1.0 - (r/100.0)).powf(t);
	println!("amount is {}",a );
	let ci = p - a;
	println!("compound interest is {}", ci);

}