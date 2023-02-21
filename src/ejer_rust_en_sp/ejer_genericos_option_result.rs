/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-02-2023
    Titulo:         introducción a RUST
    Descripción:    Tipos Genéricos
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

    Dependencias:

    Compilar:

    Licencia:

***************************************************************************************/

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** Genéricos T - Intro 0
/* Notas:       
Los parámetros de tipo genérico se definen con los símbolos de menor y mayo que
encierran el nombre que representa al parámetro de tipo.

- Si un tipo es Debug, puede usar {:?}
- Si un tipo es Display, puede usar {}
- Si un tipo implementa trait PartialOrd, permite que los elementos puedan usar
  <, > y ==, entre otros.
*/
use std::cmp::PartialOrd;   // Ahora es posible usar solo PartialOrd.

#[allow(dead_code)]
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
pub fn genericos_0(){
    let titulo = String::from(" Genéricos T - Intro 0 ");
    imprime_titulo(&titulo);

    let a = 10;
    let b = 20;
    let max_int = max(a, b);        // max_int tendrá el valor 20
    println!("Mayor es: {}", max_int);

    let x = "hello";
    let y = "world";
    let max_str = max(x, y);  // max_str tendrá el valor "world"
    println!("Mayor es: {}", max_str);
}

//***************************************************************************** Genéricos T - Intro 1
/* Notas:       
Los parámetros de tipo genérico se definen con los símbolos de menor y mayo que
encierran el nombre que representa al parámetro de tipo.

- Si un tipo es Debug, puede usar {:?}
- Si un tipo es Display, puede usar {}
- Si un tipo implementa trait PartialOrd, permite que los elementos puedan usar
  <, > y ==, entre otros.
*/
use std::fmt::Display;      // Ahora es posible usar solo Display.
use std::fmt::Debug;        // Ahora es posible usar solo Debug.

#[allow(dead_code)]
#[derive(Debug)]
struct Animal {
    nombre: String,
    edad: u8,
}

fn imprime_elemento<T: Debug>(item: T) { 
    println!("Aquí está tu elemento: {:?}", item);
}

/* Nota:        
- Si se tiene un parámetro de tipo T y otro parámetro de tipo T,
  ambos tienen que ser del mismo tipo.
- Si se tiene un parámetro de tipo T y otro parámetro de tipo U,
  pueden ser de diferente tipo, pero también pueden ser del mismo tipo.
*/
fn compara_e_imprime_0<T: Display, U: Display + PartialOrd>(texto: T, num_1: U, num_2: U) {
    println!("{} ¿Es {} mayor que {}? {}", texto, num_1, num_2, num_1 > num_2);
}

fn compara_e_imprime_1<T, U>(texto: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{} ¿Es {} menor que {}? {}", texto, num_1, num_2, num_1 < num_2);
}

#[allow(dead_code)]
pub fn genericos_1() {
    let titulo = String::from(" Genéricos T - Intro 1 ");
    imprime_titulo(&titulo);

    let charlie = Animal {
        nombre: "Charlie".to_string(),
        edad: 1,
    };

    let numero = 55;
    imprime_elemento(charlie);
    imprime_elemento(numero);

//************************************* compara_e_imprime_0
    compara_e_imprime_0("¡¡compara_e_imprime_0!!", 9, 8);

//************************************* compara_e_imprime_1
    compara_e_imprime_1("¡¡compara_e_imprime_1!!", 9, 8);
}

//***************************************************************************** Option - Intro
/* Notas:       
Option trata sobre Some o None. La existencia o no de un valor. Cuando el valor
existe, se usa Some(valor). Cuando no existe es None.
            enum Option<T> {
                None,
                Some(T),
            }
*/
fn identificador(id: u8) -> Option<String> {
    if id == 1 {
        return Option::Some("ID UNO".to_string());
    } else if id == 2 {
        return Option::Some("ID DOS".to_string());
    }
    return None;
}

//************************************* option_intro_0
#[allow(dead_code)]
pub fn option_intro_0() {
    let titulo = String::from(" Option - Intro 0 ");
    imprime_titulo(&titulo);

    let identidad = match identificador(1) {
        Some(id) => id,
        None => {
            println!("Identidad no existe");
            return;
        }
    };
    println!("Identidad: {}", identidad);
}
//************************************* option_intro_1
#[allow(dead_code)]
pub fn option_intro_1() -> Option<()> {
    let titulo = String::from(" Option - Intro 1 ");
    imprime_titulo(&titulo);

    let identidad = identificador(2)?;
    println!("Identidad: {}", identidad);
    // Si identidad es None, no imprime nada
    Some(())
}

