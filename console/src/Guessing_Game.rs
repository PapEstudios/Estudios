use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero random!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Ingrese un numero.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falla al leer su input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu numero: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Estas lejos!"),
            Ordering::Greater => println!("Estas cerca!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}