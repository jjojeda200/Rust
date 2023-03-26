/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          26-03-2023
    Titulo:         introducción a RUST
    Descripción:    Jugando con la memoria, punteros, referencias, etc.
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

/* función imprime_titulo   
La función imprime_titulo(titulo: &String) recibe como parámetro un puntero a
una cadena de texto String y utiliza la macro println!() para imprimir el valor
de la cadena de texto centrado en 80 caracteres y rodeado por asteriscos.
*/
fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** 
/* Notas:                   

*/
const COL_POR_DEFECTO: usize = 16;

fn calcula_lineas(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        lineas + 1
    } else {
        lineas
    }
}

fn muestra_linea(mem: &[u8], ascii: bool, ancho: usize) -> String {
    let mut linea = format!("{:16p} |", mem.as_ptr());
    for i in 0..ancho {
        if i < mem.len() {
            linea += &format!(" {:02x}", mem[i]);
        } else {
            linea += "   ";
        }
    }
    if ascii {
        linea += " | ";
        for i in 0..ancho {
            if i < mem.len() {
                let var_char = if mem[i] > 32 && mem[i] < 126 {
                    mem[i] as char
                } else {
                    '.'
                };
                linea.push(var_char);
            } else {
                linea.push(' ');
            }
        }
    }
    linea.push('\n');
    linea
}

fn muestra(mem: &[u8], size: usize, ascii: bool, ancho: usize) {
    println!("   Dir. Memoria  | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("  -------------- | -----------------------------------------------");
    let lineas = calcula_lineas(size, ancho);
    let mut mem_offset = 0;
    for i in 0..lineas {
        let linea = muestra_linea(&mem[mem_offset..], ascii, ancho);
        print!("{}", linea);
        mem_offset += ancho;
    }
    println!("  ---------------|------------------------------------------------");
}

fn main() {
    let mut mem = [0u8; 256]; // Vector de 256 elementos u8
    // Llenar el vector con datos de prueba
    for i in 0..mem.len() {
        mem[i] = i as u8;
    }
    //let mem = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    muestra(&mem, mem.len(), true, COL_POR_DEFECTO);
}
//***************************************************************************** 
use std::mem;

const COL_POR_DEFECTO: usize = 16;

fn calcula_lineas(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        lineas + 1
    } else {
        lineas
    }
}

fn muestra_linea_mem(mem: &[u8], ancho: usize) {
    print!("{:16p} ||", mem.as_ptr());
    for i in 0..ancho {
        print!(" {:02x}", mem.get(i).unwrap_or(&0));
    }
    print!(" || ");
    for i in 0..ancho {
        let var_char = if mem.get(i).unwrap_or(&0).is_ascii_graphic() {
            mem.get(i).unwrap_or(&0).to_owned() as char
        } else {
            '.'
        };
        print!(" {}", var_char);
    }
    println!();
}

fn muestra_mem(mem: &[u8], size: usize, ancho: usize) {
    println!("   Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("  -------------- || -----------------------------------------------");
    let lineas = calcula_lineas(size, ancho);
    for i in 0..lineas {
        let offset = i * ancho;
        muestra_linea_mem(&mem[offset..], ancho);
    }
    println!("  -----------------------------------------------------------------");
}

fn muestra_mem_obj<T>(var_a: &T) {
    let var_ptr = var_a as *const T as *const u8;
    println!("--> Tamaño ocupado en bytes ({})", mem::size_of::<T>());
    muestra_mem(unsafe { std::slice::from_raw_parts(var_ptr, mem::size_of::<T>()) }, mem::size_of::<T>(), COL_POR_DEFECTO);
}

pub fn memoria_1() {
    let mut var_a_array: [i32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

    let var_ptr = &var_a_array as *const [i32; 8] as *const u8;

    println!("  Dirección de inicio del array          ({:?})\n", var_ptr);
    muestra_mem_obj(&var_a_array);

    println!();

    let mut var_a_vector: Vec<i32> = vec![1, 2, 4, 8, 16, 32, 64, 128];

    muestra_mem_obj(&var_a_vector);
    println!("Vector .Begin {:p}", var_a_vector.as_ptr());
    // println!("Vector .[0]   {:p}", &var_a_vector[0] as *const i32);
    println!("Vector .[8]   {:p}", &var_a_vector[7] as *const i32);
    println!("Vector .end   {:p}", unsafe{var_a_vector.as_ptr().add(var_a_vector.capacity())});
    println!("varAvector contiene {} elementos.", var_a_vector.len());
    println!("capacidad final={}", var_a_vector.capacity());
    var_a_vector.push(256);
    muestra_mem_obj(&var_a_vector);


    println!("Vector .Begin {:p}", var_a_vector.as_ptr());
    println!("Vector .[0]   {:p}", &var_a_vector[0]);
    println!("Vector .[16]  {:p}", &var_a_vector[var_a_vector.len()-1]);
    println!("Vector .end   {:p}", unsafe{var_a_vector.as_ptr().add(var_a_vector.len())});
    println!("var_a_vector contiene {} elementos.", var_a_vector.len());
    println!("capacidad final={}", var_a_vector.capacity());
}