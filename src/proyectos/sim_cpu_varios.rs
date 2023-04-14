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
// Manejo de bit en un byte con operaciones lógicas y desplazamientos
/*
pub fn get0(){
    // La expresión (1<<0) es una operación de desplazamiento a la izquierda de un bit, que se utiliza para mover
    // el valor binario 1 cero posiciones a la izquierda.
    println!("Creación de un byte activando el bit 0: {:08b}",(1<<0));
    println!("Creación de un byte activando el bit 1: {:08b}",(1<<1));
    println!("Creación de un byte activando el bit 7: {:08b}",(1<<7));
    println!("máscara de bits {:08b}",(0x00 & (1<<1)));
    println!("máscara de bits {:08b}",(0xff & (1<<1)));
    let mut num = 0x00;
    println!("Esta activo el bit 1? con & (1<<1 : {}",(num & (1 << 1)) != 0);
    num = 0xff;
    println!("Esta activo el bit 1? con & (1<<1 : {}",(num & (1 << 1)) != 0);
}

pub fn get(n: u8, b: usize) -> bool {
    (n & (1 << b)) != 0
}
pub fn set(n: u8, b: usize) -> u8 {
    n | (1 << b)
}
pub fn reset(n: u8, b: usize) -> u8 {
    n & !(1 << b)
}

pub fn pruebas_mascara_bits() {
    get0();
    
    let mut n: u8 = 0b00001111;         // n = 15 en binario
    println!("El bit en la posición 3 es {}", get(n, 3));   // imprime true
    n = set(n, 5);                      // establece el bit en la posición 5 en 1
    println!("n ahora es {:08b}", n);   // imprime "n ahora es 0b00101111"
    n = reset(n, 0);                    // resetea el bit en la posición 0 a 0
    println!("n ahora es {:08b}", n);   // imprime "n ahora es 0b00101110"  

    let mut n = 0b1101;             // El número binario 1101 es el número decimal 13
    assert_eq!(get(n, 0), true);        // El bit menos significativo es 1
    assert_eq!(get(n, 1), false);       // El siguiente bit es 0
    n = set(n, 1);
    assert_eq!(get(n, 1), true);        // Ahora el segundo bit es 1
    n = reset(n, 3);
    assert_eq!(get(n, 3), false);       // Ahora el cuarto bit es 0
    assert_eq!(n, 0b0111);              // El número binario 0101 es el número decimal 5
}
*/
