fn main() {
    // Números
    // Tabla de tamaños
    /*
        Length  Signed  Unsigned
        8-bit   i8      u8
        16-bit  i16     u16
        32-bit  i32     u32
        64-bit  i64     u64
        128-bit i128    u128
        arch    isize   usize
    */

    // Tipos numéricos
    let numero_flotante: f32 = 13.0; // Número de punto flotante
    let numero_entero: i32 = 30;     // Número entero

    /*
        Ejemplos de literales numéricos:
        Decimal     98_222
        Hex         0xff
        Octal       0o77
        Binario     0b1111_0000
        Byte (solo u8) b'A'
     */

    // Booleanos
    let falso: bool = false;    // Valor booleano falso
    let verdadero: bool = true; // Valor booleano verdadero

    // Strings
    let cadena: &str = "Soy un string"; // Cadena de texto inmutable
    let mut cadena_mutable = String::new(); // Cadena de texto mutable vacía
    cadena_mutable = String::from("pepito lala"); // Asignar valor a la cadena mutable
    cadena_mutable.push_str(" otra vez"); // Agregar texto a la cadena mutable
    let cadena_convertida = cadena_mutable.to_string(); // Convertir cadena mutable a String
    println!("{}", cadena_convertida);

    // Char
    let caracter: char = 'c'; // Carácter Unicode

    // Computados
    // Tuplas
    let tuple: (i32, f32, &str) = (12, 30.22, "pepito"); // Tupla con diferentes tipos
    println!("{}", tuple.1); // Acceso al segundo elemento de la tupla
    let (x, y, z) = tuple;   // Desestructuración de la tupla
    println!("{} {} {}", x, y, z);
    let string = tuple.2;    // Acceso al tercer elemento

    // Arrays
    let array = [1, 2, 3, 4, 5, 6]; // Array inmutable
    println!("{}", array[0]); // Acceso al primer elemento del array
    let meses = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];
    println!("{}", meses[1]); // Acceso al segundo elemento del array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Array con tipo y tamaño específico
    println!("{}", a[1]); // Acceso al segundo elemento
    let a = [3; 5]; // Array con cinco veces el valor 3
    let a1 = a[1];
    println!("{}", a1); // Acceso al segundo elemento del nuevo array
}
