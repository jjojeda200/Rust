/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-02-2023
    Titulo:         introducción a RUST
    Descripción:    Colecciones HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap, VecDeque
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

use std::collections::HashMap;  // HashMap en lugar de std::collections::HashMap
use std::collections::BTreeMap; // BTreeMap lugar de std::collections::BTreeMap

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** HashMap
/* Notas:   
HashMap es una colección compuesta por claves y valores. Se puede usar la clave
para recuperar el valor que se almacenó con ella. Se puede crear un HashMap con
HashMap::new() y se pueden insertar nuevos elementos mediante .insert(clave, valor).
- Los HashMap no están ordenados.
- Los BTreeMap mantienen el orden por clave.
*/
#[derive(Debug)]
struct Ciudad {
    nombre: String,
    // Almacenará el año (clave) y la población de cada año (valor)
    poblacion: HashMap<u32, u32>,
    // poblacion: BTreeMap<u32, u32>, 
}

#[allow(dead_code)]
pub fn hashmap_0() {
    let titulo = String::from(" HashMap ");
    imprime_titulo(&titulo);


    let mut telde = Ciudad {
        nombre: "Telde".to_string(),
        poblacion: HashMap::new(),      // En este momento el HashMap está vacío
    };
    telde.poblacion.insert(1372, 3_000);
    telde.poblacion.insert(1851, 24_000);
    telde.poblacion.insert(2020, 38_000);
    telde.poblacion.insert(1372, 3_500);    // Reescribe la pareja clave/valor

    // El tipo del Hashmap es HashMap<u32, u32>. Obtiene en cada iteración un par clave/valor
    for (año, poblacion) in telde.poblacion { 
        println!("En el año {} la ciudad de {} tenía una población de {}.", año, telde.nombre, poblacion);
    }
}

//***************************************************************************** BTreeMap
#[allow(dead_code)]
pub fn btreemap_0() {
    let titulo = String::from(" BTreeMap ");
    imprime_titulo(&titulo);

    let ciudades_canadieneses = vec!["Calgary", "Vancouver", "Gimli"];
    let ciudades_alemanas = vec!["Kar", "Bad Doberan", "Bielefeld"];

    // En este momento el BTreeMap está vacío
    let mut ciudad_btreemap = BTreeMap::new();

    for ciudad in ciudades_canadieneses {
        ciudad_btreemap.insert(ciudad, "Canadá");
    }
    for ciudad in ciudades_alemanas {
        ciudad_btreemap.insert(ciudad, "Alemania");
    }

    println!("{:?}", ciudad_btreemap["Bielefeld"]);
    println!("{:?}", ciudad_btreemap.get("Bielefeld"));
    println!("{:?}", ciudad_btreemap.get("Bielefeldd"));

    // is_none() devuelve un bool: true si es None, false si es Some
    let var = ciudad_btreemap.get(&"Kar").is_none();
    // println!("{}", var);
    match &var {
        false => println!("Exite"),
        _ => println!("No exite"),
    };
    
    if ciudad_btreemap.get(&"Kar").is_none(){
        ciudad_btreemap.insert("Kar", "Canadá");
    }
    if ciudad_btreemap.get(&"Ber").is_none(){
        ciudad_btreemap.insert("Ber", "Canadá");
    }
    println!("{:?}", ciudad_btreemap.get(&"Kar"));
    println!("{:?}", ciudad_btreemap.get(&"Ber"));

}

