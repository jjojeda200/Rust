/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          06-03-2023
    Titulo:         introducción a RUST
    Descripción:    Iteradores
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

#![allow(dead_code)]
#![allow(unused_variables)]

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

/* Tipos de iterador:               
    .iter() para iterar a través de referencias a los elementos, itera sobre &T.
    .iter_mut() para iterar mediante referencias modificables (mutables), itera sobre &mut T.
    .into_iter() para obtener un iterador sobre valores, no referencias, itera sobre T.

Un bucle for es solo un iterador que es propietario de sus valores. Es por eso
que se puede hacer modificable y se pueden cambiar los valores cuando se utiliza.
*/

//*****************************************************************************
pub fn metodos_iter_0() {
    let titulo = String::from(" Introducción a Iteradores 0 ");
    imprime_titulo(&titulo);

    let vector1: Vec<u8> = vec![1, 2, 3, 4]; 

    // Explicación detallada:
    /* vector1.iter().map(|x: &u8| x + 1).collect::<Vec<u8>>()       
    vector1.iter() devuelve un iterador que produce referencias a cada elemento
    en el vector vector1. Luego, se llama al método .map() en el iterador devuelto,
    que toma como argumento una función de clausura que toma una referencia a cada
    elemento en el vector y devuelve un nuevo valor. En este caso, la función de
    clausura es |x| x + 1, que toma una referencia a cada elemento en el vector y
    le suma 1, devolviendo el nuevo valor resultante.

    La función de clausura se aplica a cada elemento del iterador devuelto por .iter(),
    uno a la vez, y se devuelve un nuevo iterador que produce los valores resultantes
    de la función de clausura. El nuevo iterador producido por .map() se puede consumir
    o recopilar en un nuevo vector usando el método collect().

    El método collect() en el nuevo iterador para crear un vector de bytes (Vec<u8>)
    que contiene los valores modificados. El método collect() toma un tipo de colección
    como argumento y devuelve una nueva colección que contiene los elementos del iterador
    original. En este caso, el tipo de colección especificado es Vec<u8>.
    */
    println!("{:?}", vector1.iter());
    println!("{:?}", vector1.iter().map(|x: &u8| x + 1));
    println!("{:?}", vector1.iter().map(|x| x + 1).collect::<Vec<u8>>());
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<u8>>();
    //println!("{:?}", vector1_a);

    /* Explicación detallada:       
    vector1_b se crea utilizando el método .into_iter() para obtener una referencia
    mutable a cada elemento del vector, y aplicando una función de mapeo para
    multiplicar cada elemento del vector original por 10.
    .into_iter() toma posesión de vector1 .
    */
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<u8>>();
    println!("{:?}", vector1_b);    // vector1 movido debido a esta llamada de método
                                    // println!("{:?}", vector1);

    /* Explicación detallada:       
    En vector2, se utiliza el método .iter_mut() para obtener una referencia mutable
    a cada elemento del vector y utilizar la función for_each() para agregar 100 a
    cada elemento del vector.
    */
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 100);
    println!("{:?}", vector2);

    let numbers: Vec<u8> = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("La suma de todos los elementos {}", sum);

}

//*****************************************************************************
pub fn metodos_iter_1() {
    let titulo = String::from(" Introducción a Iteradores 1 ");
    imprime_titulo(&titulo);

    let my_vec = vec![8, 9, 10];

    let valor = my_vec.get(3).unwrap_or_else(|| {  // Intenta "desenvolver". Si no funciona:
        // observa si el vector contiene algo en el índice [0]
        if my_vec.get(0).is_some() {
            &my_vec[0]      // Utiliza ese valor si existe algo.
        } else {
            &0              // en otro caso, usa el &0
        }
    });
    println!("Valor devuelto: {}", valor);

//*************************************
    let num_vec: Vec<u8> = vec![2, 4, 6, 8];

    // usa el vector para recorrerlo (iterar)
    let double_vec = num_vec              
        .iter()
        // para cada elemento, lo multiplica por 2
        .map(|number| number * 2)
        // y crea un nuevo vector a partir de ello
        .collect::<Vec<u8>>();
    println!("El vector resultante: {:?}", double_vec);

//*************************************
    num_vec
        // itera sobre num_vec
        .iter()
        // obtiene pares de (índice, valor)
        .enumerate()
        // imprime para cada elemento
        .for_each(|(index, number)| println!("El índice {} contiene el valor {}", index, number)); 


//*************************************
    let x =  num_vec
        .iter()
        .enumerate()
        .map(|(index, number)| println!("El índice {} contiene el valor {}", index, number));
    println!("\n{:?}", num_vec.iter());
    println!("{:?}", num_vec.iter().enumerate());
    let y = num_vec
        .iter()
        .enumerate()
        .map(|(index ,number)|(index ,number))
        .collect::<Vec<_>>();
    println!("Valor de x: {:?}", x);
    println!("Valor de y: {:?}", y);
}

//***************************************************************************** Iteradores 2 - String
#[derive(Debug, Clone)]
struct Library {
    books: Vec<String>,
}

/* Explicación detallada:       
La implementación de la estructura Library proporciona dos métodos: add_book y new.
El método add_book toma una referencia mutable &mut self a la instancia de la
estructura Library y una referencia &str al título del libro a agregar. Dentro de
este método, se utiliza el método to_string para convertir la referencia de &str
a una cadena String, y luego se agrega esta cadena al vector books utilizando el
método push.

El método new es un método de constructor que devuelve una nueva instancia de la
estructura Library. En este método, se crea un nuevo vector vacío utilizando el
método Vec::new(), y se inicializa el campo books con este vector vacío.
*/
impl Library {
    fn add_book(&mut self, book: &str) { self.books.push(book.to_string()); }

    fn new() -> Self {
        Self {
            // la mayor parte de las veces, valor por defecto
            books: Vec::new(),
        }
    }
}

/* Explicación detallada:       
Este código implementa el iterador Iterator para la estructura Library. El iterador
permite recorrer los elementos de la biblioteca de manera secuencial, de manera que
se puede utilizar en un bucle for para realizar alguna acción en cada libro.

La implementación del iterador comienza con la declaración de type Item = String,
que especifica que el tipo de elemento que se va a devolver es una cadena.

El método next es el que se utiliza para avanzar en la secuencia de elementos.
Este método toma una referencia mutable &mut self a la estructura Library para
poder manipular su estado interno. Devuelve una opción Option<String> que puede
contener una cadena (Some) si hay más elementos, o no hay elementos (None) si
se ha llegado al final de la secuencia.

El método next utiliza el método pop del vector books de la estructura Library
para obtener y eliminar el último libro de la biblioteca. Si el método pop devuelve
Some(book), se agrega la cadena " encontrado" al final del libro utilizando la
operación +. Esto crea una nueva cadena que contiene el título del libro más la
palabra "encontrado". El método next devuelve entonces Some(book) con la cadena
modificada.

Si el método pop devuelve None, significa que no hay más libros en la biblioteca
y el método next devuelve None.
*/
impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " encontrado"), // Rust permite String + &str
            None => None,
        }
    }
}

pub fn metodos_iter_string_0() {
    let titulo = String::from(" Introducción a Iteradores 2 - String ");
    imprime_titulo(&titulo);

    let mut my_library = Library::new();
    my_library.add_book("Libro 1");
    my_library.add_book("Libro 2");
    my_library.add_book("Libro 3");
    my_library.add_book("Libro 4");

    for item in my_library.clone() { // Se pasa un clon de la biblioteca para que no se destruya
        println!("{}", item);
    }
}

//*****************************************************************************