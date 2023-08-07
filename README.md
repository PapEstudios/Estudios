# RustProgramacion
    Fuentes:
[LearnXinY](https://learnxinyminutes.com/docs/es-es/rust-es/)
[Youtube1](https://www.youtube.com/playlist?list=PLAMfQH2NKM_tyKzBV1iJf5L8j7oJl6KHl)
[Youtube2](https://www.youtube.com/playlist?list=PL1GmZplwRjmNIdo_G2-Jl9mGEsDNHcOPe)
[Youtube3](https://www.youtube.com/playlist?list=PLQFivIgaGdhzNWYGlT3Rybq9EG5ZvYCR-)
[Youtube4](https://www.youtube.com/watch?v=birgJIuIerk&list=PLQFivIgaGdhzNWYGlT3Rybq9EG5ZvYCR-)
# Biblioteca std o standard de rust
## Tipos Primitivos:
    never: Un tipo experimental llamado "never". Es un tipo que no tiene valores válidos. Se utiliza para indicar que una expresión nunca terminará o nunca producirá un valor.
    array: Un arreglo de tamaño fijo, denotado como [T; N], donde T es el tipo de los elementos y N es el tamaño constante no negativo.
    bool: El tipo booleano, que puede ser verdadero (true) o falso (false).
    char: Un tipo de dato que representa un único carácter Unicode.
    f32: Un tipo de punto flotante de 32 bits, específicamente el tipo "binary32" definido en IEEE 754-2008.
    f64: Un tipo de punto flotante de 64 bits, específicamente el tipo "binary64" definido en IEEE 754-2008.
    fn: Punteros a funciones, como fn(usize) -> bool.
    i8: El tipo de dato entero con signo de 8 bits.
    i16: El tipo de dato entero con signo de 16 bits.
    i32: El tipo de dato entero con signo de 32 bits.
    i64: El tipo de dato entero con signo de 64 bits.
    i128: El tipo de dato entero con signo de 128 bits.
    isize: El tipo de dato entero con signo del tamaño del puntero.
    pointer: Punteros crudos y no seguros, *const T y *mut T.
    reference: Referencias, &T y &mut T.
    slice: Una vista de tamaño dinámico en una secuencia contigua, [T]. "Contigua" significa que los elementos están dispuestos de manera que cada elemento tiene la misma distancia de sus vecinos.
    str: Segmentos de cadena.
    tuple: Una secuencia finita y heterogénea, (T, U, ...).
    u8: El tipo de dato entero sin signo de 8 bits.
    u16: El tipo de dato entero sin signo de 16 bits.
    u32: El tipo de dato entero sin signo de 32 bits.
    u64: El tipo de dato entero sin signo de 64 bits.
    u128: El tipo de dato entero sin signo de 128 bits.
    unit: El tipo () también llamado "unidad". Representa la falta de información.
    usize: El tipo de dato entero sin signo del tamaño del puntero.
## Modulos
    assert_matchesExperimental: Módulo inestable que contiene la macro no estable "assert_matches".
    async_iterExperimental: Iteración asincrónica componible.
    intrinsicsExperimental: Intrínsecos del compilador.
    simdExperimental: Módulo SIMD portátil.
    alloc: APIs de asignación de memoria.
    any: Utilidades para tipificación dinámica o reflexión de tipos.
    arch: Módulo de instrucciones SIMD y intrínsecos de proveedores.
    array: Utilidades para el tipo primitivo de arreglo.
    ascii: Operaciones en cadenas y caracteres ASCII.
    backtrace: Soporte para capturar una traza de pila de un hilo del sistema operativo.
    borrow: Un módulo para trabajar con datos prestados.
    boxed: El tipo Box<T> para asignación en el montón.
    cell: Contenedores mutables compartibles.
    char: Utilidades para el tipo primitivo de carácter.
    clone: El trazo Clone para tipos que no pueden ser "copiados implícitamente".
    cmp: Utilidades para comparar y ordenar valores.
    collections: Tipos de colección.
    convert: Tratamientos para conversiones entre tipos.
    default: El trazo Default para tipos con un valor predeterminado.
    env: Inspección y manipulación del entorno del proceso.
    error: Interfaces para trabajar con errores.
    f32: Constantes para el tipo de punto flotante de precisión simple f32.
    f64: Constantes para el tipo de punto flotante de doble precisión f64.
    ffi: Utilidades relacionadas con enlaces FFI.
    fmt: Utilidades para el formato y la impresión de cadenas.
    fs: Operaciones de manipulación del sistema de archivos.
    future: Funcionalidad básica asincrónica.
    hash: Soporte genérico de hash.
    hint: Indicaciones al compilador que afectan cómo se debe emitir u optimizar el código. Las indicaciones pueden ser en tiempo de compilación o en tiempo de ejecución.
    i8Deprecation planned: Módulo de constantes redundantes para el tipo primitivo i8 (planeado).
    i16Deprecation planned: Módulo de constantes redundantes para el tipo primitivo i16 (planeado).
    i32Deprecation planned: Módulo de constantes redundantes para el tipo primitivo i32 (planeado).
    i64Deprecation planned: Módulo de constantes redundantes para el tipo primitivo i64 (planeado).
    i128Deprecation planned: Módulo de constantes redundantes para el tipo primitivo i128 (planeado).
    io: Tratamientos, ayudantes y definiciones de tipos para la funcionalidad básica de E/S.
    isizeDeprecation planned: Módulo de constantes redundantes para el tipo primitivo isize (planeado).
    iter: Iteración externa componible.
    marker: Tratos primitivos y tipos que representan propiedades básicas de tipos.
    mem: Funciones básicas para tratar con memoria.
    net: Primitivas de red para comunicación TCP/UDP.
    num: Funcionalidad adicional para tipos numéricos.
    ops: Operadores sobrecargables.
    option: Valores opcionales.
    os: Funcionalidad específica del sistema operativo.
    panic: Soporte para pánicos en la biblioteca estándar.
    path: Manipulación de rutas multiplataforma.
    pin: Tipos que anclan datos a su ubicación en memoria.
    prelude: El Preludio de Rust.
    primitive: Este módulo reexporta los tipos primitivos para permitir su uso que no pueda ser oscurecido por otros tipos declarados.
    process: Un módulo para trabajar con procesos.
    ptr: Manejo manual de memoria a través de punteros en bruto.
    rc: Punteros con recuento de referencias de un solo subproceso. 'Rc' significa 'Reference Counted'.
    result: Manejo de errores con el tipo Result.
    slice: Utilidades para el tipo primitivo de rebanada.
    str: Utilidades para el tipo primitivo de cadena.
    string: Una cadena UTF-8 codificada y ampliable.
    sync: Primitivas de sincronización útiles.
    task: Tipos y tratos para trabajar con tareas asincrónicas.
    thread: Hilos nativos.
    time: Cuantificación temporal.
    u8Deprecation planned: Módulo de constantes redundantes para el tipo primitivo u8 (planeado).
    u16Deprecation planned: Módulo de constantes redundantes para el tipo primitivo u16 (planeado).
    u32Deprecation planned: Módulo de constantes redundantes para el tipo primitivo u32 (planeado).
    u64Deprecation planned: Módulo de constantes redundantes para el tipo primitivo u64 (planeado).
    u128Deprecation planned: Módulo de constantes redundantes para el tipo primitivo u128 (planeado).
    usizeDeprecation planned: Módulo de constantes redundantes para el tipo primitivo usize (planeado).
    vec: Un tipo de arreglo creciente y contiguo con contenidos asignados en el montón, escrito como Vec<T>.
## Macros
    concat_bytesExperimental: Concatena literales en una secuencia de bytes.
    concat_identsExperimental: Concatena identificadores en un solo identificador.
    const_format_argsExperimental: Igual que format_args, pero puede ser utilizado en algunos contextos constantes.
    format_args_nlExperimental: Igual que format_args, pero agrega una nueva línea al final.
    log_syntaxExperimental: Imprime tokens pasados en la salida estándar.
    trace_macrosExperimental: Habilita o deshabilita la funcionalidad de rastreo utilizada para depurar otras macros.
    assert: Asegura que una expresión booleana sea verdadera en tiempo de ejecución.
    assert_eq: Asegura que dos expresiones sean iguales entre sí (usando PartialEq).
    assert_ne: Asegura que dos expresiones no sean iguales entre sí (usando PartialEq).
    cfg: Evalúa combinaciones booleanas de indicadores de configuración en tiempo de compilación.
    column: Se expande al número de columna en el que fue invocado.
    compile_error: Causa que la compilación falle con el mensaje de error proporcionado cuando se encuentra.
    concat: Concatena literales en una secuencia de caracteres estática.
    dbg: Imprime y devuelve el valor de una expresión dada para depuración rápida y sucia.
    debug_assert: Asegura que una expresión booleana sea verdadera en tiempo de ejecución.
    debug_assert_eq: Asegura que dos expresiones sean iguales entre sí.
    debug_assert_ne: Asegura que dos expresiones no sean iguales entre sí.
    env: Inspecciona una variable de entorno en tiempo de compilación.
    eprint: Imprime en la salida de error estándar.
    eprintln: Imprime en la salida de error estándar, con una nueva línea.
    file: Se expande al nombre del archivo en el que fue invocado.
    format: Crea una cadena usando la interpolación de expresiones en tiempo de ejecución.
    format_args: Construye parámetros para otras macros de formateo de cadenas.
    include: Analiza un archivo como una expresión o un elemento según el contexto.
    include_bytes: Incluye un archivo como una referencia a una matriz de bytes.
    include_str: Incluye un archivo codificado en UTF-8 como una cadena.
    is_x86_feature_detectedx86 o x86-64: Una macro para probar en tiempo de ejecución si una característica de CPU está disponible en plataformas x86/x86-64.
    line: Se expande al número de línea en el que fue invocado.
    matches: Devuelve si la expresión dada coincide con alguno de los patrones dados.
    module_path: Se expande a una cadena que representa la ruta actual del módulo.
    option_env: Inspecciona opcionalmente una variable de entorno en tiempo de compilación.
    panic: Genera un pánico en el hilo actual.
    print: Imprime en la salida estándar.
    println: Imprime en la salida estándar, con una nueva línea.
    stringify: Convierte sus argumentos en cadenas.
    thread_local: Declara una nueva clave de almacenamiento local en el hilo del tipo std::thread::LocalKey.
    todo: Indica código incompleto.
    tryDeprecated: Desempaqueta un resultado o propaga su error.
    unimplemented: Indica código no implementado generando un pánico con el mensaje "no implementado".
    unreachable: Indica código inalcanzable.
    vec: Crea un Vec que contiene los argumentos.
    write: Escribe datos formateados en un búfer.
    writeln: Escribe datos formateados en un búfer, con una nueva línea agregada.
## Palabras clave
    SelfTy: El tipo que implementa dentro de un bloque de trazo (trait o impl), o el tipo actual dentro de una definición de tipo.
    as: Realiza una conversión entre tipos o cambia el nombre de una importación.
    async: Devuelve un Futuro en lugar de bloquear el hilo actual.
    await: Suspende la ejecución hasta que el resultado de un Futuro esté listo.
    dyn: Prefijo de un tipo de objeto trazo.
    else: Qué expresión evaluar cuando una condición if es falsa.
    enum: Un tipo que puede ser uno de varios valores.
    extern: Enlaza o importa código externo.
    for: Iteración con in, implementación de trazo con impl, o límites de trazo de rango superior (para <'a>).
    if: Evalúa un bloque si una condición es verdadera.
    impl: Implementa alguna funcionalidad para un tipo.
    in: Itera sobre una serie de valores con for.
    let: Asigna un valor a una variable.
    loop: Bucle indefinido.
    match: Control de flujo basado en coincidencia de patrones.
    mod: Organiza el código en módulos.
    move: Captura el entorno de un cierre por valor.
    mut: Una variable, referencia o puntero mutable.
    pub: Hace que un elemento sea visible para otros.
    ref: Se enlaza por referencia durante la coincidencia de patrones.
    return: Devuelve un valor desde una función.
    self: El receptor de un método o el módulo actual.
    static: Un elemento estático es un valor válido durante toda la duración del programa (una duración 'estática').
    struct: Un tipo que se compone de otros tipos.
    super: El padre del módulo actual.
    trait: Una interfaz común para un grupo de tipos.
    type: Define un alias para un tipo existente.
    union: El equivalente en Rust de unión al estilo C.
    unsafe: Código o interfaces cuya seguridad de memoria no puede ser verificada por el sistema de tipos.
    use: Importa o renombra elementos de otras cajas o módulos.
    where: Agrega restricciones que deben cumplirse para usar un elemento.
    while: Bucle mientras se cumple una condición.
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