//***************************************************************************** Algunos métodos asociados 
/* Notas    .insert:    
Se utiliza para insertar un valor en un HashMap. La función toma dos 
argumentos: la llave(clave) y el valor.  Por ejemplo:

        let mut mi_map = HashMap::new();
        mi_map.insert("llave", "valor");
        println!("El mapa actual es: {:?}", my_map);
*/
/* Notas    .or_insert: 
Este método permite acceder a una entrada en un HashMap y insertar un valor 
predeterminado si la entrada no existe. Si la entrada existe, se devuelve 
una referencia mutable a ella. Por ejemplo:

        let mut my_map = HashMap::new();
        let entrada = my_map.entry("llave").or_insert("nuevo_valor");
        println!("El valor actual es: {}", entrada);
*/
/* Notas    .entry:     
Este método permite acceder a una entrada en un HashMap para su manipulación. 
Si la entrada existe, se devuelve una referencia mutable a ella. Si no
existe, se inserta una entrada con un valor predeterminado y se devuelve
una referencia mutable a ella. Por ejemplo:

        let mut my_map = HashMap::new();
        my_map.insert("llave", "valor");
        let entrada = my_map.entry("llave");
        entrada.insert("nuevo_valor");
*/
#[allow(dead_code)]
pub fn metodos_0() {
    let titulo = String::from(" Introducción a métodos adicionales ");
    imprime_titulo(&titulo);


let claves = vec!["Llave 1", "Llave 2", "Llave 3", "Llave 4", "Llave 5", "Llave 6"];

    // Inserta la primera clave "Llave 1"
    let mut mi_map = BTreeMap::new();
    mi_map.insert("Llave 1", 0);

    //*************************************
    println!("Comprueba si existen \"Llave 1\" y \"Lave 2\"");
    let mut existe = mi_map.entry("Llave 1");
    println!("Existe la clave \"Llave 1\": {:?}",existe);
    existe = mi_map.entry("Llave 2");
    println!("Existe la clave \"Llave 2\": {:?}", existe);

    //*************************************
    // Comprueba si existen y si es no, inserta las contenidas en el vector claves
    for var_llave in claves {
        mi_map.entry(var_llave).or_insert(0);
    }

    //*************************************
    // Comprueba si existen y si es no, lo inserta y le da valor, si es si, modifica
    // el valor. var_valor es una referencia mutable.
    existe = mi_map.entry("Llave 8");
    println!("Existe la clave \"Llave 8\": {:?}", existe);
    let var_valor = mi_map.entry("Llave 8").or_insert(0);
    *var_valor += 1;
    existe = mi_map.entry("Llave 8");
    println!("Existe la clave \"Llave 8\": {:?}", existe);
    println!("Tiene valor \"Llave 8\": {:?}", mi_map.get("Llave 8"));
    mi_map.remove_entry("Llave 8");
    existe = mi_map.entry("Llave 8");
    println!("Existe la clave \"Llave 8\": {:?}", existe);
    println!("Tiene valor \"Llave 8\": {:?}", mi_map.get("Llave 8"));

    //*************************************
    let var_valor = mi_map.entry("Llave 2").or_insert(0);
    *var_valor += 1;
    println!("Nuevo valor para la clave \"Llave 2\": {:?}", mi_map.entry("Llave 2"));
    println!("Tiene valor: {:?}", mi_map.get("Llave 2"));

    for (llave, num) in &mi_map {
        println!("{}, {}", llave, num);
    }
    println!("Mi Map: {:?}", mi_map);
}




//***************************************************************************** Algunos métodos asociados
#[allow(dead_code)]
pub fn metodos_1() {
    let titulo = String::from(" Ejemplo practico HashMap ");
    imprime_titulo(&titulo);

    let book_collection = vec!["Valor 1", "Valor 2", "Valor 3", "Valor 2"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        // return_value es una referencia mutable.
        // Si no contiene nada, se asigna un cero.
        *return_value +=1;  // Ahora return_value vale al menos 1.
                            // Y tenía algún valor, lo incrementa en uno
    }

    for (book, numero) in &book_hashmap {
        println!("{}, {}", book, numero);
    }

    println!("HashMap de libros: {:?}", book_hashmap);
}

//***************************************************************************** Vectores como valor
#[allow(dead_code)]
/* Notas:   
Para utilizar un vector como valor en un HashMap, puedes crear un HashMap cuyos
valores sean vectores
*/
pub fn hashmap_vectores() {
    let titulo = String::from(" Vectores como valor ");
    imprime_titulo(&titulo);

    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    
    map.insert("clave1".to_string(), vec![1, 2, 3]);
    map.insert("clave2".to_string(), vec![4, 5, 6]);
    map.insert("clave3".to_string(), vec![7, 8, 9]);

    println!("{:?}", map);

    map.entry("clave1".to_string()).or_insert(vec![]).push(4);
    map.entry("clave1".to_string()).or_insert(vec![]).push(5);
    map.entry("clave1".to_string()).or_insert(vec![]).push(6);

    println!("{:?}", map);
}