use std::io;
fn main() {
	println!("what is your name? ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("failed to read line");
	let name: String = input1.trim().parse().expect("failed to read");

	println!("what is your date of birth ");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("failed to read line");
	let birthday : i32 = input2.trim().parse().expect("failed to read");

	println!("what is your email address? ");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("failed to read line");
	let email address: String = input3.trim().parse().expect("failed to read");

	println!("what is your phone number? ");
	let mut input4 = String::new();
	io::stdin().read_line(&mut input4).expect("failed to read line");
	let phone number: i32 = input4.trim().parse().expect("failed to read");

	println!("how many siblings do you have? ");
	let mut input5 = String::new();
	io::stdin().read_line(&mut input5).expect("failed to read line");
	let number of siblings: i32 = input5.trim().parse().expect("failed to read");

	println!("how many children do have? ");
	let mut input5 = String::new();
	io::stdin().read_line(&mut input6).expect("failed to read line");
	let number of children: i32 = input6.trim().parse().expect("failed to read");

	println!("what is your medical diagnosis? ");
	let mut input7 = String::new();
	io::stdin().read_line(&mut input7).expect("failed to read line");
	let medical diagnosis: String = input7.trim().parse().expect("failed to read");

	println!("what is your village recidence ");
	let mut input8 = String::new();
	io::stdin().read_line(&mut input8).expect("failed to read line");
	let village residence: String = input8.trim().parse().expect("failed to read");

	patient information = (input1, input2 , input3 , input4 , input5 , input6 , input7 , input8);
	if alzheimer == "yes"{
		println!("patient information and 20% discount ");
	}else if >50{
		println!("patient information and 20% discount");
	}else if lives in akpabom village{
		println!("patient information and 20% discount");
	}



} 