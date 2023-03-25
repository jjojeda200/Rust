/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          13-03-2023
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
#![allow(unused_assignments)]

use crate::proyectos::{sim_cpu_registros::{self}, sim_cpu_memoria::*};

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** 
// Struct para representar los registros Z80
pub struct RegistrosZ80 {
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    pub a: u8,
    pub f: sim_cpu_registros::Flags,
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

pub fn z80_sim_1() {
    let titulo = String::from(" Z80 - Simulación CPU - Aproximación de pruebas 1");
    imprime_titulo(&titulo);

    let resultado = ejecutar_programa();
    println!("Valor de registro A después de ejecutar el programa: {:02x}", resultado);

//*************************************



//************************************* Prueba manejo registros
    println!("");
    // Inicializar la estructura de registros y de flags
    let mut z80_reg = sim_cpu_registros::RegistrosCPU::new();
    let mut z80_flags = sim_cpu_registros::Flags::new_flags();
   
    // Registro A al inicializar
    println!("Registro A: 0x{:02x}", z80_reg.get_a());
    // Registro A modificado
    z80_reg.set_a(0x12);
    println!("Registro A: 0x{:02x}", z80_reg.get_a());


    // Registro BC al inicializar
    println!("Registro BC: 0x{:04x}", z80_reg.get_bc());
    z80_reg.set_b(0xff);
    z80_reg.set_c(0b11111111);
    // Registro BC modificados por separados
    println!("Registro BC: 0x{:04x}", z80_reg.get_bc());
    // Registro BC modificados como uno solo
    z80_reg.set_reg_bc(0b0000111111110000);
    println!("Registro BC: 0x{:04x}", z80_reg.get_reg_bc());

//************************************* Prueba manejo flags
    z80_flags.set_flags(0b10110101);

    // Nota: bit 3 y 5 no se utilizan
    // Actualizar los flags con un valor de 0b11111111 (true)
    z80_flags.set_flags(0b11111111);

    // Obtener el valor de los flags
    let flags_value = z80_flags.get_flags();
    println!("Valor de los flags: 0b{:08b}", flags_value);

    // Establecer el bit de signo a 0 (false)
    z80_flags.set_bit(7, false);
    println!("Valor de los flags: 0b{:08b}", z80_flags.get_flags());
    z80_flags.set_flags(0b00000000);
    println!("Valor de los flags: 0b{:08b}", z80_flags.get_flags_1());

//************************************* Prueba manejo memoria
    println!("");
    //memoria_0();

/*
*/
    // Crea un banco de memoria por defecto de 16384 bytes (16Kb)
    let mut memoria = BancosMemoria::new();
    
    // Confirma el banco activo (Banco índice 0)
    let mut num_banco_actual = memoria.get_banco_activo() as usize;
    // Impresión de verificación

    println!("Banco de memoria Nº: {}, Tamaño del banco: {}, Capacidad del banco de memoria: {} ",
        memoria.banco_actual,
        memoria.segmento_memoria[num_banco_actual].len(),
        memoria.segmento_memoria[num_banco_actual].capacity());

    // Crea un banco de memoria adicional de 32768 bytes (32Kb)
    memoria.crear_segmento(32768);

    // Selecciona el nuevo banco (Banco índice 1)
    memoria.set_banco_activo(1);
    num_banco_actual = memoria.get_banco_activo() as usize;

    // Impresión de verificación
    println!("Banco de memoria Nº: {}, Tamaño del banco: {}, Dirección de memoria (ptr): {:p} ",
        memoria.banco_actual,
        memoria.segmento_memoria[num_banco_actual].len(),
        memoria.segmento_memoria[num_banco_actual].as_ptr());

    // selecciona el primer banco (Banco índice 0)
    memoria.set_banco_activo(0);
    num_banco_actual = memoria.get_banco_activo() as usize;



    // escribe un byte en la dirección 0x2000 del primer banco
    memoria.escribir_memoria(0x2000, 0x55);
    // lee el byte en la dirección 0x2000 del primer banco
    let byte1 = memoria.leer_memoria(10);
    println!("Byte leído en la dirección 0x2000 del primer banco: 0x{:02x}", byte1);


    println!(" {:?} ", memoria.segmento_memoria.len());
    let mut resultado = memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);
    println!(" {:?} ", memoria.segmento_memoria.len());
    resultado = memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);

    resultado = memoria.eliminar_segmento(0);
    println!(" {:?}", resultado);

    
/* 
    // crea una memoria con dos bancos: uno de 16K y otro de 32K
    let mut memory = BankedMemory::new(2, 16 * 1024 + 48 * 1024);
    memory.select_bank(0); // selecciona el primer banco (índice 0)

    // escribe un byte en la dirección 0x2000 del primer banco
    memory.write_byte(0x2000, 0x55);

    // selecciona el primer banco nuevamente
    memory.select_bank(0);


    let byte1 = memory.read_byte(0x2000);
    println!("Byte leído en la dirección 0x2000 del primer banco: {}", byte1);

    // selecciona el segundo banco (índice 1)
    memory.select_bank(1);

    // escribe un byte en la dirección 0x8000 del segundo banco
    memory.write_byte(0x8000, 0xAA);

    // selecciona el segundo banco nuevamente
    memory.select_bank(1);

    // lee el byte en la dirección 0x8000 del segundo banco
    let byte2 = memory.read_byte(0x8000);
    println!("Byte leído en la dirección 0x8000 del segundo banco: {}", byte2);

    memory.select_bank(0); // selecciona el primer banco (índice 0)
    for bank_num in 0..memory.banks.len() {
        memory.select_bank(bank_num);
        println!("Banco de memoria Nº: {}, Tamaño de memoria del banco: {}", bank_num, memory.banks[bank_num].len());
        println!("Capacidad del banco de memoria: {} ", memory.banks[bank_num].capacity());

        for addr in 0..memory.banks[bank_num].len() {
            let val = memory.read_byte((bank_num as u16) * 0x4000 + (addr as u16));
            println!("Dirección 0x{:04X}: {}", addr, val);
        }
    }
*/

}

