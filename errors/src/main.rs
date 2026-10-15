use std::fs::File;
//fn main() {
//    let greeting_file = File::open("hello.txt")?;    
//}

fn main (

    cats: String = "There are cats. They are black and white.";

    last_char_of_first_lines(cats);
)

fn last_char_of_first_lines(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}