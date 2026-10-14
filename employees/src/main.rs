use std::io;
use std::collections::HashMap;

fn main() {
    //create an empty vector of employee names and
    //and an empty hashmap of allocations
    let mut employees: Vec<String> = Vec::new();
    let mut allocations: HashMap<String, String> = HashMap::new();
    
    loop {
        //sort existing employees vector alphabetically
        //this means when viewed later, they will be viewed in alphabetical order
        employees.sort();

        //empty mut variable for initial user response
        //user prompt re: view database, add to database, exit program
        let mut response = String::new();
        println!("Do you wish to (v)iew or (a)dd to the database?");
        println!("Type x to exit the program");

        //assign user input to response variable
        io::stdin()
            .read_line(&mut response)
            .expect("Enter a value input: (v)iew or (a)dd");

        //choose match arm based on response
        match response.trim() {
            "V" | "v" => view(&allocations, &employees), //see database
            "A" | "a" => add(&mut allocations, &mut employees), //add database TODO
            "X" | "x" => break,
            _ => println!("Enter a value input: (v)iew or (a)dd"),
        }

    }

}

fn view(allocations: &HashMap<String, String>, employees: &Vec<String>) {

    for employee in employees {
        println!("{:?}: {:?}", employee, allocations[employee]);
    }
}

//TODO: add information to the database
fn add(allocations: &mut HashMap<String, String>, employees: &mut Vec<String>) {
    loop {
        //declare empty variables for employee name to be added
        //and dept they work in
        let mut name = String::new();
        let mut dept = String::new();
        
        //prompt user input for name
        println!("What is the name of the employee you wish to add?");

        //assign user input to name variable
        io::stdin()
            .read_line(&mut name)
            .expect("This is not a name.");

        //prompt user input for department
        println!("What is the department you wish to add them too?");

        //assign user input to dept variable
        io::stdin()
            .read_line(&mut dept)
            .expect("Please enter a valid input for the department.");
        
        //assign new variable for name and dept without new line with .trim() function
        //prompt user input for confirmation
        //offer exit option 
        let name_trim = name.trim().to_string();
        let dept_trim = dept.trim().to_string();
        println!("Add {name_trim} to {dept_trim}: are you sure? Y/N");
        back(); 
        
        //mut variable for yes/no confirmation
        //assing variable to y_n
        let mut y_n = String::new();
        io::stdin()
            .read_line(&mut y_n)
            .expect("Please type Y or N");

        //
        match y_n.trim() {
            "Y" | "y" => {
                employees.push(name_trim.clone());
                allocations.insert(name_trim, dept_trim);
            }
            "N" | "n" => print!("Not added. "),
            "B" | "b" => break,
            _ => println!("Please enter valid responses.")

        }

        println!("Do you ")
    }
}

fn back(){
    println!("Enter B to go back");
}
