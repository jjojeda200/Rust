/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          23-03-2023
    Titulo:         Simulación CPU Genérica
    Descripción:    CPU con direccionamiento de 8 bit (por ahora) y opcode del Intel 8080
    Referencias:    
    Crate bitflags  https://crates.io/crates/bitflags
    PanCurses       https://crates.io/crates/pancurses

***************************************************************************************/

#![allow(dead_code)]
#![allow(unused_variables)]

use pancurses::*;
use std::sync::Mutex;

use super::sim_cpu_registros::RegistrosCPU;

static mut MNEMONICO_OPCODE: Option<Mutex<String>> = None;

fn imprime_titulo(ventana: &Window, titulo: &str) {
    let max_x = ventana.get_max_x();
    let posicion_x = (max_x - titulo.len() as i32) / 2;
    ventana.mv(1, posicion_x);

    // Definición de combinación de colores
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    ventana.attrset(ColorPair(1));
    ventana.printw(&format!("{}", titulo));
    ventana.attrset(Attribute::Normal);
}

//***************************************************************************** 
/* Introducción:                            
Métodos parte de una implementación de una máquina virtual o un emulador de un procesador:
*   self.fetch_instruction(): Este método es responsable de obtener la siguiente instrucción
    del programa que se está ejecutando y devolverla al llamador. En una arquitectura real,
    esto implicaría leer la instrucción desde la memoria, posiblemente descodificar una dirección
    de memoria o un puntero de instrucción para obtener la ubicación de la instrucción en la
    memoria, y luego leer los bytes correspondientes de la memoria para obtener la instrucción real.

 *  self.decode_instruction(instruction): Una vez que se ha obtenido la instrucción del programa, el
    siguiente paso es decodificarla para determinar qué operación se debe realizar y qué operandos
    se deben usar para realizar esa operación. El formato de la instrucción variará según la
    arquitectura del procesador, pero típicamente incluirá un opcode que indica la operación a
    realizar y uno o más operandos que indican los datos sobre los que se debe realizar la operación.

*   match opcode { ... }: Una vez que se ha decodificado la instrucción, se utiliza el opcode para
    determinar qué operación debe realizarse. En el código proporcionado, se utiliza una estructura
    de control match para manejar cada opcode en particular. El código unimplemented!() indica que
    el comportamiento para este opcode aún no se ha implementado.
*/
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
*/
/* Manejar los bit 3, 4 y 5 de un byte      
    fn main() {
        let byte: u8 = 0b11100101; // byte de ejemplo
        
        // Máscara con los bits 3, 4 y 5 establecidos en 1 y los demás en 0
        let mask: u8 = 0b00111000;
        
        // Comprobación de los bits 3, 4 y 5
        if byte & mask == mask {
            println!("Los bits 3, 4 y 5 están establecidos en 1");
        } else {
            println!("Los bits 3, 4 y 5 no están establecidos en 1");
        }
        
        // Manipulación de los bits 3, 4 y 5
        let new_byte = (byte & !mask) | (0b010 << 3); // Establece los bits 3, 4 y 5 en 010
        println!("Byte original: {:08b}", byte);
        println!("Byte modificado: {:08b}", new_byte);
    }
*/

