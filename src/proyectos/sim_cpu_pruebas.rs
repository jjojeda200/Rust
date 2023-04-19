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

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::{self, CPU}};
//use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::*};
use colored::*;
//use std::io::{stdin, stdout, Write, Read};

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.red());
}

//***************************************************************************** 
pub struct Aux {
    imp_contador_programa: u16,
    imp_instruccion: u8,
    imp_mnemonico: String,
}

impl CPU{
    fn step_no_win(&mut self, aux: &mut Aux) {
        aux.imp_contador_programa = self.contador_de_programa;
        let instruccion = self.busca_instruccion();
        aux.imp_instruccion = instruccion;
        let (opcode, operandos) = self.decodifica_instruccion(instruccion);
        self.ejecuta_instruccion(opcode, operandos);

        /* (&self).info_registros()
        El paréntesis es necesario para asegurar que se tome la referencia de self antes de llamar al método
        info_registros(). Esto se debe a que el operador . tiene una mayor precedencia que el operador &
        */
//        (&self).info_opcode(opcode, operandos);
//        (&self).info_registros();
//        (&self).info_pruebas();

        //info_pruebas();
    }

    fn run_no_win(&mut self, aux: &mut Aux) {
         loop {
            self.step_no_win(aux);
            println!("Contador: 0x{:04X}, Instruccion: {:02x}, Mnemonic: {},\tReg A: {:02x}, Reg B: {:02x}",
                aux.imp_contador_programa,
                //, self.contador_de_programa -1
                //, self.memoria.leer_memoria(self.contador_de_programa -1)
                aux.imp_instruccion,
                self.mnemonic,
                self.reg_a,
                self.reg_b );

            if self.memoria.leer_memoria(self.contador_de_programa) == 0xFF { break; }
        } 

/*
// ***************
        let mut running = true;
        let mut parameter = 0;
            print!("\nEnter 'c' to change parameter, or 'q' to quit: ");
        while running {
            stdout().flush().unwrap();
    
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
    
            match input.trim() {
                "q" | "Q" => running = false,
                "c" => {
                    println!("Contador: 0x{:04X}, Instruccion: {:02x}, Mnemonic: {},\tReg A: {:02x}, Reg B: {:02x}"
                    , self.contador_de_programa
                    , self.memoria.leer_memoria(self.contador_de_programa)
                    , self.mnemonic
                    , self.reg_a
                    , self.reg_b );
                    self.step_no_win(); 
                },
                _ => println!("Invalid input"),
            }
            if !running { break; }
        }
*/
    }
}

pub fn cpu_sim_0() {
    let titulo = String::from(" Simulación CPU - Pruebas de Funciones, Métodos ");
    imprime_titulo(&titulo);

    // Inicializa la estructura de registros, flags y memoria
    let mut cpu_reg = sim_cpu_registros::CPU::new();
    // Inicializa estructura auxiliar
    let mut aux = Aux {imp_contador_programa: 0x0, imp_instruccion: 0x0, imp_mnemonico: String::new()};

    //**************************************
    let programa = vec![
    0x00,               // NOP
    0x3E, 0x04,         // Almacenar el valor 0x04 en el Registro A
    0x06, 0x0a,         // Almacenar el valor 0x0a en el Registro B
    0x04,               // Incrementa Registro B
    0x80,               // Suma el contenido del Registro B al Registro A
    0x00,               // NOP
    0x3E, 0xf0,         // Almacenar el valor 0xf0 en el Registro A
    0x06, 0x0f,         // Almacenar el valor 0x0f en el Registro B
    0x80,               // Suma el contenido del Registro B al Registro A
    0x00,               // NOP
    0x3E, 0x3b,         // Almacenar el valor 0x3b en el Registro A
    0x3C,               // Incrementa Registro A
    0x32, 0x15, 0x00,   // Mueve el contenido de A a la dirección indicada por los dos bytes siguientes 
    0x00, 0x00,         // <-- Se cambio el contenido y se convierte en 3C
    0x3A, 0x0b, 0x00,   // Mueve el contenido (0x0f) de la dirección indicada (0x0b) en los dos bytes siguientes a A 
    0x00, 0x00,
    0x06, 0xff,         // Almacenar el valor 0xff en el Registro B
    0x80,               // Suma el contenido del Registro B al Registro A
    0x00, 0x00,
    0xFF,
    0xC3, 0x00, 0x00,   // Salta a la dirección 0x0000
    0xFF, 0xFF,         // Marca fin de programa
    ];
    cpu_reg.cargar_programa(&programa);

    cpu_reg.run_no_win(&mut aux);



    //*********************************
    // Para pruebas
    //let mut cpu_reg = sim_cpu_registros::CPU::new();
    //let mut cpu_flags = sim_cpu_registros::Flags::new_flags();
    //let mut memoria = BancosMemoria::new();     // Crea un banco de memoria por defecto de 16384 bytes (16Kb)

    //pru_registros(&mut cpu_reg);                    // Prueba manejo registros
    //pru_flags(&mut cpu_reg, &mut cpu_flags);        // Prueba manejo bit de flags
    //pru_flags_cal(&mut cpu_reg, &mut cpu_flags);    // Pruebas cálculos de flags individuales
    //pru_flags_alu_0(&mut cpu_reg, &mut cpu_flags);  // Pruebas cálculos de flags - ALU
    //pru_mem_0(&mut cpu_reg, &mut memoria);          // Manejo bancos de memoria
    //pru_varias_0(&mut cpu_reg);                     // "fn muestra_mem" y manejo de LittleEndian y BigEndian
}

