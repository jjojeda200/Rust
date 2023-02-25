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

use std::collections::BTreeMap;

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

#[allow(dead_code)]
pub fn fn_interrogante() {

    let claves = vec!["Llave 1", "Llave 2", "Llave 3", "Llave 4", "Llave 5", "Llave 6"];

    // Inserta la primera clave "Llave 1"
    let mut mi_map = BTreeMap::new();
    mi_map.insert("Llave 1", 0);

    for var_llave in claves {
        mi_map.entry(var_llave).or_insert(0);
    }
    let retorno = get_value(&mi_map, "Llave 2");
    println!("{:?}", retorno);

    match get_value(&mi_map, "Llave 8") {
        Ok(valor) => println!("Valor: {}", valor),
        Err(mensaje) => println!("Error: {}", mensaje),
    }
}

fn get_value(map: &BTreeMap<&str, i32>, key: &str) -> Result<i32, String> {
    let value = map.get(key).ok_or("Valor no encontrado")?;
    Ok(*value)
}

/* fn insert_value<'a>(map: &mut BTreeMap<&str'a, i32>, key: &'a str, value: i32) -> Result<(), String> {
    map.insert(key, value).ok_or("Error insertando")?;
    Ok(())
} */

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
#[allow(dead_code)]
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

    let identidad = identificador0(1);
        println!("Identidad 1: {}", identidad);
    let identidad = identificador0(2);
        println!("Identidad 2: {}", identidad);
}

fn identificador0(id:u8) -> String{
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

//***************************************************************************** expect
/* Nota:        
El método expect se utiliza de manera similar a unwrap, permite proporcionar un
mensaje de error personalizado en caso de que el valor sea None o Err.
El mensaje de error personalizado se pasa como argumento a expect.

Es recomendable utilizar expect en lugar de unwrap en situaciones en las que
se espera que el valor subyacente sea None o Err.
*/
#[allow(dead_code)]
pub fn fn_expect_0() {
    let titulo = String::from(" expect ");
    imprime_titulo(&titulo);

    let resultado = dividir(10, 0);
    let valor = resultado.expect("Error al dividir");
    println!("El resultado es: {}", valor);
}

fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { return Err(String::from("No se puede dividir por cero")); }
    
    Ok(a / b)
}

//*****************************************************************************
/* Nota:        
La macro "assert" toma una expresión como argumento y comprueba si la expresión
es verdadera o falsa. Si la expresión es verdadera, el programa continúa su ejecución
normalmente. Si la expresión es falsa, el programa finaliza inmediatamente y se muestra
un mensaje de error que indica la ubicación del error y la expresión que falló.
*/
#[allow(dead_code)]
pub fn fn_assent() {
    let titulo = String::from(" assent / assent_eq / assent_ne ");
    imprime_titulo(&titulo);

    let var_cadena = "Cadena 0";

    assert!( var_cadena == "Cadena 0", "{} debiera ser Cadena 0", var_cadena );
    assert_eq!( var_cadena, "Cadena 0", "{} var_cadena y deberían ser iguales", var_cadena );
    assert_ne!( var_cadena, "Cadena 1", "Introduciste {}. La entrada no debe ser igual a cadena 1", var_cadena );

    println!("Si ejecuto este println, se cumplieron todos los assent");
}

//*****************************************************************************