use std::io;
fn main(){
    println!("Ingrese la edad del cliente:");
    let mut ed = String::new();
    io::stdin().read_line(&mut ed).expect("Fallo al leer el input");
    let ed : i32 = ed.trim().parse().expect("Fallo al convertir su input a numero");

    println!("Ingrese el valor de la entrada de la pelicula que el cliente desea ver:");
    let mut tp = String::new();
    io::stdin().read_line(&mut tp).expect("Fallo al leer el input");
    let tp : f32 = tp.trim().parse().expect("Fallo al convertir su input a numero");

    println!("Ingrese el numero de entradas que desea el cliente:");
    let mut ne  = String::new();
    io::stdin().read_line(&mut ne).expect("Fallo al leer el input");
    let ne: f32 = ne.trim().parse().expect("Fallo al convertir su input a numero");

    let tp1 = tp * ne;

    if ed >= 65 {
        let tpt = tp / 2.0;
        println!("El total a pagar es {tpt}");
    } else {
        println!("El total a pagar es {tp1}");
    }
}