//***************************************************************************** 


//************************************* Prueba manejo registros
fn pru_registros(cpu_reg: &mut sim_cpu_registros::CPU){
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
fn pru_flags(cpu_reg: &mut sim_cpu_registros::CPU, cpu_flags: &mut sim_cpu_registros::Flags){
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
fn pru_flags_cal(cpu_reg: &mut sim_cpu_registros::CPU, cpu_flags: &mut sim_cpu_registros::Flags) {
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
fn pru_flags_alu_0(cpu_reg: &mut sim_cpu_registros::CPU, cpu_flags: &mut sim_cpu_registros::Flags){
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

//************************************* Manejo estructura registro.bancos de memoria
fn pru_mem_0(cpu_reg: &mut sim_cpu_registros::CPU, memoria: &mut BancosMemoria) {
    let titulo = String::from(" Simulación CPU - Pruebas manejo bancos de memoria ");
    imprime_titulo(&titulo);
    
    // Confirma el banco activo (Banco índice 0)
    let mut num_banco_actual = cpu_reg.memoria.get_banco_activo() as usize;
    // Impresión de verificación

    println!("Banco de memoria Nº: {}, Tamaño del banco: {}, Capacidad del banco de memoria: {} ",
        cpu_reg.memoria.banco_actual,
        cpu_reg.memoria.segmento_memoria[num_banco_actual].len(),
        cpu_reg.memoria.segmento_memoria[num_banco_actual].capacity());

    // Crea un banco de memoria adicional de 32768 bytes (32Kb)
    cpu_reg.memoria.crear_segmento(32768);

    // Selecciona el nuevo banco (Banco índice 1)
    cpu_reg.memoria.set_banco_activo(1);
    num_banco_actual = cpu_reg.memoria.get_banco_activo() as usize;

    // Impresión de verificación
    println!("Banco de memoria Nº: {}, Tamaño del banco: {}, Dirección de memoria (ptr): {:p} ",
        cpu_reg.memoria.banco_actual,
        cpu_reg.memoria.segmento_memoria[num_banco_actual].len(),
        cpu_reg.memoria.segmento_memoria[num_banco_actual].as_ptr());

    // selecciona el primer banco (Banco índice 0)
    cpu_reg.memoria.set_banco_activo(0);
    num_banco_actual = cpu_reg.memoria.get_banco_activo() as usize;

    // escribe un byte en la dirección 0x2000 del primer banco
    cpu_reg.set_b(0xff);
    cpu_reg.set_c(0x00);
    cpu_reg.memoria.escribir_memoria(0x2000, cpu_reg.get_b());
    // lee el byte en la dirección 0x2000 del primer banco
    let byte_1 = cpu_reg.memoria.leer_memoria(0x2000);
    println!("Byte leído en la dirección 8192 (0x2000) del primer banco: 0x{:02x}", byte_1);

    println!(" {:?} ", cpu_reg.memoria.segmento_memoria.len());
    let mut resultado = cpu_reg.memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);
    println!(" {:?} ", cpu_reg.memoria.segmento_memoria.len());
    resultado = cpu_reg.memoria.eliminar_segmento(1);
    println!(" {:?}", resultado);

    resultado = cpu_reg.memoria.eliminar_segmento(0);
    println!(" {:?}", resultado);

    muestra_mem(&cpu_reg.memoria.segmento_memoria[0][0..64]);
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
fn pru_varias_0(cpu_reg: &mut sim_cpu_registros::CPU) {
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