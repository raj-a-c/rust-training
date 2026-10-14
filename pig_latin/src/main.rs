fn main() {
    let word = "doggy";
    
    for c in word.chars() {
        if c == 'a' || c == 'e' || c == 'i'|| c == 'o'|| c == 'u' {

            println!("{word}-hay");
    
            break;

        } else {
                let mut i = 0; 

                for c in word.chars() {
                    if i > 0 {
                        print!("{c}");
                    }

                    i += 1;
                    
                }
                
                print!("-");

                for c in word.chars() {
                    print!("{c}");
                    break;
                }
                print!("ay\n");
                break;
        }
    }

}
