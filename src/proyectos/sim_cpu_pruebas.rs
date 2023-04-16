/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          16-04-2023
    Titulo:         Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::{self}};
//use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::*};
use colored::*;

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.red());
}

//***************************************************************************** 

/* 
// Struct para representar los registros Z80
pub struct RegistrosZ80 {
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    pub a: u8,
    //pub f: sim_cpu_registros::Flags,
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
            //f: Default::default(),
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
    // Inicializar la estructura de registros y la memoria del sistema
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
 */

pub fn cpu_sim_0() {
    let titulo = String::from(" Simulación CPU - Pruebas de Funciones, Métodos ");
    imprime_titulo(&titulo);

/* 
    let resultado = ejecutar_programa();
    println!("Valor de registro A después de ejecutar el programa: {:02x}", resultado);
*/

    // Inicializar la estructura de registros, flags y memoria
    let mut cpu_reg = sim_cpu_registros::RegistrosCPU::new();
    //let mut cpu_reg = sim_cpu_registros::RegistrosCPU::new();
    let mut cpu_flags = sim_cpu_registros::Flags::new_flags();
    // Crea un banco de memoria por defecto de 16384 bytes (16Kb)
    let mut memoria = BancosMemoria::new();


    //pru_registros(&mut cpu_reg);                    // Prueba manejo registros
    //pru_flags(&mut cpu_reg, &mut cpu_flags);        // Prueba manejo bit de flags
    //pru_flags_cal(&mut cpu_reg, &mut cpu_flags);    // Pruebas cálculos de flags individuales
    //pru_flags_alu_0(&mut cpu_reg, &mut cpu_flags);  // Pruebas cálculos de flags - ALU
    //pru_mem_0(&mut cpu_reg, &mut memoria);          // Manejo bancos de memoria
    //pru_varias_0(&mut cpu_reg);                     // "fn muestra_mem" y manejo de LittleEndian y BigEndian

}

//***************************************************************************** 


//*************************************  Prueba manejo registros
fn pru_registros(cpu_reg: &mut sim_cpu_registros::RegistrosCPU){
    let titulo = String::from(" CPU - Simulación CPU - Prueba manejo registros ");
    imprime_titulo(&titulo);
   
    // Registro A al inicializar
    println!("Registro A: 0x{:02x}", cpu_reg.get_a());
    // Registro A modificado
    cpu_reg.set_a(0x12);
    println!("Registro A: 0x{:02x}", cpu_reg.get_a());

    // Registro BC al inicializar
    println!("Registro BC: 0x{:04x}", cpu_reg.get_bc());
    cpu_reg.set_b(0xff);
    cpu_reg.set_c(0b11111111);
    // Registro BC modificados por separados
    println!("Registro BC: 0x{:04x}", cpu_reg.get_bc());
    // Registro BC modificados como uno solo
    cpu_reg.set_reg_bc(0b0000111111110000);
    println!("Registro BC: 0x{:04x}", cpu_reg.get_reg_bc());
}

//************************************* Prueba manejo bit de flags
fn pru_flags(cpu_reg: &mut sim_cpu_registros::RegistrosCPU, cpu_flags: &mut sim_cpu_registros::Flags){
    let titulo = String::from(" Simulación CPU - Prueba manejo bit de flags ");
    imprime_titulo(&titulo);

    cpu_flags.set_flags(0b10110101);

    // Nota: bit 3 y 5 no se utilizan
    // Actualizar los flags con un valor de 0b11111111 (true)
    cpu_flags.set_flags(0b11111111);

    // Obtener el valor de los flags
    let flags_value = cpu_flags.get_flags();
    println!("Valor de los flags: 0b{:08b}", flags_value);

    // Establecer el bit de signo a 0 (false)
    println!("Valor del flags sign: {}", cpu_flags.get_bit(7));
    cpu_flags.set_bit(7, false);
    println!("Valor del flags sign: {}", cpu_flags.get_bit(7));
    println!("Valor de los flags: 0b{:08b}", cpu_flags.get_flags());

    cpu_flags.set_flags(0b00000000);
    println!("Valor de los flags: 0b{:08b}", cpu_flags.get_flags_1());

    //************************************* Integración estructura "flags" y estructura "reg/flags"
    cpu_flags.set_bit(0,true );
    println!("cpu_flags.get_bit(0): {}, cpu_reg.flags.get_bit(0): {}", cpu_flags.get_bit(0), cpu_reg.flags.get_bit(0));
    println!();
}

