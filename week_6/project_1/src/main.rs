use std::io;

fn main() {
    let mut total_price = 0.0;
    loop {
        println!("Welcome to zikorella's kitchen");
        println!("list of our food menu");
        println!("P = poundo yam/ediikaiko soup");
        println!("F = fried rice & chicken");
        println!("A = amala & ewedu soup");
        println!("E = eba & egusi soup");
        println!("W = white rice & stew");


        println!("Enter food from menu (type 'finished' when done ordering)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let item = input.trim();

        if item == "finished" {
            break;
        }

        println!("You picked {}",item);

        let mut price = 0;

        if item =="P"{
            price = 3200;
        }else if item == "F"{
            price = 3000;
        }else if item == "A"{
            price = 2500;
        }else if item == "E"{
            price = 2000;
        }else if item == "W"{
            price = 2500;
        }

        println!("Enter quantity needed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let quantity_needed:i32 = input.trim().parse().expect("please enter valid input");

        let sub_total_price = price * quantity_needed;
        total_price += sub_total_price as f32;
    }

    if total_price > 10_000.0 {
        total_price = (total_price as f32 * 0.95); //applying 5% discount 
        println!("A discount has been added to your order");
    }
    //Print the total cost 
    println!("the total cost of your order is: N{}", total_price);
    println!("thanks for coming ");
}
