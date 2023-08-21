
fn main(){
    //NUmericos
    //Tabla de tamaños
    /*
        Length	Signed	Unsigned
        8-bit	i8	u8
        16-bit	i16	u16
        32-bit	i32	u32
        64-bit	i64	u64
        128-bit	i128	u128
        arch	isize	usize
    */
    let numero_flotante : f32 = 13.0;
    let numero_entero : i32 = 30;
    /*
        Number literales Example
        Decimal	98_222
        Hex	0xff
        Octal	0o77
        Binary	0b1111_0000
        Byte (u8 only)	b'A'
     */

    //Booleanos
    let falso : bool = false;
    let verdadero : bool = true;

    //String
    let cadena : &str = "Soy un string";
    let mut cadena_mutable = String::new();
    cadena_mutable = "pepito lala";
    cadena_mutable.to_string();
    
    //char
    let caracter: char = 'c';

    //Computados
    //Tuplas
    let tuple : (i32, f32, &str) = (12, 30.22, "pepito");
    println!("{}",{tuple.1});
    let (x,y,z) = tuple;
    println!("{}{}{}",x, y, z);
    let string = tuple.2;
    
    //Arrays
    let array = [1,2,3,4,5,6];
    println!("{}", array[0]);
    let meses = ["January", "February", "March", "April", "May", "June", "July","August", "September","October", "November", "December"];
    println!("{}", meses[1]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}",a[1]);
    let a = [3; 5]; // ESto es 5 veces 3
    let a1= a[1];
    println!("{}", a1);
}