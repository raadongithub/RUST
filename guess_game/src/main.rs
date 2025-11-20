use std::io;

fn main(){
    println!("Number Guess kro bhai: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Nahi bhai, galat jawab.");
    println!("Apka jawab: {guess}");
}