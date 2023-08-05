# RustProgramacion
    Fuentes:
[LearnXinY](https://learnxinyminutes.com/docs/es-es/rust-es/)
[Youtube1](https://www.youtube.com/playlist?list=PLAMfQH2NKM_tyKzBV1iJf5L8j7oJl6KHl)
[Youtube2](https://www.youtube.com/playlist?list=PL1GmZplwRjmNIdo_G2-Jl9mGEsDNHcOPe)
[Youtube3](https://www.youtube.com/playlist?list=PLQFivIgaGdhzNWYGlT3Rybq9EG5ZvYCR-)
[Youtube4](https://www.youtube.com/watch?v=birgJIuIerk&list=PLQFivIgaGdhzNWYGlT3Rybq9EG5ZvYCR-)
# Variables y constantes
    Las variables se declaran con la keyword "let" y para hacer una palabra mutable se utiliza "let mut" y para hacer constantes se usa la keyword "const" y estas variables pueden ser globales o locales ahora veremos como se usarian en codigo real:
---

```rust
const PI = 3.1415;
fn main() {
    //Variables con let:
    let x = 5;
    //Ahora intentaremos cambiar la variable aunque no se use "mut" y veamos que pasa
    x = "cambiamos"; //Esto generara un error en la linea de comandos
    //Aunque realmente si se puede cambiar aplicando un concepto llamado shadowing que lo que hace es reasignar la variable y por asi decirlo destruir el anterior
    let x = "cambiamos"; // Esto no genera error
    //Variables con let mut
    let mut y = 5;
    //Ahora cambiaremos la variable 
    y = "cambiamos"; //Esto no genera error y se cambia de manera normal
    //Variables constantes
    const SALUDO = "hola";//ni lo intentes cambiar porque es una constante dara error si o si
}
```
## Impresion por consola
    Para imprimir con consola un elemento debe ir en el print esto "{}" asi como que se codifica para que sepa q hacer
```rust
fn main(){
    let c = "3";
    print!("{}", c);
    //Tambien puede ponerse dentro
    let x = 3;
    println!("{c}");
}
```
## Nomenclatura recomendada para escribir las variables
    Variables y constantes: snake_case
        Ejemplo: mi_variable, otra_variable, constante_importante

    Funciones y métodos: snake_case
        Ejemplo: calcular_suma, guardar_archivo, procesar_datos

    Tipos y estructuras: PascalCase
        Ejemplo: MiEstructura, TipoPersonalizado, OtroTipo

    Módulos: snake_case
        Ejemplo: mi_modulo, otro_modulo, modulo_utilidades

    Crates (paquetes): snake_case
        Ejemplo: mi_crate, otro_crate, crate_utilidades

    Enumeraciones (Enums): PascalCase
        Ejemplo: EstadoCivil, OpcionesMenu, TipoError

    Variantes de Enum: snake_case
        Ejemplo: opcion_default, variante_especial, valor_otro_caso

    Tipos genéricos: CamelCase
        Ejemplo: TipoGenerico, MiTipoGenerador, OtroTipoParametrizado

    Constantes: SCREAMING_SNAKE_CASE (mayúsculas y separadas por guiones bajos)
        Ejemplo: PI, VALOR_MAXIMO, URL_BASE

# Tipos de datos
    La mayor parte del tiempo, el compilador de Rust puede inferir el tipo de
    una variable, por lo que no necesitas escribir una anotación de tipo
    explícita. A lo largo de este tutorial, los tipos están anotados
    explícitamente en varios sitios, pero solo con propósito demostrativo. La
    inferencia de tipos puede manejar esto por ti la mayor parte del tiempo.
### Enteros y Flotantes
![Imagen_de_enteros](public/imgs/Enteros.png)
    
    El total que alcanza:
    
    i8 (entero con signo de 8 bits):
        Rango: -128 a 127

    u8 (entero sin signo de 8 bits):
        Rango: 0 a 255

    i16 (entero con signo de 16 bits):
        Rango: -32,768 a 32,767

    u16 (entero sin signo de 16 bits):
        Rango: 0 a 65,535

    i32 (entero con signo de 32 bits):
        Rango: -2,147,483,648 a 2,147,483,647

    u32 (entero sin signo de 32 bits):
        Rango: 0 a 4,294,967,295

    i64 (entero con signo de 64 bits):
        Rango: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807

    u64 (entero sin signo de 64 bits):
        Rango: 0 a 18,446,744,073,709,551,615

    i128 (entero con signo de 128 bits):
        Rango: -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727

    u128 (entero sin signo de 128 bits):
        Rango: 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455

    Para floats seria practicamente igual y aqui un pequeño ejemplo en codigo:
    ~~extra no se puede hacer una misma operacion usando los 2 tipos de datos numericos~~
```rust
fn main() {
    let x : f8 = 3.00;
    let y : i8 = 300;
}
```
### Boleanos
    aqui se usa la keyword "bool" que solo tiene 2 valores: false y true el tamaño es de 1 bit.
    Ejemplo:i
```rust
fn main() {
    let x : bool = false;
    let y : bool = true;
}
```
### De 1 solo caracter
    aqui se usa la keyword "char" y debe ir en comillas simples se puede poner tanto emojis como palabras de otros idiomas o cualquier otra cosa que requiera de 1 solo caracter.
    Ejemplo:
```rust
fn main() {
    let x : char = 'c';
    let y : char = '😊';
}
```
## Tipos compuestos
### Tuplas
    Se usa para agrupar varios valores de diferentes tipos con una longitud fija. Una vez creada no puede crecer ni encogerse de tamaño. 
    Se escriben separados por comas y entres parentesis. Cada posicion en la tupa tiene diferentes tipos y los tipos son diferentes valores ademas no tienen q ser iguales.
```rust
fn main() {
    let tupla: (i32, f32, &str) = (600, 503.00, "hola pepe")

    // Una tupla es un conjunto de tamaño fijo de valores. Pueden ser de diferente tipo.
    let x: (i32, &str, f64) = (1, "hola", 3.4);
    let (x,y,z) = tupla;
    
    // Desestructurando `let`
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 hola 3.4

    // Indexando
    println!("{}", x.1); // hola
    let primero = tupla.0;
}
```
### Arrays 

## Tipo string
    Los strings se declaran con la keyword "&str" pero tambien con la keyword "String" la difrencia es q "&str" es un string inmutable y string es un string mutable tipo se usa "String" cuando no sabes que tamaño tendra la cadena de texto. Puedes declarar la variable con "::from" y luego agregarle mas valores con ".push" y cuando sabes exactamente cuando terminaras la cadena usas ".push_str".
    un ejemplo:
```rust
fn main() {
    let x : &str = "holamundo";
    let y : &str = "el pepe";
    let s_slice: &str = &s;
    /*
    En esta línea, se declara una "porción de cadena" o "slice". Un slice es una vista inmutable de otra cadena o matriz, lo que significa que no contiene los datos en sí, sino que es un puntero inmutable a un fragmento de la cadena original.
    La sintaxis &s se utiliza para crear una referencia inmutable (&) a la cadena original s, que luego se asigna a s_slice.
    */
    println!("{} {}", s, s_slice); // hola mundo hola mundo
    let mut ns : String::from = "hola don pepito";
    ns.push = 'hola';
    ns.push_str = " don jose";
}
``` 
# Operadores
## Operadores aritméticos:
    + : Suma
    - : Resta
    * : Multiplicación
    / : División
    % : Módulo (resto de la división)

## Operadores de asignación:
    = : Asignación simple
    += : Suma y asignación
    -= : Resta y asignación
    *= : Multiplicación y asignación
    /= : División y asignación
    %= : Módulo y asignación

## Operadores de comparación:
    == : Igual a
    != : Diferente de
    < : Menor que
    > : Mayor que
    <= : Menor o igual que
    >= : Mayor o igual que

## Operadores lógicos:
    && : Y lógico (AND)
    || : O lógico (OR)
    ! : Negación lógica (NOT)

## Operadores de bits:
    & : AND a nivel de bits
    | : OR a nivel de bits
    ^ : XOR a nivel de bits
    << : Desplazamiento a la izquierda (shift left)
    >> : Desplazamiento a la derecha (shift right)

## Operadores de incremento y decremento:
    ++ : Incremento (no es compatible en Rust, solo se puede usar como x += 1)
    -- : Decremento (no es compatible en Rust, solo se puede usar como x -= 1)

## Otros operadores:
    . : Acceso a miembros (para acceder a miembros de una estructura o módulo)
    : : Separador de tipo (por ejemplo, en la declaración de variables con tipo)
    ::: Resolución de alcance (para acceder a constantes, funciones o tipos asociados a un módulo)
# Funciones
    Estos se escriben fuera del cuerpo de la funcion principal o main, ademas, lo que va dentro de la funcion que escribiremos debe ir sin ";" aunque segun parece si puere tener ;.
    Ejemplo:
```rust
fn pepitosaluda(){
    println!("Hola me llamo pepito!");
}
fn main(){
    pepitosaluda()
}
```
# Estructuras de control
## Estructura de control if
    El if de toda la vida.
    En Rust, el if es una estructura de control condicional que permite que un bloque de código se ejecute solo si se cumple una condición específica. La sintaxis básica del if en Rust es la siguiente:
    ejemplo:
```rust
    if 1 == 1 {
        println!("Las matemáticas funcionan!");
    } else {
        println!("Oh no...");
    }

    // `if` como una expresión
    let valor = if true {
        "bueno"
    } else {
        "malo"
    };
```




```rust
fn main(){  
    //////////////
    // 2. Tipos //
    //////////////

    // Estructuras
    struct Punto {
        x: i32,
        y: i32,
    }

    let origen: Punto = Punto { x: 0, y: 0 };

    // Una estructura con campos sin nombre, una ‘estructura de tupla’
    struct Punto2(i32, i32);

    let origen2 = Punto2(0, 0);

    // Enums básicos como en C
    enum Direccion {
        Izquierda,
        Derecha,
        Arriba,
        Abajo,
    }

    let arriba = Direccion::Arriba;

    // Enum con campos
    enum OpcionalI32 {
        UnI32(i32),
        Nada,
    }

    let dos: OpcionalI32 = OpcionalI32::UnI32(2);
    let nada = OpcionalI32::Nada;

    // Genéricos //

    struct Foo<T> { bar: T }

    // Esto está definido en la librería estándar como `Option`
    enum Opcional<T> {
        AlgunVal(T),
        SinVal,
    }

    // Métodos //

    impl<T> Foo<T> {
        // Los métodos reciben un parámetro explícito `self`
        fn get_bar(self) -> T {
            self.bar
        }
    }

    let un_foo = Foo { bar: 1 };
    println!("{}", un_foo.get_bar()); // 1

    // Traits (conocidos como interfaces o typeclasses en otros lenguajes) //

    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }

    let otro_foo = Foo { bar: 1 };
    println!("{:?}", otro_foo.frobnicate()); // Some(1)

    /////////////////////////////////
    // 3. Comparación con patrones //
    /////////////////////////////////

    let foo = OpcionalI32::UnI32(1);
    match foo {
        OpcionalI32::UnI32(n) => println!("es un i32: {}", n),
        OpcionalI32::Nada  => println!("no es nada!"),
    }

    // comparación de patrones avanzada
    struct FooBar { x: i32, y: OpcionalI32 }
    let bar = FooBar { x: 15, y: OpcionalI32::UnI32(32) };

    match bar {
        FooBar { x: 0, y: OpcionalI32::UnI32(0) } =>
            println!("Los números son cero!"),
        FooBar { x: n, y: OpcionalI32::UnI32(m) } if n == m =>
            println!("Los números son iguales"),
        FooBar { x: n, y: OpcionalI32::UnI32(m) } =>
            println!("Números diferentes: {} {}", n, m),
        FooBar { x: _, y: OpcionalI32::Nada } =>
            println!("El segudo número no es nada!"),
    }

    /////////////////////////
    // 4. Flujo de control //
    /////////////////////////

    // bucles `for`
    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    // Rangos
    for i in 0u32..10 {
        print!("{} ", i);
    }
    println!("");
    // imprime `0 1 2 3 4 5 6 7 8 9 `
    // bucle `while`
    while 1 == 1 {
        println!("El universo está funcionando correctamente.");
    }

    // Bucle infinito
    loop {
        println!("Hola!");
    }

    ////////////////////////////////////////
    // 5. Seguridad de memoria y punteros //
    ////////////////////////////////////////

    // Posesión de punteros – solo uno puede ‘poseer’ un puntero en cada momento
    // Esto significa que cuando la `Box` queda fuera del ámbito, puede ser
    // liberada automáticamente de manera segura.
    let mut mio: Box<i32> = Box::new(3);
    *mio = 5; // dereferenciar
    // Aquí, `ahora_es_mio`, toma posesión de `mio`. En otras palabras, `mio` se
    // mueve.
    let mut ahora_es_mio = mio;
    *ahora_es_mio += 2;

    println!("{}", ahora_es_mio); // 7
    // println!("{}", mio); // esto no compilaría, porque `now_its_mine` es el
    // que posee el puntero

    // Referencia – un puntero inmutable que referencia a otro dato
    // Cuando se crea una referencia a un valor, decimos que el valor ha sido
    // ‘tomado prestado’.
    // Mientras un valor está prestado como inmutable, no puede ser modificado o
    // movido.
    // Una prestación dura hasta el fin del ámbito en el que se creó.
    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var); // A diferencia de `mio`, `var` se puede seguir usando
    println!("{}", *ref_var);
    // var = 5; // esto no compilaría, porque `var` está prestada
    // *ref_var = 6; // esto tampoco, porque `ref_var` es una referencia
    // inmutable

    // Referencia mutable
    // Mientras que un valor está prestado como mutable, no puede ser accedido
    // desde ningún otro sitio.
    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2; // '*' se usa para apuntar al var2 prestado como mutable

    println!("{}", *ref_var2); // 6 , //var2 no compilaría. //ref_var2 es de
                               // tipo &mut i32, por lo que guarda una
                               // referencia a un i32 no el valor.
    // var2 = 2; // esto no compilaría porque `var2` está prestado
}
```