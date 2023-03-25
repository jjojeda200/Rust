/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          02-03-2023
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
Se define una constante COL_POR_DEFECTO con valor 16. Esta constante se utiliza
como ancho por defecto para mostrar el contenido de la memoria.
*/
const COL_POR_DEFECTO: usize = 16;

/* Notas: fn calcula_lineas 
La función calcula_lineas recibe dos argumentos: size y ancho, ambos de tipo usize.
Esta función calcula la cantidad de líneas necesarias para mostrar el contenido de
la memoria. Para hacer esto, divide el tamaño total (size) por el ancho de línea
(ancho) y luego verifica si el tamaño total no es un múltiplo del ancho. Si no lo
es, agrega una línea adicional. La función devuelve la cantidad total de líneas.
*/
fn calcula_lineas(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        return lineas + 1;
    }
    lineas
}

/* Notas: muestra_linea_mem 
La función muestra_linea_mem recibe dos argumentos: mem que es una referencia a un
arreglo de bytes (&[u8]) y ancho que es el ancho de la línea. Esta función imprime
una línea de contenido de memoria, que consta de tres partes separadas por ||. La
primera parte muestra la dirección de memoria en la que comienza la línea, la segunda
parte muestra los bytes que se encuentran en esa línea y la tercera parte muestra una
representación en caracteres de los bytes de la línea.
Para imprimir las direcciones de memoria, la función utiliza el puntero base de la
referencia mem (mem.as_ptr()) y lo formatea con {:16p}. Luego, utiliza un ciclo for
para imprimir los bytes en la línea.
*/
fn muestra_linea_mem(mem: &[u8], ancho: usize) {
    print!("{:16p} ||", mem.as_ptr());
    for i in 0..ancho {
        print!(" {:02x}", mem[i]);
    }
    // print!(" || ");
    // for i in 0..ancho {
    //     let var_char = if mem[i] > 32 && mem[i] < 128 { mem[i] } else { b'.' };
    //     print!(" {}", var_char as char);
    // }
    println!();
}

/* Notas: muestra_mem       
La función muestra_mem recibe tres argumentos: mem que es una referencia a un arreglo
de bytes (&[u8]), size que es el tamaño total de la memoria y ancho que es el ancho de
la línea. Esta función imprime todo el contenido de la memoria, dividiéndolo en líneas
utilizando la función muestra_linea_mem. Primero, imprime una cabecera con las etiquetas
de las columnas. Luego, calcula la cantidad de líneas necesarias utilizando la función
calcula_lineas y utiliza un ciclo for para imprimir cada línea. Para hacer esto, utiliza
una variable offset que se incrementa en ancho en cada iteración, para ir avanzando en
la memoria.
*/
fn muestra_mem(mem: &[u8], size: usize, ancho: usize) {
    println!("   Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("  -------------- || -----------------------------------------------");
    let lineas = calcula_lineas(size, ancho);
    let mut offset = 0;
    for _ in 0..lineas {
        muestra_linea_mem(&mem[offset..], ancho);
        offset += ancho;
    }
    println!("  -----------------------------------------------------------------");
}

/* Notas: muestra_mem_obj   
La función muestra_mem_obj recibe un argumento genérico var_a y se utiliza para mostrar
el contenido de memoria de cualquier objeto de Rust. La función utiliza std::mem::size
of para obtener el tamaño del objeto y std::slice::from_raw_parts para crear una referencia
a la memoria del objeto. Luego, llama a la función muestra_mem para imprimir el contenido
de la memoria.
*/
pub fn muestra_mem_obj<T>(var_a: T) {
    let var_ptr = &var_a as *const T as *const u8;
    println!("--> Tamaño ocupado en bytes ({})", std::mem::size_of::<T>());
    muestra_mem(unsafe { std::slice::from_raw_parts(var_ptr, std::mem::size_of::<T>()) }, std::mem::size_of::<T>(), COL_POR_DEFECTO);
}

/* Notas: memoria           
Finalmente, la función memoria define dos variables: var_a_array que es un arreglo estático
de 8 elementos y var_a_vector que es un vector dinámico inicializado con los mismos valores
que var_a_array. Luego, utiliza la función muestra_mem_obj para mostrar el contenido de la
memoria de ambas variables
*/    
#[allow(unused)]
pub fn memoria() {
    let titulo = String::from(" Imprime zonas de memoria ");
    imprime_titulo(&titulo);

    println!("  Zona de memoria del Array");
    let var_a_array: [i32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    muestra_mem_obj(var_a_array);

    let mut var_a_vector: Vec<i32> = vec![1, 2, 4, 8, 16, 32, 64, 128];
    //muestra_mem_obj(var_a_vector.clone());
    println!("Vector .begin {:p}", var_a_vector.as_ptr());
    // println!("Vector .[0]   {:p}", &var_a_vector[0]);
    println!("Vector .[7]   {:p}", &var_a_vector[7]);
    unsafe {println!("Vector .end   {:p}", var_a_vector.as_ptr().add(var_a_vector.len()));};
    println!("varAvector contiene {} elementos.", var_a_vector.len());
    println!("capacidad final={}", var_a_vector.capacity());
    var_a_vector.push(256);

}