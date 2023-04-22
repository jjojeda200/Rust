/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-04-2023
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
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
//#![allow(unused_assignments)]

use colored::*;

/* función imprime_titulo   
La función imprime_titulo(titulo: &String) recibe como parámetro un puntero a
una cadena de texto String y utiliza la macro println!() para imprimir el valor
de la cadena de texto centrado en 80 caracteres y rodeado por asteriscos.
*/
fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.blue());
}

//***************************************************************************** 
/* Descripción Función:                     
COLUMNAS_POR_DEFECTO, es una constante que se utiliza para definir el ancho de cada línea en la función
muestra.

La función, calcula_lineas, calcula el número de líneas que se necesitarán para mostrar toda la región
de memoria en la consola. Recibe como entrada el tamaño total de la región de memoria y el ancho de cada
línea. La función divide el tamaño total por el ancho de línea para obtener el número de líneas completas,
y luego verifica si hay bytes adicionales que no caben en una línea completa. Si hay bytes adicionales, se
agrega una línea adicional para mostrarlos.
*/
const COLUMNAS_POR_DEFECTO: usize = 16;

fn calcula_lineas(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        lineas + 1
    } else {
        lineas
    }
}

/* Descripción Función:                     
La función, muestra_linea, recibe una sección de memoria de tamaño ancho y crea una cadena de caracteres
que muestra los bytes de la sección en formato hexadecimal y ASCII. La función comienza creando una cadena
que muestra la dirección de inicio de la sección de memoria y la marca de separación "|". Luego, itera a
través de los bytes de la sección, agregando su representación en hexadecimal a la cadena. Si la sección no
contiene suficientes bytes para llenar toda la línea, la función agrega espacios en blanco en su lugar. Si
se especifica la opción ascii, la función agrega una marca de separación "|" seguida de la representación
ASCII de los bytes de la sección.
*/
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

/* Descripción Función:                     
La función, muestra, es la función principal que muestra la región de memoria en la consola. Primero, imprime
una cabecera que indica el formato de la salida. Luego, calcula el número total de líneas que serán necesarias
para mostrar toda la región de memoria utilizando la función calcula_lineas. Luego, itera a través de cada línea
de la región de memoria, utilizando la función muestra_linea para mostrar cada línea en la consola. Finalmente, imprime una línea adicional para marcar el final de la salida.
*/
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

pub fn memoria_0() {
    let titulo = String::from(" Rutinas de salida por pantalla - Formato memoria Hex 0 ");
    imprime_titulo(&titulo);

    let mut mem = [0u8; 256];       // Vector de 256 elementos u8
    // Llenar el vector con datos de prueba
    for i in 0..mem.len() {
        mem[i] = i as u8;
    }
    //let mem = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    muestra(&mem, mem.len(), true, COLUMNAS_POR_DEFECTO);
}

//***************************************************************************** 
use std::mem;
/* Descripción Función:                     
COLUMNAS_POR_DEFECTO, es una constante que se utiliza para definir el ancho de cada línea en la función
muestra.

La función, calcula_lineas, calcula el número de líneas que se necesitarán para mostrar toda la región
de memoria en la consola. Recibe como entrada el tamaño total de la región de memoria y el ancho de cada
línea. La función divide el tamaño total por el ancho de línea para obtener el número de líneas completas,
y luego verifica si hay bytes adicionales que no caben en una línea completa. Si hay bytes adicionales, se
agrega una línea adicional para mostrarlos.
*/
const COL_POR_DEFECTO: usize = 16;

fn calcula_lineas_mem(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        lineas + 1
    } else {
        lineas
    }
}

/* Descripción Función:                     
- La función muestra_linea_mem es una función en Rust que imprime una línea de memoria en formato hex y ASCII
- print!("{:16p} ||", mem.as_ptr()); Imprime la dirección de memoria base de la línea actual en formato hex
 con una ancho de 16 caracteres, seguido de ||.
- for i in 0..ancho { print!(" {:02x}", mem.get(i).unwrap_or(&0)); } Este bucle itera sobre los primeros ancho
  bytes del slice mem e imprime cada byte en formato hexadecimal con una ancho de dos caracteres.
- print!(" || "); Esta línea imprime || para separar la sección hexadecimal de la sección ASCII.

- for i in 0..ancho Este bucle itera sobre los primeros ancho bytes del slice mem e imprime cada byte en formato
  ASCII.
  - La función mem.get(i) devuelve una referencia al byte en la ubicación i dentro del arreglo mem. La función
    unwrap_or(&0) se utiliza para desempaquetar el resultado de la función anterior y devuelve el valor del byte
    si existe o devuelve 0 si la referencia es nula.
  - La función is_ascii_graphic() se llama en el valor de byte obtenido y devuelve true si el valor es un carácter
    ASCII gráfico. En caso afirmativo, el valor del byte se convierte en un char utilizando la función to_owned()
    para crear una copia del valor y luego se realiza la conversión explícita al tipo char con la sintaxis as char.
*/
fn muestra_linea_mem(mem: &[u8], ascii: bool, ancho: usize) {
    print!("{:16p} ||", mem.as_ptr());
    for i in 0..ancho {
        print!(" {:02x}", mem.get(i).unwrap_or(&0));
    }

    if ascii {
        print!(" || ");
        for i in 0..ancho {
            let var_char = if mem.get(i).unwrap_or(&0).is_ascii_graphic() {
                mem.get(i).unwrap_or(&0).to_owned() as char
            } else {
                '.'
            };
            print!(" {}", var_char);
        }
    }
    println!();
}

