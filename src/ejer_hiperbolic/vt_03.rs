/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          06-02-2023
    Canal youtube:  Hyperbolic Time Academy
                    https://www.youtube.com/@hyperbolictimeacademy

    Lista Repro:    Rust programming - beginners
                    https://www.youtube.com/playlist?list=PLPCT_BwWXlAPD-NA8SCn_nhMTLVxw99V4

    Video tutorial: Create a Quiz Game in Rust, Part 1 - Rust Programming Videos For Beginners - Lesson 03
                    https://www.youtube.com/watch?v=Pv_hle4p1j4&list=PLPCT_BwWXlAPD-NA8SCn_nhMTLVxw99V4&index=2

***************************************************************************************/

use std::io;
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;


#[allow(dead_code)]
#[allow(unused_variables)]
pub fn vt_03() {
    // Titulo, autor, ISBN10, idioma(bool)
    let libro = vec![
        ("MCS-80", "Intel", 0000000000, false),
        ("The Z80 Microprocessor Architecture, Interfacing, Programming and Design", "Ramesh Gaonkar", 0023404841_u64, false),
        ("Microcomputers And Microprocessors: The 8080, 8085 and Z-80 Programming, Interfacing and Troubleshooting", "John E. Uffenbeck", 0135840619, false),
        ("MICRO PROCESADADORES Arquitectura Programación y Desarrollo De Sistemas", "Jose Mª Angulo Usategui", 8428312370, true),
        ("MICRO PROCESADADORES Fundamentos Diseño y Aplicaciones", "Jose Mª Angulo Usategui", 842831148, true),
        ("Microprocesador Z-80, programación e interfaces (Libro2)", "Peter R. Rony,Elizabeth A. Nichols,Joseph C. Nichols", 9684100124, true),
        ("Programación del Microprocesador Z80 (Libro1)", "Peter R. Rony,Elizabeth A. Nichols,Joseph C. Nichols", 8426704085, true),
        ("Microprocesadores: Del chip al sistema", "Rodnay Zaks", 8426703909, true),
        ];    

    entrada_text_teclado();

// solución con loop
    let mut contador:usize = 0;
    loop {
        println!(" ID: {}, {:?}", contador+1, libro[contador].0);
        contador +=1;
        if contador == libro.len() { break; }
    }

// solución con while
/* 
    contador = 0;
    while true {
        println!(" ID: {}, Titulo: {:?}", contador, libro[contador].0);
        println!(" ID: {}, Titulo: {:?}", contador, libro[id as usize]);
        contador +=1;
        if contador == libro.len() {
            break;
        }        
    }
*/

    let mut id = intro();
    if id == 0 {
        println!("La entrada es nula");
    } else {
        id -= 1;
        println!("Titulo: {:?}", libro[id as usize].0);
        println!("Autor : {:?}", libro[id as usize].1);
        println!("ISBN  : {:?}", libro[id as usize].2);
        println!("Idioma: {:?}", libro[id as usize].3);
        // println!(" ID: {}, Titulo: {:?}", contador, libro[id as usize]);
    }

//*************************************
    let titulo = " Volcado a archivo ";
    // sin variable, relleno con -, centrado, longitud de 30 caracteres
    println!("{:*^80}", titulo);

    //*********************************
    // Crea un archivo y escribe los datos en él
    let mut file = File::create("libros.txt").unwrap();
    file.write_all(format!("{:?}\n", libro).as_bytes()).unwrap();
/*    // Equivalente a la linea superior
    let libro_string = format!("{:?}", libro);
    file.write_all(libro_string.as_bytes()).unwrap();
*/

}

/*
La función read_line del módulo io::stdin se utiliza para leer la entrada del teclado
y almacenarla en la variable input. Luego, el método trim se utiliza para eliminar los
espacios en blanco al principio y al final de la cadena y el método is_empty se utiliza
para comprobar si la cadena es nula o no.
*/
fn entrada_text_teclado() {
    let mut input = String::new();
    println!("Escribe un texto: ");
    match io::stdin().read_line(&mut input) {
        Ok(nro_bytes) => {
            if input.trim().is_empty() {
                println!("La entrada es vacía\n");
            } else {
                println!("Número de Bytes del texto: {nro_bytes}, {input}");
            }
        },
        Err(error) => println!("Error al leer la entrada: {}", error),
    }
}

// Solución UNO aplicada
fn intro() -> u32 {
    println!("\nSelecciona ID del libro para ver detalles ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    if input.trim().is_empty() {
        // println!("La entrada es nula");
        return 0;
    } else {
        let id: u32 = u32::from_str(&input.trim()).unwrap();    // use std::str::FromStr;
        id
    }
}
/* Entrada de texto sin control de vacía 
use std::io;

fn entrada_text() {
    let mut input: String = String::new();
    println!("Escribe un texto: ");
    let result = io::stdin().read_line(&mut input);
    match result {
        Ok(nro_bytes) =>  println!("Número de Bytes del texto: {nro_bytes}, {input}"),
        Err(error) => println!("Error {}", error),
    }
}
*/
/* Solución UNO a entrada vacía 
use std::io;

fn main() {
    let mut input = String::new();

    println!("Ingresa una cadena: ");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    if input.trim().is_empty() {
        println!("La entrada es nula");
    } else {
        println!("La entrada no es nula");
    }
}
*/
/* Solución DOS a entrada vacía 
use std::io;

fn main() {
    let mut input = String::new();
    println!("Escribe un texto: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().is_empty() {
                println!("La entrada es nula");
            } else {
                println!("La entrada es: {}", input.trim());
            }
        },
        Err(error) => println!("Error al leer la entrada: {}", error),
    }
}
*/
/* Solución TRES a entrada vacía 
use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) if n > 0 => println!("Has introducido: {}", input),
        Ok(_) => println!("No has introducido nada"),
        Err(error) => println!("Error al leer la entrada: {}", error),
    }
}
*/