//************************************* Pruebas cálculos de flags individuales
fn pru_flags_cal(cpu_reg: &mut sim_cpu_registros::RegistrosCPU, cpu_flags: &mut sim_cpu_registros::Flags) {
    let titulo = String::from(" Simulación CPU - Prueba cálculos de flags individuales ");
    imprime_titulo(&titulo);

    cpu_reg.set_a(0xfe);
    cpu_reg.set_b(0x02);

    // Ejecutar la instrucción "ADD A, B" y calcula Flags
    println!("Contenido inicial:
    Registro A: 0x{:02X} {:08b}
    Registro B: 0x{:02X} {:08b}
    Carry     : {}, {}"
    , cpu_reg.get_a(), cpu_reg.get_a()
    , cpu_reg.get_b(), cpu_reg.get_b()
    , cpu_flags.get_bit(0), cpu_flags.get_bit_1(0));
    let resultado = cpu_flags.flags_acarreo_add(cpu_reg.get_a(), cpu_reg.get_b());
    // cpu_reg.set_a(resultado);
    println!("Contenido posterior:
    Registro A: 0x{:02X} {:08b}
    Carry     : {}, {}"
    , resultado, resultado
    , cpu_flags.get_bit(0), cpu_flags.get_bit_1(0));
    cpu_flags.flags_paridad(resultado);
    println!("    Paridad   : {}, {}", cpu_flags.get_bit(2), cpu_flags.get_bit_1(2));
    cpu_flags.flags_acarreo_auxiliar_add(cpu_reg.get_a(), cpu_reg.get_b());
    println!("    Half-Carry: {}, {}", cpu_flags.get_bit(4), cpu_flags.get_bit_1(4));
    cpu_flags.flags_cero(resultado);
    println!("    Cero      : {}, {}", cpu_flags.get_bit(6), cpu_flags.get_bit_1(6));
    cpu_flags.flags_signo(resultado);
    println!("    Signo     : {}, {}", cpu_flags.get_bit(7), cpu_flags.get_bit_1(7));
    cpu_reg.set_a(resultado);

}

//************************************* Pruebas cálculos de flags - ALU
fn pru_flags_alu_0(cpu_reg: &mut sim_cpu_registros::RegistrosCPU, cpu_flags: &mut sim_cpu_registros::Flags){
    let titulo = String::from(" Simulación CPU - Cálculos de flags - ALU (ADD, ADC) ");
    imprime_titulo(&titulo);

    let test = true;
    let contempla_cf = true;

    cpu_reg.set_a(0xfe);
    cpu_reg.set_b(0x02);
    //cpu_flags.set_bit(0, false);
    println!("Contenido inicial:
    Registro A: 0x{:02X} {:08b}
    Registro B: 0x{:02X} {:08b}
    Carry     : {}, {}"
    , cpu_reg.get_a(), cpu_reg.get_a()
    , cpu_reg.get_b(), cpu_reg.get_b()
    , cpu_flags.get_bit(0), cpu_flags.get_bit_1(0));
    let resultado_add = cpu_flags.add(cpu_reg.get_a(), cpu_reg.get_b(), contempla_cf, test);
    cpu_reg.set_a(resultado_add);
    println!("Resultado ADD: 0x{:02x}, {:08b}", cpu_reg.get_a(), cpu_reg.get_a());
    println!("Banderas: S={} Z={} HC={} P={} C={}"
            , cpu_flags.get_bit(7)
            , cpu_flags.get_bit(6)
            , cpu_flags.get_bit(4)
            , cpu_flags.get_bit(2)
            , cpu_flags.get_bit(0),);
    println!();

    cpu_reg.set_a(0xfe);
    cpu_reg.set_b(0x02);
    println!("Contenido inicial:
    Registro A: 0x{:02X} {:08b}
    Registro B: 0x{:02X} {:08b}
    Carry     : {}, {}"
    , cpu_reg.get_a(), cpu_reg.get_a()
    , cpu_reg.get_b(), cpu_reg.get_b()
    , cpu_flags.get_bit(0), cpu_flags.get_bit_1(0));
    let resultado_adc = cpu_flags.adc(cpu_reg.get_a(), cpu_reg.get_b(), test);
    cpu_reg.set_a(resultado_adc);
    println!("Resultado ADC: 0x{:02x}, {:08b}", cpu_reg.get_a(), cpu_reg.get_a());
    println!("Resultado ADC: 0x{:02x}, {:08b}", resultado_adc, resultado_adc);
    println!("Banderas: S={} Z={} HC={} P={} C={}"
            , cpu_flags.get_bit(7)
            , cpu_flags.get_bit(6)
            , cpu_flags.get_bit(4)
            , cpu_flags.get_bit(2)
            , cpu_flags.get_bit(0),);
}

//************************************* Manejo bancos de memoria
fn pru_mem_0(cpu_reg: &mut sim_cpu_registros::RegistrosCPU, memoria: &mut BancosMemoria) {
    let titulo = String::from(" Simulación CPU - Pruebas manejo bancos de memoria ");
    imprime_titulo(&titulo);
    
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
    cpu_reg.set_b(0xff);
    cpu_reg.set_c(0x00);
    memoria.escribir_memoria(0x2000, cpu_reg.get_b());
    // lee el byte en la dirección 0x2000 del primer banco
    let byte_1 = memoria.leer_memoria(0x2000);
    println!("Byte leído en la dirección 8192 (0x2000) del primer banco: 0x{:02x}", byte_1);

    println!(" {:?} ", memoria.segmento_memoria.len());
    let mut resultado = memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);
    println!(" {:?} ", memoria.segmento_memoria.len());
    resultado = memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);

    resultado = memoria.eliminar_segmento(0);
    println!(" {:?}", resultado);
}