/* Descripción Función:                     
La función muestra_mem acepta tres argumentos: mem, size y ancho. mem es una referencia a un slice de bytes (&[u8])
que contiene la memoria que se desea mostrar. size es el tamaño total de la memoria a mostrar y ancho es el ancho
deseado para mostrar los datos en cada línea.
La función luego calcula el número de líneas necesarias para mostrar toda la memoria utilizando la función
calcula_lineas_mem.
Dentro del ciclo for, la función calcula el desplazamiento en la memoria utilizando el índice de la línea actual y el
ancho deseado. Luego llama a la función muestra_linea_mem para mostrar los datos correspondientes a esa línea.
*/
fn muestra_mem(mem: &[u8], size: usize, ancho: usize) {
    let ascii = false;

    println!("   Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("  -------------- || -----------------------------------------------");
    let lineas = calcula_lineas_mem(size, ancho);
    for i in 0..lineas {
        let offset = i * ancho;
        muestra_linea_mem(&mem[offset..], ascii, ancho);
    }
    println!("  -----------------------------------------------------------------");
}

/* Descripción Función:                     
Esta función toma un argumento de tipo genérico T, que es una referencia compartida (&) a un valor de tipo T. En Rust,
las referencias compartidas son una forma de referenciar un valor sin poseer la propiedad de ese valor. Por lo tanto,
en esta función, var_a es una referencia compartida al valor T que se va a examinar.

La primera línea de la función convierte la referencia compartida var_a en un puntero crudo (*const T) y luego lo
convierte a un puntero crudo a bytes (*const u8). Los punteros crudos en Rust son punteros sin restricciones que
apuntan a una dirección de memoria específica. En este caso, se convierte la referencia compartida var_a en un puntero
crudo para poder examinar la memoria a la que apunta el valor de var_a.

La segunda línea de la función imprime el tamaño en bytes del tipo T utilizando la función mem::size_of::<T>(). Esta
función se utiliza para obtener el tamaño en bytes de un tipo determinado en Rust.

Finalmente, la función llama a otra función llamada muestra_mem con tres argumentos: un slice de bytes
(std::slice::from_raw_parts(var_ptr, mem::size_of::<T>())), que representa la memoria ocupada por el valor de var_a,
el tamaño en bytes de T (mem::size_of::<T>()), y un valor constante llamado COL_POR_DEFECTO. El slice de bytes se crea
a partir del puntero crudo a bytes var_ptr utilizando la función std::slice::from_raw_parts. Esta función crea un slice
a partir de un puntero crudo y una longitud. En este caso, la longitud se establece en el tamaño en bytes de T. La
función muestra_mem se utiliza para imprimir el contenido de la memoria ocupada por el valor de var_a.

Es importante destacar que la función muestra_mem_obj utiliza el modificador unsafe en la llamada a
std::slice::from_raw_parts. Este modificador indica que el código dentro de este bloque es inseguro y puede tener efectos
secundarios no deseados. En este caso, se utiliza el modificador unsafe porque el puntero crudo a bytes var_ptr no tiene
restricciones y puede apuntar a cualquier dirección de memoria. La función std::slice::from_raw_parts también es insegura
porque se utiliza para crear un slice a partir de un puntero crudo sin comprobar si el puntero es válido o no. Por lo tanto,
es importante tener cuidado al utilizar el modificador unsafe en Rust y seguir las buenas prácticas de programación para
evitar errores y vulnerabilidades en el código.
*/
fn muestra_mem_obj<T>(var_a: &T) {
    let var_ptr = var_a as *const T as *const u8;
    println!("--> Tamaño ocupado en bytes ({})", mem::size_of::<T>());
    /*
    La función std::slice::from_raw_parts crea un slice a partir de un puntero a un bloque de memoria.
    El primer argumento es el puntero a la memoria que se utilizará para crear el slice.
    El segundo argumento es el tamaño en bytes del tipo de datos que se almacenará en el slice.
    El puntero a la memoria se proporciona como var_ptr, que se supone que es un puntero a un bloque
    de memoria que contiene datos de tipo T. El segundo argumento, std::mem::size_of::<T>(), es una
    llamada a la función size_of del módulo mem de Rust.
    Esta función devuelve el tamaño en bytes del tipo de datos T.
    */
    muestra_mem(unsafe { std::slice::from_raw_parts(var_ptr, mem::size_of::<T>()) }, mem::size_of::<T>(), COL_POR_DEFECTO);
}

/* Descripción Función:                     

*/
pub fn memoria_1() {
    let titulo = String::from(" Rutinas de salida por pantalla - Formato memoria Hex 1 ");
    imprime_titulo(&titulo);
    
    let mut buffer = [0u8; 256];
    for i in 0..256 {
        buffer[i] = i as u8;
    }
    muestra_mem(&buffer, buffer.len(), 16);

    let mut var_a_array: [i32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

    let var_ptr = &var_a_array as *const [i32; 8] as *const u8;

    println!("  Dirección de inicio del array          ({:?})\n", var_ptr);
    muestra_mem_obj(&var_a_array);

    println!();

    let mut var_a_vector: Vec<i32> = vec![1, 2, 4, 8, 16, 32, 64, 128];

    muestra_mem_obj(&var_a_vector);
    println!("Vector .Begin {:p}", var_a_vector.as_ptr());
     println!("Vector .[0]   {:p}", &var_a_vector[0] as *const i32);
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

//***************************************************************************** 