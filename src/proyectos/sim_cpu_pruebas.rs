/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-04-2023
    Titulo:         Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
/* Detalles                                 

*/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
//#![allow(unused_imports)]       // Por el "use super::*;" de los test
//#![allow(unused_mut)]           // Por el uso de los test

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::{self}};

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


pub fn cpu_sim_0() {
    let titulo = String::from(" CPU - Simulación CPU - Aproximación de pruebas 1 ");
    imprime_titulo(&titulo);

    let resultado = ejecutar_programa();
    println!("Valor de registro A después de ejecutar el programa: {:02x}", resultado);



//************************************* Prueba manejo registros
    let titulo = String::from(" CPU - Simulación CPU - Prueba manejo registros ");
    imprime_titulo(&titulo);

    // Inicializar la estructura de registros y de flags
    let mut cpu_reg = sim_cpu_registros::RegistrosCPU::new();
    //let mut cpu_reg = sim_cpu_registros::RegistrosCPU::new();
    let mut cpu_flags = sim_cpu_registros::Flags::new_flags();
   
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

//************************************* Prueba manejo memoria
    let titulo = String::from(" CPU - Simulación CPU - Prueba manejo memoria ");
    imprime_titulo(&titulo);

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

//************************************* Prueba manejo bit de flags
/*
    let titulo = String::from(" CPU - Simulación CPU - Prueba manejo bit de flags ");
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
*/

//************************************* Pruebas cálculos de flags individuales
/*
    let titulo = String::from(" CPU - Simulación CPU - Prueba cálculos de flags individuales ");
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
    println!("");
*/

//************************************* Pruebas cálculos de flags - ALU
    let titulo = String::from(" CPU - Simulación CPU - Prueba cálculos de flags - ALU ");
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
    //println!("Registro A       {:08b}", cpu_reg.get_a());
    //println!("Registro B       {:08b}", cpu_reg.get_b());
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
    println!();
    

    // Prueba de integración estructura de flags en estructura de registros
    cpu_flags.set_bit(0,true );
    println!("cpu_flags.get_bit(0): {}, cpu_reg.flags.get_bit(0): {}", cpu_flags.get_bit(0), cpu_reg.flags.get_bit(0));
    println!();

//************************************* Mostrar memoria
    let mut vec: [u8; 128] = [0;128];
    for i in 0..vec.len() { vec[i] = (i+0) as u8; }

    vec[30] = 0xAA;
    vec[31] = 0xFF;
    vec[32] = 0xFF;

    muestra_mem(&vec);
    
}

fn muestra_mem(vec: &[u8]) {

    //    let lineas = vec.len() / 16 + if vec.len() % 16 != 0 { 1 } else { 0 };
    println!(" Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    println!("-------------- || ------------------------------------------------");
    
    let mut found_pair = false;
    //let mut dir: u16 = 0;
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
        //dir += 0x10;
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

#[test]
fn test_muestra_memm() {
    // Test para comprobar si la función se detiene correctamente cuando se encuentra el valor hexadecimal "0xFFFF".
    let mut vec: [u8; 40] = [0;40];
    vec[30] = 0xAA;
    vec[31] = 0xFF;
    vec[32] = 0xFF;
    muestra_mem(&vec);
}

