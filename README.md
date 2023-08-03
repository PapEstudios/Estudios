# RustProgramacion
## Variables y constantes
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
# Tipos de datos
## Tipos escalares
### Enteros
![Imagen_de_enteros](public/imgs/Enteros.png)
