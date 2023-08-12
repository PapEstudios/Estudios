use std::io;
fn main() {
    let mut select_operation = String::new();
    io::stdin().read_line(&mut select_operation).expect("Error al leer la operación");
    let select_operation = select_operation.trim();

    if select_operation == "multiplicacion" {
        println!("Ingrese los 2 elementos que desea operar");

        let mut select_number1 = String::new();
        io::stdin().read_line(&mut select_number1).expect("Error al leer el número");
        let x: isize = match select_number1.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let mut select_number2 = String::new();
        io::stdin().read_line(&mut select_number2).expect("Error al leer el número");
        let y: isize = match select_number2.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let result1 = x * y;
        println!("El resultado de la multiplicación es: {}", result1);
    } else if select_operation == "suma" {
                println!("Ingrese los 2 elementos que desea operar");

        let mut select_number1 = String::new();
        io::stdin().read_line(&mut select_number1).expect("Error al leer el número");
        let x: isize = match select_number1.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let mut select_number2 = String::new();
        io::stdin().read_line(&mut select_number2).expect("Error al leer el número");
        let y: isize = match select_number2.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let result2 = x + y;
        println!("El resultado de la suma es: {}", result2);
    } else if select_operation == "resta" {
                println!("Ingrese los 2 elementos que desea operar");

        let mut select_number1 = String::new();
        io::stdin().read_line(&mut select_number1).expect("Error al leer el número");
        let x: isize = match select_number1.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let mut select_number2 = String::new();
        io::stdin().read_line(&mut select_number2).expect("Error al leer el número");
        let y: isize = match select_number2.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let result3 = x - y;
        println!("El resultado de la resta es: {}", result3);
    } else if select_operation == "division" {
                println!("Ingrese los 2 elementos que desea operar");

        let mut select_number1 = String::new();
        io::stdin().read_line(&mut select_number1).expect("Error al leer el número");
        let x: isize = match select_number1.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let mut select_number2 = String::new();
        io::stdin().read_line(&mut select_number2).expect("Error al leer el número");
        let y: isize = match select_number2.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };

        let result4 = x / y;
        println!("El resultado de la division es: {}", result4);
    } else {
        println!("Usted no agregó una opción válida. Pruebe con: multiplicacion, division, resta o suma.");
    }
}
