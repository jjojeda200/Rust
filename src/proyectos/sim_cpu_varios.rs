/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          26-03-2023
    Titulo:         Simulación CPU
    Descripción:    Rutinas variadas
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
/* Detalles
*/
#![allow(dead_code)]
#![allow(unused_variables)]

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** 
pub fn ejemplo_impresion_datos_hex() {
    // Vector de 256 elementos u8
    let mut data = [0u8; 256];
    // Llenar el vector con datos de prueba
    for i in 0..data.len() {
        data[i] = i as u8;
    }
    // Imprimir los datos en formato hexadecimal
    let mut line_count = 0;
    for (i, byte) in data.iter().enumerate() {
        if i % 15 == 0 {
            print!("\n{:04X}: ", i);
            line_count += 1;
            if line_count == 16 {
                line_count = 0;
                // Esperar por una pulsación de tecla
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if data[i] == 0xFF {
                    break;
                }
            }
        }
        print!("{:02X} ", byte);
    }
}

//***************************************************************************** 


//***************************************************************************** 