//************************************* Prueba con propio enum Opciones
enum Opciones<T> {
    None,
    Some(T),
}

fn identificador_enum_opciones(id: u8) -> Opciones<std::string::String> {
    if id == 1 {
        return Opciones::Some("Id UNO".to_string());
    } else if id == 2 {
        return Opciones::Some("Id DOS".to_string());
    }
    return Opciones::None;
}

#[allow(dead_code)]
pub fn option_intro_2() {
    let titulo = String::from(" Option - Intro 2 ");
    imprime_titulo(&titulo);

    let identidad = match identificador_enum_opciones(3) {
        Opciones::Some(id) => id,
        Opciones::None => {
            println!("Identidad no existe");
            return;
        }
    };
    println!("Identidad: {}", identidad);
}

//***************************************************************************** Option - Detalles
/* Explicación  
Esta función puede devolver Some(i32) si existe el índice, o None cuando no
existe. En este caso, el valor de retorno cuando existe, i32, estará "envuelto"
en un tipo Option.
*/

fn toma_valor(valor: &Vec<i32>, var_posicion:usize) -> Option<i32> {
    println!("Posición interrogada: {}", var_posicion);
    println!("Contenido en: {:?}, Número de elementos: {:?}", valor, valor.len());
    if valor.len() < var_posicion {     // .len() devuelve el número de elementos del vector.
                                        // Debe ser menos de 5 si se quiere recuperar el dato. 
        None                            // cuando no lo es, devuelve None
    } else {
        Some(valor[var_posicion])
    }
}

//************************************* option_0
#[allow(dead_code)]
pub fn option_0() {
    let titulo = String::from(" Option - Detalles 0 ");
    imprime_titulo(&titulo);

    let posicion:usize = 1;
    let vec_0 = vec![1, 2];
    let vec_1 = vec![1, 2, 3, 4, 5];
    println!("Contenido posición {:?} del vec_0, {:?}", &posicion, toma_valor(&vec_0, posicion));
    println!("Contenido posición {:?} del vec_1, {:?}", &posicion, toma_valor(&vec_1, posicion));
    
/* Explicación  
Para obtener el valor envuelto, se puede usar alguna de las funciones que tiene el tipo Option.
La función unwrap() recupera el valor contenido en el Some, pero entra en pánico si es un None.

    La siguiente línea falla con unwrap ya que contiene None. -->
*/
    // println!("Contenido posición {:?} del vec_0, {:?}", &posicion, toma_valor(&vec_0, posicion).unwrap());
    println!("Contenido posición {:?} del vec_1, {:?} (unwrap)", &posicion, toma_valor(&vec_1, posicion).unwrap());
}

//************************************* option_1
#[allow(dead_code)]
pub fn option_1() {
    let titulo = String::from(" Option - Detalles 1 ");
    imprime_titulo(&titulo);

    let posicion:usize = 3;
    let vec_0 = vec![1, 2];
    let vec_1 = vec![1, 2, 3, 4, 5];

    // Se crea un vector para guardar los valores option. El vector es de tipo: Vec<Option<i32>>.
    // Es decir, un vector de Option<i32>.
    let mut option_vec = Vec::new(); 

    option_vec.push(toma_valor(&vec_0, posicion)); // Guarda "None" en el vec
    option_vec.push(toma_valor(&vec_1, posicion)); // Guarda "Some(5)" en el vec

    manejar_opcion(option_vec);
    // Revisa el vector y realiza la acción que corresponda, imprime el valor si es un Some.
    // Y indica si es un None.
}

fn manejar_opcion(mi_opcion: Vec<Option<i32>>) {
    for item in mi_opcion {
      match item {
        Some(numero) => println!("¡Encontré un {} en esa posición", numero),
        None => println!("¡Posición excede longitud del vector!"),
      }
    }
  }

//************************************* option_2
/* Explicación  
En el código siguiente, se usa el método .is_some() para preguntar si el tipo del Option es Some,
también hay otro método complementario denominado .is_none().
*/

