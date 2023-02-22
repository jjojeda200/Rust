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

    Dependencias:

    Compilar:

    Licencia:

***************************************************************************************/

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
use std::collections::HashMap;  // HashMap en lugar de std::collections::HashMap
use std::collections::BTreeMap; // BTreeMap lugar de std::collections::BTreeMap

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
