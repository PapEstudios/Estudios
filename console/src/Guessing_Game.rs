use std::io; //Biblioteca de entrada y salida
use std::cmp::Ordering;
use rand::Rng; //Biblioteca random
fn main() {
    //Pedimos que ingrese un numero cualquiera
    println!("Ingrese un numero cualquiera!");

    let mut guess_number = String::new(); //Esto creara una string vacia

    io::stdin()//Aca le decimos que queremos usar la libreria Stdin

    .read_line(&mut guess_number)//Aca le decimos que queremos un input con el "buf" guess_number que es un mutable

    .expect("El pepe ya no es etesetch :(");//Aca decimos q en caso de q el Result sea "Err" muestre el msg y si es "Ok" muestre la variable readline
    
    println!("El numero que usted ingreso es: {guess_number}"); //Mostramos el numero random del usuario

    let secret_number = rand::thread_rng().gen_range(1..=100); //Creamos el numero random y le damos el rnago de 1 a 100

    println!("Y el numero correcto es: {}", secret_number); //mostramos el numeor random

    match guess_number.cmp(secret_number) {
        Ordering::Less => println!("Muy lejos!"),
        Ordering::Greater => println!("Muy cerca!"),
        Ordering::Equal => println!("Ganaste!")
    }
}