struct CPU {
    memory: [u8; 256],
    program_counter: u8,
    registro: [u8; 8],
    reg_a: u8,      // Registro A de 8 bits
    reg_b: u8,      // Registro B de 8 bits
    reg_c: u8,      // Registro C de 8 bits
    reg_d: u8,      // Registro D de 8 bits
    reg_e: u8,      // Registro E de 8 bits
    reg_h: u8,      // Registro H de 8 bits
    reg_l: u8,      // Registro L de 8 bits
    reg_ix: u16,    // Registro IX de 16 bits
    reg_iy: u16,    // Registro IY de 16 bits
    contador_de_programa: u16,
    puntero_de_pila: u16,
    registro_instrucciones: u8,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: [0; 256],
            program_counter: 0,
            registro: [0; 8],
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_ix: 0,
            reg_iy: 0,
            contador_de_programa: 0,
            puntero_de_pila: 0,
            registro_instrucciones: 0,
        }
    }

    fn load_program(&mut self, program: Vec<u8>) {
    /* Descripción:                         
    Este fragmento de código es una forma de cargar el programa en la memoria del CPU. Toma como
    entrada un vector de bytes llamado program y usa el método iter() para obtener un iterador sobre
    los elementos del vector. Luego, utiliza el método enumerate() para obtener una tupla que contiene
    el índice i y una referencia &instruction a cada elemento del vector.
    Dentro del cuerpo del for, se asigna el valor de instruction en la posición i de la memoria del
    CPU. En otras palabras, está copiando cada elemento del programa en la memoria del CPU. El índice
    i aumenta en cada iteración para asegurarse de que los elementos se copien en la posición correcta
    de la memoria.
    */
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
    }

    fn fetch_instruction(&mut self) -> u8 {
        // Obtener la instrucción de la memoria en la dirección del contador de programa (1 byte)
        let instruction = self.memory[self.program_counter as usize];
        // let instruction = u16::from_le_bytes([self.memory[self.program_counter], self.memory[self.program_counter + 1]]);
        /* Manejo Little Endian (2 bytes) ^^
        let instruction = self.memory[self.program_counter as usize];
        La instrucción activa extrae un byte de la memoria en la posición actual del contador del
        programa (self.program_counter) y lo almacena en la variable "instruction". Esta instrucción
        asume que la memoria almacena cada opcode en un solo byte.
        
        let instruction = u16::from_le_bytes([self.memory[self.program_counter], self.memory[self.program_counter + 1]]);
        La segunda instrucción, en cambio, extrae dos bytes de la memoria en las posiciones actual y
        siguiente del contador del programa (self.program_counter y self.program_counter + 1), y los
        convierte en un valor de 16 bits en orden "little-endian" utilizando el método from_le_bytes
        de la estructura u16. Este valor de 16 bits resultante es almacenado en la variable "instruction".
        Esta instrucción asume que los opcodes se almacenan en dos bytes consecutivos en la memoria.
        */

        // Incrementar el contador de programa para apuntar a la siguiente dirección de memoria
        self.program_counter += 1;
        instruction
    }

    fn decode_instruction(&self, instruction: u8) -> (u8, [u8; 2]) {
        /* let opcode = instruction >> 4;   
        El código instruction >> 4 es una operación de desplazamiento a la derecha a nivel de bits.
        El operador >> desplaza los bits de la variable instruction hacia la derecha en 4 posiciones.
        El resultado de la operación se asigna luego a la variable opcode.
        Esto asume que la variable instruction contiene un valor que representa una instrucción de en
        formato binario, donde los primeros cuatro bits especifican el opcode.
        Ejemplo:
            let inst0: u8 = 0b10110100;
            println!("El valor de inst0 es: {:b}", inst0);  // El valor de inst0 es: 10110100
            let inst1 = inst0 >> 4;
            println!("El valor de inst1 es: {:b}", inst1);  // El valor de inst1 es: 1011
        */
        let opcode = instruction;

        // let operands = [instruction & 0x0F, self.memory[self.program_counter as usize]];
        /* instruction & 0x0F             ^^
        La instrucción que se presenta en la pregunta utiliza el operador "&" (AND binario) para
        hacer una operación bit a bit entre la variable "instruction" y el valor hexadecimal "0x0F".
        "0x0F" en hexadecimal representa el número 00001111 en binario. Al utilizar el operador "&"
        entre "instruction" y "0x0F", se realiza una operación lógica AND entre los bits de cada
        número en la misma posición, de la siguiente manera:
            let inst0: u8 = 0b11011010;     // variable con valor binario 11011010
            let inst1: u8 = inst0 & 0x0F;   // operación AND con 0x0F (00001111)
            println!("Inst0: {:08b}", inst0);   // imprime Inst0: 11011010
            println!("Inst1: {:08b}", inst1);   // imprime Inst1: 00001010
        */
        
        let operands = [self.memory[self.program_counter as usize ], self.memory[(self.program_counter + 1) as usize ]];
        (opcode, operands)
    }

    fn execute_instruction(&mut self, opcode: u8, operands: [u8; 2]) {
        let titulo_ventana_opcode = String::from(" OP Codes ");
        let pos_x = 60;
        let opcode_window = newwin(10, 16, 10, pos_x);
        opcode_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&opcode_window, &titulo_ventana_opcode);
        let pos_y = opcode_window.get_cur_y();

        match opcode {
            0x00 => { // NOP: No hace nada
                //self.program_counter += 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("NOP"))); }
            },

            0x04 => { // INR B incrementa el contenido en el registro (B)
                self.registro[1] += 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR B"))); }
                /* 0x04 con contro de desbordamiento
                let b = registers.get_b();
                let result = b.wrapping_add(1);
                registers.set_b(result);
                registers.set_flags(Flags::from_increment(b, result));
                */
            },

            0x06 => { // MVI B,d8 cargar un valor de 8 bits en el registro (B)
                self.registro[1] = self.memory[self.program_counter as usize];
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI B,d8"))); }
                self.program_counter += 1;
            },

// Pendiente implementar acceso a 16 bit ***
            0x0A => { // LDAX A,(BC) cargar el valor contenido en la dirección BC bits en el acumulador (A)
                self.registro[0] = operands[0];
                unsafe {
                    MNEMONICO_OPCODE = Some(Mutex::new(String::from("LDAX A,[BC]")));
                }
            },

            0x3C => { // INR A incrementa el contenido en el registro (A)
                self.registro[0] += self.registro[0];
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR A"))); }
                /* 0x3C con contro de desbordamiento
                let a = registers.get_a();
                let result = a.wrapping_add(1);
                registers.set_a(result);
                registers.set_flags(Flags::from_increment(a, result));
                */
            },

            0x3E => { // MVI A,n cargar un valor de 8 bits en el acumulador (A)
                self.registro[0] = self.memory[self.program_counter as usize];
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI A,d8"))); }
                self.program_counter += 1;
            },

            0x80 => { // ADD A,B suma el contenido del registro B al acumulador (A)
                self.registro[0] = self.registro[0].wrapping_add(self.registro[1]);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("ADD A,B"))); }
                /* 0x80 Sin propagación de acarreo  
                let a: u8 = get_register_value(Register::A);    // obtener el valor del registro A
                let b: u8 = get_register_value(Register::B);    // obtener el valor del registro B
                let result = a.wrapping_add(b);                 // suma sin propagación de acarreo (wrapping add)
                set_register_value(Register::A, result);        // guardar el resultado en el registro A
                */
                /* 0x88 Con propagación de acarreo  
                let a: u8 = get_register_value(Register::A);    // obtener el valor del registro A
                let b: u8 = get_register_value(Register::B);    // obtener el valor del registro B
                let (result, carry) = a.overflowing_add(b);     // suma con propagación de acarreo (overflowing add)
                set_register_value(Register::A, result);        // guardar el resultado en el registro A
                if carry {
                    set_flag(Flag::C);      // establecer la bandera de acarreo si se produce acarreo
                } else {
                    clear_flag(Flag::C);    // borrar la bandera de acarreo si no se produce acarreo
                }
                */
            },

