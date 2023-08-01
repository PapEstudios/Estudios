fn main() {
    println!("Hello, world!");
    let ed : i32 = 65;
    let tp : f64 = 30.00;
    let ne : f64 = 3.00;
    let tp1 = tp * ne;
    if ed >= 65 {
        let tpt = tp / 2.0;
        println!("El total a pagar es {tpt}");
    } else {
        println!("El total a pagar es {tp1}");
    }
}
