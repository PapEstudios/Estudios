fn main() {
    //Condicion normal
    let pepe = 23;
    if pepe != 23 {
        println!("pepe");
    } else if pepe == 20 {
        println!("sexo");
    } else {
        println!("Andres");
    }
    
    //Condicion en linea
    let condition = true;
    let number = if condition { "5" } else { "six" };
    println!("The value of number is: {}", number);
    
    //Bucle loop
    loop {
        println!("again!");
        break; // Agregamos un break para que el bucle no sea infinito.
    }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result); // Cambiamos {result} por {}", ya que se usa el formato de macro de println.
    
    // Bucle 'counting_up
    let mut count = 0; // Agregamos la declaración de la variable count.
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    
    // Bucle while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    
    // Bucle for
    for element in &a {
        println!("the value is: {}", element);
    }
    
    for number in (1..=4).rev() {
        println!("{}!", number);
    }
    
    for number in 1..=5 {
        println!("Number: {}", number);
    }
    
    println!("LIFTOFF!!!");
}