// Revisar implementar acceso a 16 bit *****
            0xC3 => { // JMP nn marca PC con la dirección indicada por los dos siguientes bytes 
                self.program_counter = self.memory[self.program_counter as usize];
                //self.registro[0] = self.memory[self.program_counter as usize];
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("JMP nn"))); }
                /* Instrucción 0xC3 manejando direcciones de 16 bits
                fn 0x3C (address: u16) {
                    // La dirección a la que saltaremos es una palabra de 16 bits, por lo que
                    // debemos leer dos bytes de memoria consecutivos.
                    let high_byte = memory[address as usize];
                    let low_byte  = memory[(address + 1) as usize];
                    // Combinamos los dos bytes leídos en una sola dirección de memoria de 16 bits.
                    /* ((high_byte as u16) << 8) | (low_byte as u16)
                    Convertimos el byte más significativo (high_byte) en un valor u16 utilizando la sintaxis as u16.
                    Luego, utilizamos el operador de desplazamiento a la izquierda << para mover este byte 8 bits
                    hacia la izquierda, lo que coloca los bits del byte en las posiciones más altas del valor de 16
                    bits. A continuación, convertimos el byte menos significativo (low_byte) en un valor u16 de la
                    misma manera. Finalmente, combinamos los dos valores u16 utilizando el operador OR |, lo que
                    establece los bits del byte menos significativo en las posiciones más bajas del valor de 16 bits.
                    Ejemplo:
                        let inst0: u8 = 0b11011010;         // variable con valor binario 11011010
                        let inst1: u8 = inst0 & 0x0F;       // operación AND con 0x0F (00001111)
                        println!("Inst0: {:08b}", inst0);   // imprime Inst0: 11011010
                        println!("Inst1: {:08b}", inst1);   // imprime Inst1: 00001010
                        let inst2 = ((inst1 as u16) << 8) | (inst0 as u16);
                        println!("Inst2: {:016b}", inst2);  // imprime Inst2: Inst2: 0000101011011010
                    */
                    let jump_address = ((high_byte as u16) << 8) | (low_byte as u16);
                    // Saltamos a la dirección de memoria especificada.
                    pc = jump_address;
                }
                */
            },

