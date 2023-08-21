fn main() {
    prueba(17.30);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y); // Corregido {y} a {}

    let f = fun_con_retorno();
    println!("Returned value: {}", f); // Proporcionado un mensaje más claro

    // Usando funciones con tipo de retorno implícito
    let square = square(5);
    println!("Square of 5 is: {}", square);
}

fn prueba(x: f32) {
    println!("La hora es: {}", x); // Cambiado {x} a {}
}

fn fun_con_retorno() -> i32 {
    3
}

// Función para calcular el cuadrado de un número
fn square(x: i32) -> i32 {
    x * x
}