#[allow(dead_code)]
pub fn option_2() {
    let titulo = String::from(" Option - Detalles 2 ");
    imprime_titulo(&titulo);

    let posicion:usize = 4;
    let vec_0 = vec![1, 2];
    let vec_1 = vec![1, 2, 3, 4, 5];

    

    let vec_of_vecs = vec![vec_0, vec_1];
    for vec in vec_of_vecs {
        println!("Contenido en: {:?}", vec);
        let numero_interno = toma_valor(&vec, posicion);
        if numero_interno.is_some() {
            // .is_some() devuelve true si es Some, false si es None
            // ahora es seguro usar .unwrap() ya que es seguro que es Some
            println!("Contenido posición {:?} del vec, {:?}", &posicion, numero_interno.unwrap());
            // println!("Tenemos: {}", numero_interno.unwrap()); 
        } else {
            println!("No tenemos nada.");
        }
    }
}

//***************************************************************************** Result - Intro 0
/* Notas:       
Result trata sobre Ok o Error. La existencia de un resultado correcto o no (la
existencia de un error).

            enum Result<T, E> {
                Ok(T),
                Err(E),
}
*/

/* Explicación  
Al tener dos parámetros de tipo, Result<T, E>, se debe indicar qué se devuelve cuando
el resultado es correcto, Ok(T), y qué se devuelve cuando el resultado es erróneo, Err(E). 
*/
fn resultado(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {return Ok(())} else {return Err(())}
}

fn comprueba_valor(numero: i32) -> Result<i32, String> {
    match numero {
        4 => Ok(numero),
        _ => Err("El número no es cuatro.".to_string()),             // Este es el mensaje de error
    }
}

#[allow(dead_code)]
pub fn result_0() {
    let titulo = String::from(" Result - Intro 0 ");
    imprime_titulo(&titulo);
    /*
    Al igual que para Option, para Result, dos métodos, respectivamente, para chequear de
    forma sencilla el tipo concreto, son: .is_some(), is_none(), is_ok() y is_err().
    */
    if resultado(5).is_ok() {println!("Es correcto")} else {println!("Es un error")}
    if resultado(4).is_err() {println!("Es un error")} else {println!("Es correcto")}

    //*************************************
    for numero in 0..=6 {
        println!("El número {}: {:?}", numero, comprueba_valor(numero));
        // Para evitar Panic al usar unwrap -->
        if comprueba_valor(numero).is_ok() {
            println!("El número {}: {:?}", numero, comprueba_valor(numero).unwrap());
        } else {
            println!("El número {}: {:?}", numero, comprueba_valor(numero));
        }
    }
/*    ************************************* Guardando en un vector de resultados    
    let mut result_vec = Vec::new();      // Crea un vector para contener el resultado

    for numero in 0..=6 {
        result_vec.push(comprueba_valor(numero));                   // Guarda cada resultado
    }
    println!("{:?}", result_vec);
    */
}


//***************************************************************************** Result - Intro 1
#[allow(dead_code)]
pub fn result_1() {
    let titulo = String::from(" Result - Intro 1 ");
    imprime_titulo(&titulo);
    /* Nota:    
    El uso de match con Option y Result puede requerir mucho código. Por ejemplo, el
    método .get() devuelve un Option sobre un Vec.
    */
    let mut vector = vec![1, 2, 3, 6, 5, 4];
    let get_one = vector.get(0);    // recupera el primer número
    let get_two = vector.get(10);   // recupera None
    println!("{:?}", get_one);
    println!("{:?}", get_two);

    //************************************* Obtener los valores usando match
    for index in 0..10 {
        match vector.get(index) {
          Some(number) => println!("Obtener los valores usando match: {}", number),
          None => {}
        }
    }

    //************************************* Obtener los números usando if let Some
    /* Importante:  
    if let Some(number) = vector(index) significa que "compruebe si el valor de vector(index)
    es compatible con Some(number).
    */
    for index in 0..10 {
        if let Some(number) = vector.get(index) {
            println!("Obtener los valores usando if let Some: {}", number);
        }
    }

    //************************************* 

    let mut indice = 0;
    while let Some(numero) = vector.get(indice) {
        println!("Obtener los valores usando while let Some y .get: {}", numero);
        indice += 1; // Incrementar el índice en cada iteración
    }        
 
    while let Some(valor) = vector.pop() {
        println!("Obtener los valores usando while let Some y .pop: {}, Longitud del vector: {}", valor, vector.len());
    }

}