// Revisar *********************************
            0xFF => println!("Fin del programa"),
            _ =>    { print!(""); },
        }
        opcode_window.refresh();
    }

    fn step(&mut self) {
        let instruction = self.fetch_instruction();
        let (opcode, operands) = self.decode_instruction(instruction);
        //println!(" {:02x}", instruction);
        self.execute_instruction(opcode, operands);
        /* (&self).info_registros()         
        El paréntesis es necesario para asegurar que se tome la referencia de self antes de llamar al método
        info_registros(). Esto se debe a que el operador . tiene una mayor precedencia que el operador &
        */
        (&self).info_opcode(opcode, operands);
        (&self).info_registros();
        info_pruebas();
    }

    fn run(&mut self, window: &Window) {

    //************************************** Ventana principal
        let mut pos_y = 3;
        loop {
            window.mv(pos_y, 2);
            window.printw(format!("Contador: 0x{:02x}, Instruccion: {:02x}", self.program_counter, self.memory[self.program_counter as usize]));
            window.printw(format!(" Reg A: {:02x}, Reg B: {:02x}", self.registro[0], self.registro[1]));
            pos_y += 1;
            if pos_y == 29 {pos_y = 3;}
            self.step();
            
            match window.getch() {
                Some(Input::Character(tecla)) => {
                    if tecla == 'q' || tecla == 'Q' {
                        return;
                    }
                },
                Some(Input::KeyDC) => break,
                Some(input) => { window.addstr(&format!("{:?}", input)); },
                None => ()
            }

            if self.memory[self.program_counter as usize] == 0xFF {
                break;
            }
        }
    }

}

pub fn cpu_generica_0() {
    let titulo = String::from(" Simulacion CPU Generica 0 ");
    let mut ventana_principal = initscr();
    /* Comprobación de coordenadas          
    ventana_principal.printw(format!("{}, {}", ventana_principal.get_max_y(), ventana_principal.get_max_x()));
    ventana_principal.refresh();
    ventana_principal.getch();
    */
    ventana_principal.resize(30,60);
    ventana_principal.border('|', '|', '-', '-', '+', '+', '+', '+');
    noecho();
    start_color();
    imprime_titulo(&ventana_principal, &titulo);
    ventana_principal.refresh();

    //**************************************
    let mut cpu8080 = RegistrosCPU::new();


    //**************************************
    let mut cpu = CPU::new();
    let program = vec![
        0x00,               // NOP
        0x3E, 0x04,         // Almacenar el valor 4 en el registro 0 (A)
        0x00,               // NOP
        0x06, 0x0a,         // Almacenar el valor 4 en el registro 0 (B)
        0x04,               // Incrementa registro 1 (B)
        0x80,               // Suma el contenido del Registro 1 (B) al registro 0 (A)
        0xC3, 0x00,         // Salta a la dirección 00 (modificar para direccionamiento de 2 bytes)
    ];

    cpu.load_program(program);
    cpu.run(&ventana_principal);

    //**************************************

    endwin();
}

