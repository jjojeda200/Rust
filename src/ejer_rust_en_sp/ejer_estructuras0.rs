/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-02-2023
    Titulo:         introducción a RUST
    Descripción:    Estructuras / Tipos de colección
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
//***************************************************************************** Array
/* Nota:
Los Arrays:
- No pueden cambiar de tamaño.
- Tienen datos del mismo tipo.
- Los números de índice comienzan en 0 (no en 1).
- los rangos son excluyentes (es decir, no incluyen el último número).
*/
#[allow(dead_code)]
pub fn arreglos(num: u8) {
    let titulo = String::from(" Arrays / Arreglos ");
    imprime_titulo(&titulo);

    let arr0 = [2, 4, 6, 8, 0];
    let arr1: [u8; 4] = [2, 4, 6, 8];
    let arr2 = [num; 6];
    let arr3 = ["X"; 6];

    println!("arr0: pos 0 = {}, pos 2 = {}", arr0[0], arr0[2]);

    let mut j = 0;
    println!("arr1: tamaño arreglo {}", arr1.len());
    while j < arr1.len() {
        println!("arr1: pos = {}", arr1[j]);
        j += 1;
    }

    for i in arr1.iter() {
        println!("arr1: valor = {}, pos = {:?}", i, arr1.into_iter());
        println!("arr1: valor = {}, pos = {:?}", i, arr1.as_ptr_range());
    }

    println!(
        "arr2: pos 0 = {:x}, pos 2 = {}, tamaño arreglo {}",
        arr2[0],
        arr2[2],
        arr2.len()
    );

    println!(
        "arr3: pos 0 = {:?}, pos 2 = {}, tamaño arreglo {}",
        arr3[0],
        arr3[2],
        arr3.len()
    );
    println!("arr3: {:?}\n", arr3);

    let array_de_diez = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let tercero_a_quinto = &array_de_diez[2..5];
    let segundo_a_fin = &array_de_diez[1..];
    let inicio_a_quinto = &array_de_diez[..5];
    let todos = &array_de_diez[..];

    println!(
        "Tres a quinto:\t{:?}\nSegundo a fin:\t{:?}\nCero a quinto:\t{:?}\ntodo:\t\t{:?}",
        tercero_a_quinto, segundo_a_fin, inicio_a_quinto, todos
    );
}

