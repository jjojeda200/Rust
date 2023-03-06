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
/* Notas

*/
pub fn metodos_iter_0() {
    let titulo = String::from(" Introducción a Iteradores ");
    imprime_titulo(&titulo);
    
    // Se usa .iter() y .into_iter() sobre este vector
    let vector1 = vec![1, 2, 3]; 

    /*
    vector1_a se crea utilizando el método .iter() para obtener una referencia
    inmutable a cada elemento del vector, y aplicando una función de mapeo para
    sumar 1 a cada elemento del vector original.
    */
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();

    /*
    vector1_b se crea utilizando el método .into_iter() para obtener una referencia
    mutable a cada elemento del vector, y aplicando una función de mapeo para
    multiplicar cada elemento del vector original por 10.
    */
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

    /*
    En el segundo vector, vector2, se utiliza el método .iter_mut() para obtener
    una referencia mutable a cada elemento del vector y utilizar la función for_each()
    para agregar 100 a cada elemento del vector.
    */
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 100);

    let numbers: Vec<u8> = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);

    println!("{:?}", vector1_a);
    println!("{:?}", vector2);
    println!("{:?}", vector1_b);
}

//*****************************************************************************
