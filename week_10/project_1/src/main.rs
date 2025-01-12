use std::io;

struct electronics{
    name: String,
    price:f32,
    quantity:u32
}

fn main() {
    let mut Records = vec![
    
        Electronics{
        name:"HP laptop".to_string(),
        price: 650_000.00,
        quantity:10
      },
      Electronics{
        name:"IBM laptop".to_string(),
        price: 755_000.00,
        quantity:6
      }
      Electronics{
        name:"Toshiba laptop".to_string(),
        price: 550_000.00,
        quantity:10
      }
      Electronics{
        name:"Dell laptop".to_string(),
        price: 850_000.00,
        quantity:4
      }
    ];

    let mut checklist: Vec<Electronics> =Vec::new();
    println!("Welcome to Ogbeifuna Electronics Store");

    loop{
        let selection: u32 = read_line(
            "Please enter any option from 1-4
              1) Add an electronics to Checklist
              2) Remove an electronics from checklist
              3) List of all electronics
              4) Checkout 
              ",).parse().expect("invalid option");
        match selection {
            1=> add_to_checklist(&mut Records, &mut checklist),
            2=> remove_from_checklist(&mut Records),
            3=> list_of_electronics(&Records),
            4=>{
                checkout(&mut checklist);
                break;
            }
            _=> println!("invalid option")
        }
    }
}

fn checkout (checklist: &mut Vec<Electronics>){
    let mut total = 0.0;
    for electronics in checklist.iter() {
        total += electronics.price * electronics.quantity as f32;
    }
    list_of_electronics(checkout);
    println!("Total: ${:.2}", total);
}

fn remove_from_checklist(records: &mut Vec<Electronics>, checklist: &mut Vec<Electronics>) {
    println!("Your checklist:");
    list_of_electronics(checklist);
    let selection: u32 = read_line("Enter the electronic number to remove:").parse().unwrap();
    let idx = selection as usize - 1;
    if idx >= checklist.len() {
        println!("Invalid selection");
        return;
    }
    let electronics = &mut checklist[idx];
    let quantity: u32 = read_line(
        format!("Please select quantity of {} to remove from your checklist. Available units: {}",
            electronics.name, electronics.quantity).as_str(),).parse().unwrap();
    if quantity > electronics.quantity {
        println!("Not enough units in your electronics");
        return;
    }

    electronics.quantity -= quantity;
    for chlist_electronics in checklist {
        if electronics.name == chlist_electronics.name {
            chlist_electronics.quantity += quantity;
            break;
        }
    }
    println!("{} units of {} removed from your checklist",
        quantity, electronics.name);
}

fn add_to_checklist(checklist: &mut Vec<electronics>, checklist: &mut Vec<electronics>) {
    println!("Please select an electronic to add to your cart:");
    list_of_electronics(checklist);
    let selection: u32 = read_line("Choose 1-4").parse().unwrap();
    let idx = selection as usize - 1;
    if idx >= checklist.len() {
        println!("Invalid selection");
        return;
    }
    let electronics = &mut records[idx];
    let quantity: u32 = read_line(
        format!(
            "Please select quantity of {} to add to your checklist. Available units: {}",
            electronics.name, electronics.quantity
        ).as_str(),
    ).parse().unwrap();

    if quantity > electronics.quantity {
        println!("Not enough units available");
        return;
    }
    electronics.quantity -= quantity;
    checklist.push(electronics {
        name: electronics.name.clone(),
        price: electronics.price,
        quantity,
    });
    println!("{} units of {} added to your ", quantity, electronics.name);
}

fn list_of_electronics(Records: &Vec<electronics>) {
    let mut i = 1;
    for electronics in Records {
        println!(
            "{}. {} - ${:.2} - {}",
            i, electronics.name, electronics.price, electronics.quantity
        );
        i += 1;
    }
}

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}