/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          01-02-2023
    Titulo:         introducción a RUST
    Descripción:    Traits (Rasgos)
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
#[allow(unused_variables)]

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}
//***************************************************************************** 




//***************************************************************************** Método windows
/* Notas:   
https://www.youtube.com/watch?v=ykQbsTHqKFo&t=42s
Fragmento de código que utiliza el método windows() en un vector valores.
El método windows() devuelve un iterador sobre ventanas de tamaño fijo que se
mueven a lo largo del vector. En este caso, se está iterando sobre ventanas de
tamaño 4.

Luego se utiliza el método find() para buscar una ventana que coincida
exactamente con el arreglo [3, 4, 5, 6]. Si se encuentra una ventana que
coincide, se devuelve Some(window), de lo contrario se devuelve None.

Finalmente, se utiliza el método is_some() para determinar si se encontró una
ventana que coincida. Si se encontró una ventana, result tendrá un valor de true,
de lo contrario tendrá un valor de false.

La función dbg!() se utiliza para imprimir el valor de result en la consola.
*/
#[allow(dead_code)]
pub fn fn_windows() {
    let titulo = String::from(" Método windows ");
    imprime_titulo(&titulo);

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = values
        .windows(4)
        .find(|window| window == &[3, 4, 5, 6])
        .is_some();
    dbg!(result);
}

//***************************************************************************** Método Crate Itertools
/* Notas:   
https://www.youtube.com/watch?v=ykQbsTHqKFo&t=42s
Este es un ejemplo de código en Rust que utiliza el crate (paquete) itertools y
define un trait (rasgo) personalizado llamado "Prefix". El trait proporciona un
método "has_prefix" que verifica si una secuencia comienza con una sub-secuencia
dada.
El método utiliza los métodos "iter" y "positions" del tipo de referencia "&[T]"
para encontrar la posición del primer elemento de la sub-secuencia en la secuencia
y luego utiliza la función "find" para verificar si la sub-secuencia completa
comienza en esa posición. El método devuelve true si la sub-secuencia se encuentra
al principio de la secuencia y false en caso contrario. El programa principal
define dos vectores de diferentes tipos de datos (enteros y flotantes) y llama al
método "has_prefix" para verificar si la sub-secuencia [3, 4, 5] está presente al
principio del vector de enteros y si la sub-secuencia [4.20, 5.0] está presente al
principio del vector de flotantes. El resultado de cada llamada se imprime mediante
la función "dbg".
*/
use itertools::Itertools;

#[allow(dead_code)]
pub fn fn_trait() {
    let titulo = String::from(" Crate Itertools + Trait ");
    imprime_titulo(&titulo);
    
    let integers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let i_slice = &integers[..];
    let integer_result = i_slice.has_prefix(&[3, 4, 5]);
    dbg!(integer_result);

    let floats = vec![1.0, 4.0, 5.0, 7.0];
    let float_result =
        floats.as_slice().has_prefix(&[4.20, 5.0]);
    dbg!(float_result);
}

trait Prefix<T> {
    fn has_prefix(&self, prefix: &[T]) -> bool;
}

impl<T> Prefix<T> for &[T]
where
    T: PartialEq,
{
    fn has_prefix(&self, prefix: &[T]) -> bool {
        self.iter()
            .positions(|v| *v == prefix[0])
            .find(|&index| {
                let range = index..(index + prefix.len());
                self[range] == *prefix
            })
            .is_some()
    }
}