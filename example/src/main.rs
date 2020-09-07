use healthcard_rust::*;

fn main() {
    println!("Read data from German public health insurance cards (eGK)");
    let card = get_card();
    println!("Generation {:?}", healthcard_rust::get_card_generation(&card));
    println!("{:?}", healthcard_rust::get_card_data(&card).to_string())
}
