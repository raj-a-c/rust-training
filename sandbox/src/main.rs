use std::io;

fn main() {
    //prompt user for weight
    println!("Enter a weight:");
    let mut input_weight = String::new();
    let weight_error = String::new("Invalid weight");

    //read theinput
    fn read_string(input_weight: &mut String, weight_error: &mut String);

    let weight: u32 = input_weight
        .trim()
        .parse()
        .expect("Please enter a valid non-negative integer");


    //println!("Enter noradrenaline strength (l = low-strength, h = high-strength):");
    //let mut input_strength = String::new();
    
    //println!("Enter noradrenaline rate in ml/hr:");
    //let mut input_rate = String::new();
}

fn read_string(random_string: &mut String, error_string: &mut String) {
    io::stdin()
        .read_line(random_string)
        .expect(error_string);
}