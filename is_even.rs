use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    print!("Input a number: ");
    let _i = io::stdout().flush();
    io::stdin()
        .read_line(&mut n)
        .expect("error");
    let n_new = &n;
    println!("Is {} even? {}", n_new.trim(), 
                match is_even(n_new.to_string()) {
                    true => "Yes",
                    false => "No",
                }
             );
}
fn is_even(number: String) -> bool {
    return (number.trim().parse::<isize>().unwrap()) % 2 == 0;
}