//*****************************************************************************
fn info_pruebas() {
    let titulo_ventana_comentarios = String::from(" Pruebas / Info");
    let comentarios_window = newwin(10, 60, 30, 0);
    comentarios_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    imprime_titulo(&comentarios_window, &titulo_ventana_comentarios);
    //comentarios_window.refresh();

    let pos_y = 2;
    let pos_x = 2;

    comentarios_window.mv(pos_y, 2);

    let inst0: u8 = 0b11011010;
    let inst1: u8 = inst0 & 0x0F;
    let inst2 = ((inst1 as u16) << 8) | (inst0 as u16);
    comentarios_window.mvprintw(pos_y+1, pos_x, format!("Inst0: {:08b}", inst0));
    comentarios_window.mvprintw(pos_y+2, pos_x, format!("Inst1: {:08b}", inst1));
    comentarios_window.mvprintw(pos_y+3, pos_x, format!("Inst2: {:016b}", inst2));

    comentarios_window.refresh();
}

impl CPU{   // Funciones de manejo de ventanas

    // Función manejo ventana de los registros
    fn info_registros(&self) {
        let titulo_ventana_reg = String::from(" Registros ");
        //let max_x = window.get_max_x();
        let reg_window = newwin(12, 16, 0, 60);
        reg_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&reg_window, &titulo_ventana_reg);

        reg_window.mvprintw(2, 2, format!("A: {:02X}", self.registro[0]));
        reg_window.mvprintw(3, 2, format!("B: {:02X}", self.registro[1]));
        reg_window.mvprintw(3, 9, format!("C: {:02X}", self.registro[2]));
        reg_window.mvprintw(4, 2, format!("D: {:02X}", self.registro[3]));
        reg_window.mvprintw(4, 9, format!("E: {:02X}", self.registro[4]));
        reg_window.mvprintw(5, 2, format!("H: {:02X}", self.registro[5]));
        reg_window.mvprintw(5, 9, format!("L: {:02X}", self.registro[5]));
        reg_window.mvprintw(7, 2, format!("iX: {:04X}", self.reg_ix));
        reg_window.mvprintw(8, 2, format!("iY: {:04X}", self.reg_iy));
        reg_window.refresh();
    }

    // Función manejo ventana de los OP Code
    fn info_opcode(&self, opcode: u8, operandos: [u8;2] ) {
        let titulo_ventana_opcode = String::from(" OP Code ");
        let pos_x = 60;
        let opcode_window = newwin(10, 16, 12, pos_x);
        opcode_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&opcode_window, &titulo_ventana_opcode);
        let pos_y = opcode_window.get_cur_y();

        let mnemonico_opcode = unsafe { MNEMONICO_OPCODE.as_ref().unwrap().lock().unwrap() };
        opcode_window.mvprintw(2, 2,format!("{}", mnemonico_opcode));
        opcode_window.mvprintw(3, 2,format!("Hex: 0x{:02X}", opcode));
        opcode_window.mvprintw(4, 2,format!("PC : {:04x}", self.program_counter));
        opcode_window.mvprintw(6, 2,format!("Contenido en"));
        opcode_window.mvprintw(7, 2,format!("PC +1: 0x{:02X}", operandos[0]));
        opcode_window.mvprintw(8, 2,format!("PC +2: 0x{:02X}", operandos[1]));
        opcode_window.refresh();
    }

}

//***************************************************************************** 