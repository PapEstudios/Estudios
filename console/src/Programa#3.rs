use std::io;
fn main(){
    let mut t1 = String::new();
    let mut t2 = String::new();
    let mut t3 = String::new();
    let mut nombre = String::new();
    let mut paralelo = String::new();
    let mut curso = String::new();
    println!("Ingrese su nombre");
    io::stdin().read_line(&mut nombre).expect("Fallo al leer el input");
    println!("Ingrese su curso");
    io::stdin().read_line(&mut curso).expect("Fallo al leer el input");
    println!("Ingrese su paralelo");
    io::stdin().read_line(&mut paralelo).expect("Fallo al leer el input");
    println!("Ingrese su primera nota");
    io::stdin().read_line(&mut t1).expect("Fallo al leer el input");
    println!("Ingrese su segunda nota");
    io::stdin().read_line(&mut t2).expect("Fallo al leer el input");
    println!("Ingrese su tercera nota");
    io::stdin().read_line(&mut t3).expect("Fallo al leer el input");
    let t1:f32 = match t1.trim().parse() {
        Ok(n) => n,
        Err(_) => 0.0,
    };
    let t2:f32 = match t2.trim().parse() {
        Ok(n) => n,
        Err(_) => 0.0,
    };
    let t3:f32 = match t3.trim().parse() {
        Ok(n) => n,
        Err(_) => 0.0,
    };
    let curso:i32 =match curso.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    let promedio = (t1+t2+t3)/3.00;
    println!("El promedio es {promedio}");
    if promedio >= 14.0 {
        println!("Usted paso de año con una nota de {promedio} del curso {curso}");
    }else{
        println!("Se quedo de año");
    }
}