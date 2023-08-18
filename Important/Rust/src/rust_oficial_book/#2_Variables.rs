const TILINGOD: &str = "El dios tilin";
fn main(){
    println!("{TILINGOD}");
    //Declaracion de variables
    let x = 3;
    println!("{x}");
    //Shadowing
    let x = 4;
    println!("{x}");
    //Variables mutables
    let mut pepe = "pepe";
    println!("{pepe}");
    pepe = "tilin";
    println!("{pepe}");    
}