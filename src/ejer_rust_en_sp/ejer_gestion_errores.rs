/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          12-02-2023
    Titulo:         introducción a RUST
    Descripción:    Funciones de prueba de control de errores, result, option, match de Rust
    Referencias:
    Rust Programming Language
                https://doc.rust-lang.org/stable/book/
    Rust Reference
                https://doc.rust-lang.org/reference/introduction.html
    Rust by examples
                https://doc.rust-lang.org/beta/rust-by-example/index.html
    Recetas de Rust Cookbook
                https://rust-lang-nursery.github.io/rust-cookbook/
    El Lenguaje de Programación Rust
                https://github.com/goyox86/elpr-sources
    Rust en español fácil
                https://www.jmgaguilera.com/rust_facil/actualizaciones.html
    Tour de Rust
                https://tourofrust.com/TOC_es.html
    Crate std   https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/index.html
                https://doc.rust-lang.org/std/index.html
    Crate gtk   https://gtk-rs.org/gtk3-rs/git/docs/gtk/index.html

***************************************************************************************/

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** panic!
#[allow(dead_code)]
pub fn funcion_panic(){
    let mensaje = "Error: Llamada intencionada a la macro panic!";
    panic!("{}",mensaje);
}

//***************************************************************************** ?
/* Nota:        
Una forma más corta que "match" y que "if let" de gestionar un valor de tipo
Result (y Option) es el operador ?
El operador ? es un atajo para match que devuelve el error si Result es Err,
o devuelve el valor envuelto en Ok si Result es correcto.
*/
fn identificador0(id:u8) -> String{
    if id == 1 {
        let cadena = Option::Some("Id UNO".to_string());
        println!("Cadena envolviendo un: {:?}", cadena);
        let val_ok = cadena.unwrap();
        return val_ok;
    } else  {
        return "Identidad No existe".to_string();
    }
    
}
#[allow(dead_code)]
pub fn fn_interrogante() {
    let titulo = String::from(" Operador ? ");
    imprime_titulo(&titulo);
    
}


//***************************************************************************** unwrap
/* Nota:        
unwrap como expect son métodos que se utilizan para manejar los valores Option
y Result, que son tipos de datos que pueden o no tener un valor.

El método unwrap se utiliza para desempaquetar un valor de tipo Option o Result
y devolver el valor subyacente si existe, o producir un error en caso contrario.
unwrap con un valor None o un Err, producirá un error en tiempo de ejecución.

Teniendo una variable numero de tipo Option<i32> que sabemos que tiene un
valor, podemos utilizar unwrap para obtener el valor subyacente:
*/
pub fn fn_unwrap_0() {
    let titulo = String::from(" unwrap 0 ");
    imprime_titulo(&titulo);
    
    let numero: Option<i32> = Some(5);
    let valor = numero.unwrap();
    println!("El valor es: {}", valor);
}

#[allow(dead_code)]
pub fn fn_unwrap_1() {
    let titulo = String::from(" unwrap 1 ");
    imprime_titulo(&titulo);

    let identidad = identificador1(1);
        println!("Identidad: {}", identidad);
    let identidad = identificador1(2);
        println!("Identidad: {}", identidad);
}

fn identificador1(id:u8) -> String{
    if id == 1 {
        let cadena = Option::Some("Id UNO".to_string());
        println!("Cadena envuelta por: {:?}, sin desenvolver", cadena);
        let val_ok = cadena.unwrap();
        println!("Cadena desenvuelta de el Some: {}", val_ok);
        return val_ok;
    } else  {
        return "Identidad No existe".to_string();
    }
    
}


//***************************************************************************** unwrap
/* Nota:        

En este caso, como sabemos que numero tiene un valor, podemos utilizar unwrap
para obtener ese valor sin necesidad de manejar un posible error.

Por otro lado, el método expect se utiliza de manera similar a unwrap, pero nos permite proporcionar un mensaje de error personalizado en caso de que el valor sea None o Err. El mensaje de error personalizado se pasa como argumento a expect.

Por ejemplo, si tenemos una variable resultado de tipo Result<i32, &str> que podría ser un valor Ok o Err, podemos utilizar expect para manejar el caso en el que sea Err y proporcionar un mensaje de error personalizado:


let resultado: Result<i32, &str> = Err("No se pudo obtener el valor");
let valor = resultado.expect("Error al obtener el valor");

En este caso, si resultado es Err, el programa terminará y se imprimirá el mensaje de error personalizado que hemos proporcionado en expect. Si resultado es Ok, se devolverá el valor subyacente como en el caso de unwrap.

En general, es recomendable utilizar expect en lugar de unwrap en situaciones en las que se espera que el valor subyacente sea None o Err, ya que nos permite proporcionar información útil sobre el error que se ha producido.
*/
//*****************************************************************************