//************************************* Pruebas de "fn muestra_mem" y manejo de LittleEndian y BigEndian
/* Little-endian - Big-endian               
u16::from_le_bytes y u16::from_be_bytes son dos métodos en el tipo u16 de Rust que te permiten
convertir un arreglo de bytes en un valor u16. La diferencia entre ellos está en el orden de
los bytes utilizado para interpretar el arreglo.

u16::from_le_bytes interpreta el arreglo en orden de bytes little-endian, lo que significa que
el byte menos significativo está primero en el arreglo y el byte más significativo está último.
Este es el orden de bytes utilizado por la mayoría de las computadoras basadas en Intel.

u16::from_be_bytes interpreta el arreglo en orden de bytes big-endian, lo que significa que el
byte más significativo está primero en el arreglo y el byte menos significativo está último. Este
es el orden de bytes utilizado por la mayoría de los protocolos de red y algunas otras arquitecturas
de computadora.

Por ejemplo, si tienes un arreglo de bytes [0x78, 0x56], u16::from_le_bytes lo interpretará como el
valor u16 0x5678, mientras que u16::from_be_bytes lo interpretará como el valor u16 0x7856.

little endian 0xABCD =
Dirección de memoria:  | 0x0000 | 0x0001 |
                       +--------+--------+
     Contenido (hex):  |  0xCD  |  0xAB  |
                       +--------+--------+
*/
fn pru_varias_0(cpu_reg: &mut sim_cpu_registros::RegistrosCPU) {
    let titulo = String::from(" Pruebas de \"fn muestra_mem\" y manejo de LittleEndian y BigEndian ");
    imprime_titulo(&titulo);

    cpu_reg.set_b(0xff);
    cpu_reg.set_c(0x00);
    let bytes: [u8; 2] = [cpu_reg.get_b(), cpu_reg.get_c()];
    let bytes_le = u16::from_le_bytes([cpu_reg.get_b(), cpu_reg.get_c()]);
    println!("Byte: {:02x}{:02x}, Byte Little: {:04x}", bytes[0], bytes[1], bytes_le);
    println!();

    let mut vec: [u8; 512] = [0;512];
    for i in 0..vec.len() { vec[i] = (i+0) as u8; }
    for i in 16..49 { vec[i] = 00 as u8; }

    println!("Byte: {:02x}{:02x}, Byte Little: {:04x}", bytes[0], bytes[1], bytes_le);
    vec[20] = bytes[0];
    vec[21] = bytes[1];
    vec[22] = ((bytes_le >> 8) & 0xFF) as u8;   // 8 bits + significativos en la posición 22 del vector
    vec[23] = (bytes_le & 0xFF) as u8;          // 8 bits - significativos en la posición 23 del vector
    vec[51] = 0xFF;
    vec[52] = 0xFF;

    muestra_mem(&vec);

    //********************************* Más LittleEndian con "swap_bytes()"
    // https://doc.rust-lang.org/std/primitive.u16.html#method.swap_bytes
    println!();
    let direccion = u16::from_be_bytes([cpu_reg.get_b(), cpu_reg.get_c()]);
    let direccion_le = direccion.swap_bytes();
    println!("Dirección: {:04x}, Dirección_le: {:04x}", direccion, direccion_le);

}

//***************************************************************************** fn nuestra_mem
fn muestra_mem(vec: &[u8]) {
    // let lineas = vec.len() / 16 + if vec.len() % 16 != 0 { 1 } else { 0 };
    println!(" Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("-------------- || ------------------------------------------------");
    
    let mut found_pair = false;
    //let mut direccion: u16 = 0;
    let mut ultimo_byte: u8 = 0;
    let mut buffer = String::with_capacity(16);
    //let mut buffer = String::new();
    for group in vec.chunks(16) {
        if found_pair { break; }
        
        buffer.clear();
 
/*
        for i in 0..group.len() {
            let byte = group[i];
            
            if i == 0 && byte == 0xff && ultimo_byte == 0xff {
                found_pair = true;
                break;
            }
            
            if i == (group.len() - 1)  && group[i] == 0xff { ultimo_byte = byte; }

            if byte == 0xff && i < group.len() - 1 && group[i + 1] == 0xFF {
                found_pair = true;
                break;
            }
            buffer.push_str(&format!("{:02X} ", byte));
        }
        //println!("       {:04x}    || {}", dir, &buffer.trim_end());
        //direccion += 0x10;
*/

        for (i, byte) in group.iter().enumerate() {
            if i == 0 && *byte == 0xFF && ultimo_byte == 0xFF {
                found_pair = true;
                break;
            }
        
            if i == group.len() - 1 && *byte == 0xFF {
                ultimo_byte = *byte;
            }

            if *byte == 0xFF && i < group.len() - 1 && group[i + 1] == 0xFF {
                found_pair = true;
                break;
            }
            if i == 0 {buffer.push_str(&format!("{:p} || ", byte));}
        
            buffer.push_str(&format!("{:02X} ", byte));
        }

        println!("{}", &buffer.trim_end());
    }
}

//***************************************************************************** 