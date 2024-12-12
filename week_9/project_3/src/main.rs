use std::io::Read;
fn main() {
    let c = read_line("commissioner.txt");
    let m = read_line("ministry.txt");
    let g = read_line("geopolitical_zone.txt");

    let comissioner:Vec<&str> = c.split("\n").collect();
    let ministry:Vec<&str> = m.split("\n").collect();
    let geopolitical_zone: Vec<&str> = g.split("\n").collect();

    for i in 0..comissioner.len(){
        println!("{},{},{}",comissioner[i].trim(), ministry[i].trim(), geopolitical_zone[i].trim());
    }
}

fn read_line(a: &str) -> String {
    let mut file = std::fs::File::open(a).unwrap();
    let mut context = String::new();
    file.read_to_string(&mut context).unwrap();
    return context.trim().to_string();
}