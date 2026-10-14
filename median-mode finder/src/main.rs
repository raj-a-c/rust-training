use std::collections::HashMap;

fn main() {
    let list: Vec<i32> = vec![1, 4, 5, 6, 6, 7];
    //MODE
    let map: HashMap<i32, i32> = HashMap::new();

    mode(map, &list);
    median(list);
}

fn median(mut list: Vec<i32>) {
    
    //will need the list length to determine if odd or even
    list.sort();

    let int_length = list.len();
    let f_length = int_length as f64;

    let half = int_length / 2;

    if &f_length % 2.0 == 0.0 {

        let median: f64 = (list[half]  + list[half - 1]) as f64 / 2.0;

        println!("The median is {median}.");

    } else {
        println!("The median is {}.", list[half]);
    }

}

fn mode(mut map: HashMap<i32, i32>, list: &[i32]) {
    let mut highest = 0; //highest number of occurrences of a given value

    //Iterates of over the list of numbers
    //Allocates each number as a key.
    //Then allocates number of times each number occurs as a value in the key:value
    for &value in list {
        let count = map.entry(value).or_insert(0);
        *count += 1;

        //updates "highest"
        if *count > highest {
            highest = *count;
        }
    }
    
    //can be more than one mode
    //create a empty vector to add to with each mode
    let mut mode: Vec<i32> = Vec::new();

    //find all the keys(a number from the original list) which are modes
    //the value in the key: value pair will == highest
    //add them to the "mode" vector
    for (key, value) in &map {
        if *value == highest {
            mode.push(*key);
        }
    }

    //find the length of "mode"
    //different outputs depending on the number of modes
    let no_of_modes = mode.len();

    
    if no_of_modes == 0 { //if there is no mode, an error has occure
        println!("Error!");
    } else if no_of_modes == 1 { //if there is 1 mode, use the singular
        println!("The mode is {}.", mode[0]);
    } else { //if there are multiple modes, print a statement followed by a vertical list
        println!("The modes are:");

        let mut i = 0;
        while i < no_of_modes {
            println!("{}", mode[i]);
            i += 1;
        }
        //was too lazy to make the output look prettier for the multiple modes output
    }
    
}
