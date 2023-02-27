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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}
//***************************************************************************** 




//***************************************************************************** Método windows
/* Notas:   
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

//***************************************************************************** Método windows
/* Notas:   

*/
use itertools::Itertools;

pub fn fn_trait() {
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