/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          10-03-2023
    Titulo:         Simulación CPU
    Descripción:    
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

use crate::proyectos::sim_cpu_struct::{self};



// Struct para representar los registros Z80
pub struct RegistrosZ80 {
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    pub a: u8,
    pub f: sim_cpu_struct::Flags,
}

impl Default for RegistrosZ80 {
    fn default() -> Self {
        RegistrosZ80 {
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            a: 0,
            f: Default::default(),
        }
    }
}

// Struct para representar la memoria del sistema
pub struct Memoria {
    datos: [u8; 65536],
}

// Función para leer un byte de la memoria
pub fn leer_memoria(memoria: &Memoria, direccion: u16) -> u8 {
    memoria.datos[direccion as usize]
}

// Función para escribir un byte en la memoria
pub fn escribir_memoria(memoria: &mut Memoria, direccion: u16, dato: u8) {
    memoria.datos[direccion as usize] = dato;
}

// Función para ejecutar una instrucción del procesador
pub fn ejecutar_instruccion(registros: &mut RegistrosZ80, opcode: u8, memoria: &mut Memoria) -> u32 {
    // Implementar la lógica para decodificar y ejecutar cada instrucción
    // Utilizar la función leer_memoria para leer operandos de la memoria
    // Utilizar la función escribir_memoria para escribir resultados en la memoria
    // Actualizar los registros según sea necesario
    // Devolver el número de ciclos que consume la instrucción

    match opcode {
        0x00 => {
            registros.pc += 1;
            4
        }
        // LD A, n
        0x3E => {
            let n = leer_memoria(memoria, registros.pc + 1);
            registros.a = n;
            registros.pc += 2;
            8
        }
        // HALT
        0x76 => {
            registros.pc += 1;
            4
        }
        _ => unimplemented!("opcode {:02X} not implemented", opcode),
        //_ =>  0,
    }
}

// Función para actualizar los registros Z80 después de ejecutar una instrucción
pub fn actualizar_registros(registros: &mut RegistrosZ80) {
    // Implementar la lógica para actualizar los registros según sea necesario
    
}

// Función para actualizar los registros Z80 a lo largo de varios ciclos de reloj
pub fn actualizar_registros_z80(registros: &mut RegistrosZ80, ciclos_restantes: &mut u32, memoria: &mut Memoria) {
    while *ciclos_restantes > 0 {
        // Ejecutar la siguiente instrucción y restablecer el contador
        let opcode = leer_memoria(memoria, registros.pc);
        *ciclos_restantes -= ejecutar_instruccion(registros, opcode, memoria);

        // Actualizar los registros de acuerdo a los efectos de la instrucción
        actualizar_registros(registros);
    }
}

fn ejecutar_programa() -> u8 {
    // Inicializar la estructura de registros Z80 y la memoria del sistema
    let mut registros = RegistrosZ80::default();
    registros.pc = 0x100;
    let mut memoria = Memoria { datos: [0; 65536] };

    // Escribir un programa en la memoria
    escribir_memoria(&mut memoria, 0x100, 0x3E); // LD A, 0x42
    escribir_memoria(&mut memoria, 0x101, 0x42);
    escribir_memoria(&mut memoria, 0x102, 0x76); // HALT

    // Ejecutar el programa a una velocidad de 4 MHz
    let mut ciclos_restantes = 4 * 60000;
    actualizar_registros_z80(&mut registros, &mut ciclos_restantes, &mut memoria);

    // Devolver el valor del registro A después de ejecutar el programa
    registros.a
}

pub fn mn() {
    let resultado = ejecutar_programa();
    println!("Valor de registro A después de ejecutar el programa: {:02x}", resultado);

//*************************************
    // Para crear un struct Flags a partir de un byte:
    let flags_byte: u8 = 0b00101101;                // ejemplo
    let flags = sim_cpu_struct::Flags {
        carry: (flags_byte & 0b00000001) != 0,
        subtract: (flags_byte & 0b00000010) != 0,
        parity_overflow: (flags_byte & 0b00000100) != 0,
        half_carry: (flags_byte & 0b00010000) != 0,
        zero: (flags_byte & 0b01000000) != 0,
        sign: (flags_byte & 0b10000000) != 0,
    };
    println!("Flags: {:08b}", flags_byte);
    println!("Flags: {:02x}", flags_byte);

    // Para crear un byte a partir de los flags:
    let flags_byte = 
        (flags.carry as u8) |
        ((flags.subtract as u8) << 1) |
        ((flags.parity_overflow as u8) << 2) |
        ((flags.half_carry as u8) << 4) |
        ((flags.zero as u8) << 6) |
        ((flags.sign as u8) << 7);

    println!("Flags: {:08b}", flags_byte);

    let substract_bit_mask: u8 = 0b00000010;    // mascara para el bit de substract

    // Verificar el estado actual del bit de substract
    let substract_bit_is_set = flags_byte & substract_bit_mask != 0;

    // Modificar el bit de substract
    let new_flags_byte = if substract_bit_is_set {
        flags_byte & !substract_bit_mask   // borrar el bit de substract
    } else {
        flags_byte | substract_bit_mask    // establecer el bit de substract
    };
    println!("Flags: {:08b}", flags_byte);
    println!("Flags: {:02x}", flags_byte);

//************************************* Prueba manejo registros
    // Inicializar la estructura de registros
    let mut z80_reg = sim_cpu_struct::Z80Reg::new();

    println!("Registro A: 0x{:02x}", z80_reg.get_a());
    println!("Registro F: 0x{:02x}", z80_reg.get_f());
    
    // Asignar un valor al registro
    z80_reg.set_a(0x12);
    println!("Registro A: 0x{:02x}", z80_reg.get_a());

    z80_reg.set_f(0b00101101);
    println!("Registro F: 0x{:02x}", z80_reg.get_f());


    z80_reg.set_b(0xff);
    z80_reg.set_c(0b11111111);
    println!("Registro BC: 0x{:04x}", z80_reg.get_bc());

}