//***************************************************************************** Vector
/* Nota:    
Los Vec siempre contienen valores y para eso sirven <> (paréntesis angulares).
Todos los elementos de un vector tienen que tener un mismo tipo.
Ejemplos:
- Un Vec<String> es un vector que contiene elementos String
- Vec<(i32, i32)> es un vector en el que cada elemento de contenido es una tupla
(i32, i32).
- Vec<Vec<String>> es un vector en el que cada elemento es otro vector de String.
*/
#[allow(dead_code)]
pub fn vector() {
    let titulo = String::from(" Vectores ");
    imprime_titulo(&titulo);

    let mut v: Vec<u8> = vec![10, 11, 12, 13, 14, 15];
    println!("Capacidad del vector {}", v.capacity());
    v.push(20);

    println!("vector {:?}", v);
    println!("Capacidad del vector {},  {:?}", v.capacity(), v.as_ptr());
    let direccion = v.as_ptr();
    imprimir_contenido(direccion);

    // Nombrando posiciones
    let str_vec = vec!["Uno", "Dos", "Tres"];
    // Las variables serán: b para str_vec[1], y c para str_vec[2]
    let (_, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("str_vec[1] por la variable asignada b: {:?} y str_vec[2] por la varibal asignada c: {}", b, c);
    println!("str_vec[0] por su posicion en el vec : {:?}", str_vec[0]);
}

fn imprimir_contenido(direccion: *const u8) {
    let contenido = unsafe {
        let puntero = direccion as *const u8;
        *puntero
    };
    let puntero = direccion as *const u8;
    println!("Contenido de la dirección {:?}: {:X}", direccion, contenido);
    unsafe {
        println!(
            "Contenido de la dirección {:?}: {:X}\n",
            direccion, *puntero
        );
    }
}

//***************************************************************************** Tuplas
/* Nota:    
Una función que no recibe ningún parámetro (recibe una tupla vacía). Al añadir
un ; como último dato antes de terminar la función se está indicando que se
debe retornar una tupla vacía.
*/
#[allow(dead_code)]
pub fn tupla() {
    let titulo = String::from(" Tuplas ");
    imprime_titulo(&titulo);
    salto_print();
    // Esta tupla es de tipo (&str, i32, Vec<char>, char, [i32; 3], f64).
    let random_tuple = ("Esto es un texto", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "El interior de la tupla contiene: 
Primer elemento : {:?}
Segundo elemento: {:?}
Tercer elemento : {:?}
Cuarto elemento : {:?}
Quinto elemento : {:?}
Sexto elemento  : {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );
}

fn salto_print() {
    println!("Estoy imprimiendo");
    // Al añadir un como último dato antes de terminar la función se está
    // indicando que se debe retornar una tupla vacía.
}

//***************************************************************************** Estructura
/* Nota:    
Se utiliza struct cuando un tipo de datos debe representar una cosa Y otra cosa
a la vez. Las estructuras sirven para unir diferentes elementos en uno solo.

Existen tres tipos de estructuras:
- Estructura unitaria "unit struct", que no tiene nada. Simplemente se escribe
  su nombre seguido de un punto y coma.
- Estructura tupla, o estructura sin nombres. Solo es necesario escribir los
  tipos de dato que contiene, sin nombres de campo.
- Estructura con nombres. En estas se declaran los nombres de los campos y sus
  tipos en un bloque {}. Estos bloques no se terminan con punto y coma.
Los campos de una estructura con nombres se separan con comas.
*/
struct Point {
    x: u8,
    y: u8,
}

#[allow(dead_code)]
pub fn estructura() {
    let titulo = String::from(" Estructura ");
    imprime_titulo(&titulo);
    
    let n: u8 = 64;
    println!("n es: Dec {}, Hex {:x}, char {}", n, n, n as char);

    // Instanciar a `Point`
    let point: Point = Point { x: 10, y: 11 };
    // Acceder a los campos del punto
    println!("Valores X: ({:x}, y Y: {:x})", point.x, point.y);

    // Instanciar a `Point`
    let point: Point = Point { x: 14, y: 15 };
    println!("Valores X: ({:x}, y Y: {:x})\n", point.x, point.y);
}

//***************************************************************************** Enumerador
/* Nota:    
Se utiliza enum cuando un tipo de datos puede representar una cosa O alguna
cosa diferente.
Los enumerados permiten que un tipo de datos represente a diferentes cosas en
diferente momento.
*/

#[derive(Debug)]
enum Estado {
    Inicia,
    Ejecuta { nivel: u32 },
    Finaliza(Animación),
}

#[derive(Debug)]
enum Animación {
    Ejecutando,
    Parando,
}

#[allow(dead_code)]
pub fn enumeradores() {
    let titulo = String::from(" Enumarador ");
    imprime_titulo(&titulo);

    let mut estado = Estado::Inicia;
    println!("{:?}", estado);
    loop {
        match estado {
            Estado::Inicia => {
                println!("Iniciando");
                estado = Estado::Ejecuta { nivel: 100 };
            }
            Estado::Ejecuta { nivel: 0 } => {
                estado = Estado::Finaliza(Animación::Ejecutando);
                println!("Finalizando");
            }
            Estado::Ejecuta { ref mut nivel } => {
                *nivel -= 25;
                print!("Bajando nivel\t");
            }
            Estado::Finaliza(Animación::Ejecutando) => {
                println!("Transición");
                estado = Estado::Finaliza(Animación::Parando);
            }
            Estado::Finaliza(Animación::Parando) => break,
        }
    }
    println!("Fín");
}

//***************************************************************************** Desestructurar
/* Nota:    
La desestructuración permite desempaquetar los valores de una estructura de
datos en variables individuales mediante la asignación a una lista de variables
con nombres específicos que representan cada uno de los campos de la estructura.
La desestructuración se puede utilizar con tuplas, matrices y otros tipos de datos.  
*/
struct Persona {
    nombre: String,
    edad: i32,
    altura: f32,
}

/* Descripción: 
Se define una estructura Persona con tres campos. Se crea una instancia de esta
estructura y se le asigna a la variable persona. Finalmente, se desestructura la
variable persona en las variables nombre, edad y altura, que luego se utilizan
para imprimir sus valores en la consola.
*/
#[allow(dead_code)]
pub fn desestructurar(){
    let titulo = String::from(" Desestructurar ");
    imprime_titulo(&titulo);
    
    let persona = Persona { 
        nombre: String::from("Juan"),
        edad: 30,
        altura: 1.70,
    };

    let Persona { nombre, edad, altura } = persona;
    println!("Nombre: {}, Edad: {}, Altura: {}", nombre, edad, altura);
}

//***************************************************************************** Referencias y el operador punto .
/* Nota:    
Cuando se usa el operador . (punto), no se necesita utilizar el operador * para
desreferenciar.
*/

struct Item {
    numero: u8,
}

impl Item {
    fn compara_numero(&self, otro_numero: u8) {     // tiene una referencia a self
        println!("¿Son {} y {} iguales? {}", self.numero, otro_numero, self.numero == otro_numero);
        // No se necesita escribir (*self).numero
    }
}

#[allow(dead_code)]
pub fn operador_punto() {
    let titulo = String::from(" Operador . (punto) ");
    imprime_titulo(&titulo);

    let item = Item { numero: 8 };

    let referencia_numero = &item.numero;      // el tipo de referencia_numero es &u8
    println!("{}", *referencia_numero == 8);        // así funciona

    let referencia_item = &item;             // el tipo de referencia_item es &Item
    println!("{}", referencia_item.numero == 8);    // así funciona

    // Con método "compara_numero()"
    let item_referencia0 = &item;                // De tipo &Item
    let item_referencia1 = &item_referencia0 ;  // De tipo &&Item

    item.compara_numero(8);                // El método funciona
    item_referencia0.compara_numero(8);    // El método funciona
    item_referencia1.compara_numero(8);    // El método funciona
}

//*****************************************************************************