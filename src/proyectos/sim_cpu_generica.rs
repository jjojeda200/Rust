/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          08-04-2023
    Titulo:         Simulación CPU Genérica
    Descripción:    CPU con direccionamiento de 8 bit (por ahora) y opcode del Intel 8080
    Referencias:
    Crate bitflags  https://crates.io/crates/bitflags
    PanCurses       https://crates.io/crates/pancurses

***************************************************************************************/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::Flags};
use pancurses::*;

use std::sync::Mutex;
static mut MNEMONICO_OPCODE: Option<Mutex<String>> = None;

fn imprime_titulo(ventana: &Window, titulo: &str) {
    let max_x = ventana.get_max_x();
    let posicion_x = (max_x - titulo.len() as i32) / 2;
    ventana.mv(1, posicion_x);

    // Definición de combinación de colores
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_YELLOW, COLOR_BLACK);
    ventana.attrset(ColorPair(1));
    ventana.printw(&format!("{}", titulo));
    ventana.attrset(Attribute::Normal);
}

//***************************************************************************** Notas
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

//*****************************************************************************
struct CPU {
    memoria: BancosMemoria,
    flags: Flags,
    memory: [u8; 256],
    program_counter: u8,
    registro: [u8; 8],
    reg_a: u8,   // Registro A de 8 bits
    reg_b: u8,   // Registro B de 8 bits
    reg_c: u8,   // Registro C de 8 bits
    reg_d: u8,   // Registro D de 8 bits
    reg_e: u8,   // Registro E de 8 bits
    reg_h: u8,   // Registro H de 8 bits
    reg_l: u8,   // Registro L de 8 bits
    reg_ix: u16, // Registro IX de 16 bits
    reg_iy: u16, // Registro IY de 16 bits
    contador_de_programa: u16,
    puntero_de_pila: u16,
    registro_instrucciones: u8,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memoria: BancosMemoria {
                segmento_memoria: vec![vec![0; 1024]; 1],
                //segmento_memoria: vec![vec![0; 16384]; 1],
                banco_actual: 0,
            },
            flags: Flags { carry: false, subtract: true, parity_overflow: false, half_carry: false, zero: false, sign: false, },
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

    /* Descripción:
        Función de cargar del programa en la memoria del CPU. Toma como entrada un vector de bytes llamado
        program y usa el método iter() para obtener un iterador sobre los elementos del vector. Luego,
        utiliza el método enumerate() para obtener una tupla que contiene el índice i y una referencia
        &instruction a cada elemento del vector.
        Dentro del cuerpo del for, se asigna el valor de instruction en la posición i de la memoria del
        CPU. En otras palabras, está copiando cada elemento del programa en la memoria del CPU. El índice
        i aumenta en cada iteración para asegurarse de que los elementos se copien en la posición correcta
        de la memoria.
        */
    fn cargar_programa(&mut self, programa: &Vec<u8>) {
        for (i, &instruccion) in programa.iter().enumerate() {
            self.memoria.escribir_memoria(i as u16, instruccion);
        }
    }

    fn cargar_programa0(&mut self, programa: Vec<u8>) {
        for (i, &instruction) in programa.iter().enumerate() {
            self.memory[i] = instruction;
        }
    }

    fn fetch_instruccion(&mut self) -> u8 { 
        // Obtener la instrucción de la memoria en la dirección del contador de programa (1 byte)
        let instruccion = self.memoria.leer_memoria(self.contador_de_programa); 
        // Incrementar el contador de programa para apuntar a la siguiente dirección de memoria
        // self.contador_de_programa += 1;
        instruccion
    }

    fn fetch_instruction(&mut self) -> u8 {
        // Obtener la instrucción de la memoria en la dirección del contador de programa (1 byte)
        let instruction = self.memory[self.program_counter as usize];
        // let instruction = u16::from_le_bytes([self.memory[self.program_counter], self.memory[self.program_counter + 1]]);
        // Incrementar el contador de programa para apuntar a la siguiente dirección de memoria
        //self.contador_de_programa += 1;
        instruction
    }

    fn decode_instruccion(&self, instruccion: u8) -> (u8, [u8; 2]) {
        let opcode = instruccion;
        let operandos = [
            self.memoria.leer_memoria(self.contador_de_programa +1),
            self.memoria.leer_memoria(self.contador_de_programa +2),
            //self.memory[(self.program_counter + 1) as usize],
            //self.memory[(self.program_counter + 2) as usize],
        ];
        (opcode, operandos)
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

        let operands = [
            self.memory[(self.program_counter + 1) as usize],
            self.memory[(self.program_counter + 2) as usize],
        ];
        (opcode, operands)
    }

    fn execute_instruction(&mut self, opcode: u8, operands: [u8; 2]) {
        match opcode {
            0x00 => { // NOP: No hace nada
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("NOP"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
            }

            0x04 => { // INR B incrementa el contenido en el Registro (B)
                self.reg_b += 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR B"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
            }

            0x05 => { // DCR B decrementa el contenido en el Registro (B)
//self.registro[1] -= 1;
                self.reg_b -= 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("DCR B"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
                /* 0x054 sin control de desbordamiento
                    let b = get_register_value(Register::B);
                    let result = b.wrapping_sub(1);
                    set_register_value(Register::B, result);
                    set_flags_sub(b, 1, result);

                ****************************
                El flag Z se establece en 1 si el resultado de la operación es cero (es decir, si el
                valor original de B era 1).
                El flag S se establece en 1 si el bit más significativo del resultado es 1 (es decir,
                si el valor original de B era mayor o igual a 0x80).
                El flag P se establece en 1 si el número de bits 1 en el resultado es par.
                El flag CY no se ve afectado por esta instrucción.
                El flag AC se establece en 1 si ocurre un acarreo en el nibble inferior de la operación
                (es decir, si el valor original de B era menor o igual a 0x0F).

                Por lo tanto, en la implementación en Rust de la instrucción 0x05 en una CPU 8080, la
                función opcode_05 debería actualizar los flags de la CPU según las reglas descritas
                anteriormente:

                impl Cpu {
                    fn opcode_05(&mut self) {
                        let old_value = self.b;
                        self.b = self.b.wrapping_sub(1);
                        self.flags.z = self.b == 0;
                        self.flags.s = (self.b & 0x80) != 0;
                        self.flags.p = self.b.count_ones() % 2 == 0;
                        self.flags.ac = (old_value & 0x0F) == 0;
                    }
                }
                */
            }

            0x06 => { // MVI B,d8 cargar un valor de 8 bits en el Registro (B)
//self.registro[1] = self.memory[(self.program_counter + 1) as usize];
                self.reg_b = self.memoria.leer_memoria(self.contador_de_programa + 1);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI B,d8"))); }
//self.program_counter += 2;
                self.contador_de_programa += 2;
            }

// Pendiente implementar acceso a 16 bit ***
            0x0A => { // LDAX A,(BC) cargar el valor contenido en la dirección BC bits en el acumulador (A)
                self.reg_a = operands[0];
                unsafe {
                    MNEMONICO_OPCODE = Some(Mutex::new(String::from("LDAX A,[BC]")));
                }
            }

            0x3C => { // INR A incrementa el contenido en el Registro (A)
//self.registro[0] += 1;
                self.reg_a += 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR A"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
                /* 0x3C sin control de desbordamiento
                let a = registers.get_a();
                let result = a.wrapping_add(1);
                registers.set_a(result);
                registers.set_flags(Flags::from_increment(a, result));

                ****************************
                fn increment_a(registers: &mut Registers) {
                let a = registers.get_a();
                let result = a.wrapping_add(1);
                registers.set_a(result);

                // Actualizar los flags afectados
                let mut flags = Flags::default();
                flags.set_zero(result == 0);
                flags.set_sign((result & 0x80) != 0);
                flags.set_parity(result.count_ones() % 2 == 0);
                flags.set_auxiliary_carry((a & 0x0f) == 0x0f);
                flags.set_carry(false);
                registers.set_flags(flags);
                }

                En esta implementación, después de actualizar el Registro A, se crea una nueva instancia
                de la estructura Flags y se establecen los flags correspondientes de acuerdo con el
                resultado del incremento.

                El flag Zero se establece si el resultado es cero.
                El flag Sign se establece si el bit más significativo del resultado es 1.
                El flag Parity se establece si el número de bits 1 en el resultado es par.
                El flag Auxiliary Carry se establece si el incremento ha generado un acarreo de 4 bits
                desde los bits más bajos de A.
                El flag Carry se establece en 0 ya que el incremento no ha generado un acarreo desde el
                bit más significativo de A.
                */
            }

            0x3D => { // DCR A decrementa el contenido en el Registro (A)
//self.registro[0] -= 1;
                self.reg_a -= 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("DCR A"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
                /*
                El acumulador en la arquitectura 8080 es un registro especial de 8 bits que almacena los resultados de las operaciones aritméticas y lógicas realizadas por el procesador. Si se decrementa el acumulador en 8080 hasta que llega a 0, esto provocará un desbordamiento (overflow) en el registro, es decir, que el valor almacenado en el acumulador pasará de 0xFF (255 en decimal) a 0x00 (0 en decimal).
                Además, al realizar una operación que decrementa el valor del acumulador, se establecerá la bandera de cero (Z) en 1 si el resultado es 0, o en 0 en caso contrario. También se establecerá la bandera de signo (S) si el bit más significativo del resultado es 1.
                */

                /* 0x3D sin control de desbordamiento
                let a = registers.get_a();
                let result = a.wrapping_sub(1);
                registers.set_a(result);
                registers.set_flags(Flags::from_decrement(a, result));

                ****************************
                El flag de Carry (C) no se ve afectado por esta instrucción.
                El flag de Signo (S) se establece si el resultado de la operación tiene el bit más
                significativo (MSB) en 1, lo que indica que el resultado es negativo. En Rust, podemos
                establecer el flag de Signo de la siguiente manera:

                let sign = result & 0x80 != 0;
                registers.set_sign(sign);

                El flag de Paridad (P/V) se establece si el resultado de la operación tiene un número
                par de bits en 1. En Rust, podemos establecer el flag de Paridad de la siguiente manera:

                let parity = result.count_ones() % 2 == 0;
                registers.set_parity(parity);

                El flag de Ajuste/Substracción (A) se establece en 1 para indicar que se realizó una
                operación de sustracción. En Rust, podemos establecer el flag de Ajuste/Substracción
                de la siguiente manera:

                registers.set_adjust(true);

                El flag de Cero (Z) se establece si el resultado de la operación es cero. En Rust,
                podemos establecer el flag de Cero de la siguiente manera:

                let zero = result == 0;
                registers.set_zero(zero);

                El flag de Desbordamiento (V) se establece si la resta resulta en un valor fuera del
                rango permitido por el tamaño del Registro (en este caso, 8 bits). En Rust, podemos
                establecer el flag de Desbordamiento de la siguiente manera:

                let overflow = a == 0x80;
                registers.set_overflow(overflow);

                Es importante tener en cuenta que, en Rust, la estructura Flags almacena los flags de
                estado de la CPU como booleanos, donde true indica que el flag está activo y false
                indica que el flag está inactivo. La función from_decrement() de la estructura Flags
                se encarga de establecer los valores adecuados de estos booleanos para cada flag
                afectado por la instrucción de decremento de Registro A.
                */
            }

            0x3E => { // MVI A,n cargar un valor de 8 bits en el acumulador (A)
//self.registro[0] = self.memory[(self.program_counter + 1) as usize];
                self.reg_a = self.memoria.leer_memoria(self.contador_de_programa + 1);
                // La siguiente linea para pruebas (borrar) -> 
                // self.flags.flags_paridad(self.reg_a);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI A,d8"))); }
//self.program_counter += 2;
                self.contador_de_programa += 2;
            }

            0x80 => { // ADD A,B suma el contenido del Registro B al acumulador (A)
//self.registro[0] = self.registro[0].wrapping_add(self.registro[1]);
                //self.reg_a = self.reg_a.wrapping_add(self.reg_b);
                let resultado = self.flags.flags_acarreo(self.reg_a, self.reg_b);
                self.reg_a = resultado;
                self.flags.flags_paridad(resultado);
                self.flags.flags_acarreo_auxiliar(resultado);
                self.flags.flags_cero(resultado);
                self.flags.flags_signo(resultado);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("ADD A,B"))); }
//self.program_counter += 1;
                self.contador_de_programa += 1;
            }

// Revisar implementar manejo de direccionamiento de 16 bit *****
            0xC3 => { // JMP nn marca PC con la dirección indicada por los dos siguientes bytes
//self.program_counter = self.memory[(self.program_counter + 1) as usize];
                self.contador_de_programa = self.memoria.leer_memoria(self.contador_de_programa + 1) as u16;
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
            }

// Revisar *********************************
            _ => {
                print!("");
            }
        }
    }

    fn step(&mut self) {
        let instruccion = self.fetch_instruccion();
        let (opcode, operands) = self.decode_instruccion(instruccion);
        //let instruction = self.fetch_instruction();
        //let (opcode, operands) = self.decode_instruction(instruction);
        self.execute_instruction(opcode, operands);

        /* (&self).info_registros()
        El paréntesis es necesario para asegurar que se tome la referencia de self antes de llamar al método
        info_registros(). Esto se debe a que el operador . tiene una mayor precedencia que el operador &
        */
        (&self).info_opcode(opcode, operands);
        (&self).info_registros();
        (&self).info_pruebas();

        //info_pruebas();
    }

    fn run(&mut self, window: &Window) {
        //************************************** Ventana principal
        let mut pos_y = 3;
        loop {
            window.mv(pos_y, 2);
            window.printw(format!( "Contador: 0x{:04X}, Instruccion: {:02x}"
                , self.contador_de_programa
                , self.memory[self.contador_de_programa as usize] ));
            window.printw(format!( " Reg A: {:02x}, Reg B: {:02x}"
                , self.reg_a
                , self.reg_b ));
            pos_y += 1;
            if pos_y == 29 { pos_y = 3; }
            self.step();

            match window.getch() {
                Some(Input::Character(tecla)) => {
                    if tecla == 'q' || tecla == 'Q' {
                        return;
                    }
                }
                Some(Input::KeyDC) => break,
                Some(input) => {
                    window.addstr(&format!("{:?}", input));
                }
                None => (),
            }

            if self.memoria.leer_memoria(self.contador_de_programa) == 0xFF {
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
    let val_y = 50;
    let val_x = 100;
    
    // Verificación medidas de la terminal
    if ventana_principal.get_max_y() < val_y || ventana_principal.get_max_x() < val_x {
        endwin();
        println!(
            "La Terminal es menor del tamaño requerido: {} x {} \n",
            val_y, val_x
        );
        return;
    }

    ventana_principal.resize(30, 60);
    ventana_principal.border('|', '|', '-', '-', '+', '+', '+', '+');
    noecho();
    start_color();
    imprime_titulo(&ventana_principal, &titulo);
    ventana_principal.mvprintw(0, 2, " Salir: q/Q ");
    ventana_principal.refresh();

    //**************************************
    //let mut cpu8080 = RegistrosCPU::new();
    //let mut memoria = BancosMemoria::new();
    //**************************************
    let mut cpu = CPU::new();
    let programa = vec![
        0x00,           // NOP
        0x3E, 0x04,     // Almacenar el valor 0x04 en el Registro A
        0x00,           // NOP
        0x06, 0x0a,     // Almacenar el valor 0x0a en el Registro B
        0x04,           // Incrementa Registro B
        0x80,           // Suma el contenido del Registro B al Registro A
        0x00,           // NOP
        0x3E, 0xff,     // Almacenar el valor 0xff en el Registro A
        0x06, 0xe0,     // Almacenar el valor 0xe0 en el Registro B
        0x00,           // NOP
        0x80,           // Suma el contenido del Registro B al Registro A
        0xC3, 0x00,     // Salta a la dirección 00 (modificar para direccionamiento de 2 bytes)
        0xFF, 0xFF,     // Marca fin de programa
    ];
    cpu.cargar_programa(&programa);
    cpu.cargar_programa0(programa.clone());
    //cpu.info_pruebas(0000);

    cpu.run(&ventana_principal);

    //**************************************
    echo();
    endwin();
}

//*****************************************************************************
/*
fn info_pruebas() {
    let titulo_ventana_comentarios = String::from(" Pruebas / Info");
    let comentarios_window = newwin(10, 60, 30, 0);
    comentarios_window.border('|', '|', '-', '-', '+', '+', '+', '+');
    imprime_titulo(&comentarios_window, &titulo_ventana_comentarios);
    comentarios_window.refresh();

    let pos_y = 2;
    let pos_x = 2;

    comentarios_window.mv(pos_y, 2);

    let inst0: u8 = 0b11011010;
    let inst1: u8 = inst0 & 0x0F;
    let inst2 = ((inst1 as u16) << 8) | (inst0 as u16);
    comentarios_window.mvprintw(pos_y+1, pos_x, format!("Inst0: {:08b}", inst0));
    comentarios_window.mvprintw(pos_y+2, pos_x, format!("Inst1: {:08b}", inst1));
    comentarios_window.mvprintw(pos_y+3, pos_x, format!("Inst1: {:08b}", inst2));

    //comentarios_window.mvprintw(pos_y+3, pos_x, format!("Inst1: {:08b}", mem.leer_byte(0000)));
    //comentarios_window.mvprintw(pos_y+1, pos_x, format!("{:?}", self.));

    comentarios_window.refresh();
}
*/

impl CPU {                                   // Funciones de manejo de ventanas
    
    // Función manejo ventana de info / pruebas
    fn info_pruebas(&self /* , mem: u8 */) {
        let titulo_ventana_comentarios = String::from(" Pruebas / Info");
        let comentarios_window = newwin(21, 90, 29, 0);
        comentarios_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&comentarios_window, &titulo_ventana_comentarios);
        comentarios_window.refresh();

        let pos_y = 2;
        let pos_x = 2;
        comentarios_window.mv(pos_y, pos_x);

        comentarios_window.mvprintw( pos_y + 0, pos_x, format!("Direccion a la que apunta PC de memoria: {:04X}", self.program_counter));
        comentarios_window.mvprintw( pos_y + 1, pos_x, format!("Contenido en memory a la que apunta PC : {:02X}", self.memory[(self.program_counter) as usize]));
        comentarios_window.mvprintw( pos_y + 2, pos_x, format!("Contenido registro[0]: {:02X}", self.registro[0]));
        comentarios_window.mvprintw( pos_y + 3, pos_x, format!("Contenido registro[1]: {:02X}", self.registro[1]));

        comentarios_window.mvprintw( pos_y + 5, pos_x, format!("Acarreo {}, Paridad: {}, Acarreo Auxiliar {}, Zero {}, Signo {}"
                                   , self.flags.get_bit(0)
                                   , self.flags.get_bit(2)
                                   , self.flags.get_bit(4)
                                   , self.flags.get_bit(6)
                                   , self.flags.get_bit(7)));

        comentarios_window.attrset(ColorPair(2));
        comentarios_window.mvprintw( pos_y + 6, pos_x, "*****************************************************************");

        
        //let var_a_array: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
        let mut var_a_array = &self.memoria.segmento_memoria[*(&self.memoria.get_banco_activo()) as usize];
        muestra_mem(&comentarios_window, 9, 2, var_a_array);
        //muestra_mem_obj(&comentarios_window, 8, 2, var_a_array);
        //muestra_mem_obj(&comentarios_window, 8, 2, (&self).memoria.segmento_memoria.clone());

        /*
        comentarios_window.mvprintw(pos_y+1,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+2,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+3,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+4,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+5,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+6,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+7,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+8,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+9,  pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+10, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+11, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+12, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+13, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+14, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+15, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+16, pos_x, format!("", ));
        comentarios_window.mvprintw(pos_y+17, pos_x, format!("", ));
        */

        // pruebas_00(&comentarios_window, pos_y, pos_x);

        comentarios_window.attrset(Attribute::Normal);
        comentarios_window.refresh();
    }

    // Función manejo ventana de los registros
    fn info_registros(&self) {
        let titulo_ventana_reg = String::from(" Registros ");
        //let max_x = window.get_max_x();
        let reg_window = newwin(12, 16, 0, 59);
        reg_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&reg_window, &titulo_ventana_reg);

        reg_window.mvprintw(2, 2, format!("A: {:02X}", self.reg_a));
        reg_window.mvprintw(3, 2, format!("B: {:02X}", self.reg_b));
        reg_window.mvprintw(3, 9, format!("C: {:02X}", self.reg_c));
        reg_window.mvprintw(4, 2, format!("D: {:02X}", self.reg_d));
        reg_window.mvprintw(4, 9, format!("E: {:02X}", self.reg_e));
        reg_window.mvprintw(5, 2, format!("H: {:02X}", self.reg_h));
        reg_window.mvprintw(5, 9, format!("L: {:02X}", self.reg_l));
        reg_window.mvprintw(7, 2, format!("iX: {:04X}", self.reg_ix));
        reg_window.mvprintw(8, 2, format!("iY: {:04X}", self.reg_iy));
        reg_window.refresh();
    }

    // Función manejo ventana de los OP Code
    fn info_opcode(&self, opcode: u8, operandos: [u8; 2]) {
        let titulo_ventana_opcode = String::from(" OP Code ");
        let pos_x = 74;
        let opcode_window = newwin(12, 16, 0, pos_x);
        opcode_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&opcode_window, &titulo_ventana_opcode);
        let pos_y = opcode_window.get_cur_y();

        let mnemonico_opcode = unsafe { MNEMONICO_OPCODE.as_ref().unwrap().lock().unwrap() };
        opcode_window.mvprintw(2, 2, format!("{}", mnemonico_opcode));
        opcode_window.mvprintw(3, 2, format!("Hex: 0x{:02X}", opcode));
        opcode_window.mvprintw(5, 2, format!("PC : {:04x}", self.contador_de_programa));
        opcode_window.mvprintw(6, 2, format!("Contenido en"));
        opcode_window.mvprintw(7, 2, format!("PC +1: 0x{:02X}", operandos[0]));
        opcode_window.mvprintw(8, 2, format!("PC +2: 0x{:02X}", operandos[1]));
        opcode_window.refresh();
    }
}

//***************************************************************************** Muestra memoria en formato tabla
fn muestra_mem(comentarios_window: &Window, mut pos_y: i32, mut pos_x: i32, vec: &[u8], /* , size: usize, ancho: usize */) {

    // let mut lineas = vec.len() / 16;
    // if (vec.len() & 0xf) != 0 {
    //     lineas += 1;
    // }

    comentarios_window.mv(pos_y, pos_x);
    comentarios_window.mvprintw( pos_y, pos_x,format!(" Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F"));
    comentarios_window.mvprintw( pos_y + 1, pos_x,format!("-------------- || -----------------------------------------------"));
    pos_y = pos_y + 1;
    pos_x = 20;
    comentarios_window.mv(pos_y, pos_x);

/* Análisis Lógico                          
1-  Se definen variables mutables "doble_ff", "ultimo_byte", "lineas", "dir" y "buffer".
2-  Se inicia un bucle for que iterará sobre los elementos del vector "vec" en grupos de 16 elementos, utilizando el método "chunks(16)".
3-  Dentro del bucle for se evalúa la condición "if doble_ff == true { break; }". Si la condición es verdadera, se sale del bucle.
4-  Se limpia el contenido del buffer para recibir un nuevo grupo de elementos.
5-  Se inicia otro bucle for que recorre cada elemento del grupo actual, usando la variable "i" para mantener el índice actual.
6-  Se guarda el valor actual de "group[i]" en la variable "byte".
7-  Si "i" es igual a 0 y el valor de "byte" es igual a 0xff y el valor de "ultimo_byte" es igual a 0xff, se establece el valor de "doble_ff" a true y se sale del bucle.
8-  Si "i" es igual al tamaño del grupo menos 1 y el valor de "byte" es igual a 0xff, se guarda el valor de "byte" en la variable "ultimo_byte".
9-  Si el valor de "byte" es igual a 0xff y "i" es menor que el tamaño del grupo menos 1 y el siguiente elemento del grupo es igual a 0xff, se establece el valor de "doble_ff" a true y se sale del bucle.
10- Se agrega la representación en hexadecimal de "byte" al buffer.
11- Se muestran los elementos del buffer en una ventana utilizando la función "mvprintw" de la biblioteca ncurses.
12- Se actualiza la posición vertical de la ventana, se incrementa el valor de "dir" en 0x10 y se actualiza el valor de "lineas".
13- Si "lineas" es igual a 8, se establece el valor de "doble_ff" a true y se sale del bucle. De lo contrario, se incrementa el valor de "lineas".
14- Se repite el bucle for desde el punto 3 hasta que se hayan recorrido todos los grupos del vector "vec" o hasta que se establezca el valor de "doble_ff" a true.
*/
/* Diagrama de flujo                        
Inicio
|
|__ Inicializar variables mutables
|__ Iniciar bucle for para recorrer grupos del vector vec
   |
   |__ Evaluar si se ha encontrado doble_ff
   |   |
   |   |__ Si es verdadero, salir del bucle
   |   |
   |   |__ Si es falso, continuar con el flujo
   |
   |__ Limpiar el buffer
   |
   |__ Iniciar bucle for para recorrer elementos del grupo actual
      |
      |__ Guardar el valor actual de byte
      |
      |__ Evaluar si i es igual a 0 y byte es igual a 0xff y ultimo_byte es igual a 0xff
      |   |
      |   |__ Si es verdadero, establecer doble_ff a true y salir del bucle
      |   |
      |   |__ Si es falso, continuar con el flujo
      |
      |__ Evaluar si i es igual al tamaño del grupo menos 1 y byte es igual a 0xff
      |   |
      |   |__ Si es verdadero, guardar byte en ultimo_byte
      |   |
      |   |__ Si es falso, continuar con el flujo
      |
      |__ Evaluar si byte es igual a 0xff y i es menor que el tamaño del grupo menos 1 y el siguiente elemento del grupo es igual a 0xff
      |   |
      |   |__ Si es verdadero, establecer doble_ff a true y salir del bucle
      |   |
      |   |__ Si es falso, continuar con el flujo
      |
      |__ Agregar la representación en hexadecimal de byte al buffer
      |
      |__ Mostrar los elementos del buffer en la ventana
      |
      |__ Actualizar la posición vertical de la ventana, dir y lineas
      |
      |__ Evaluar si lineas es igual a 8
      |   |
      |   |__ Si es verdadero, establecer doble_ff a true y salir del bucle
      |   |
      |   |__ Si es falso, incrementar lineas
      |
   |
Fin del bucle for de elementos del grupo actual
|
|__ Repetir el bucle for de grupos del vector vec si doble_ff es falso
|
Fin del bucle for de grupos del vector vec o si doble_ff es verdadero

*/

    let mut doble_ff = false;
    let mut ultimo_byte: u8 = 0;
    let mut lineas: u8 = 0;
    let mut dir: u16 = 0;
    let mut buffer = String::new();
    for group in vec.chunks(16) {
        if doble_ff == true { break; }

        buffer.clear();                     // Limpiar el buffer para el nuevo grupo
        for i in 0..group.len() {
            let byte = group[i];

            if i == 0 && byte == 0xff && ultimo_byte == 0xff {
                doble_ff = true;
                break;
            }

            if i == (group.len() - 1)  && group[i] == 0xff { ultimo_byte = byte; }

            if byte == 0xff && i < group.len() - 1 && group[i + 1] == 0xff {
                doble_ff = true;
                //pos_y +=1;
                break;
            }

            buffer.push_str(&format!("{:02X} ", byte));
        }
        comentarios_window.mvprintw(pos_y as i32, 2, format!("       {:04x}    || ", dir));
        comentarios_window.mvprintw(pos_y as i32, pos_x, &buffer);
        pos_y += 1;
        dir += 0x10;
        if lineas == 8 { doble_ff = true; } else { lineas += 1}
    }
    comentarios_window.refresh();

}

//***************************************************************************** pruebas_00 "Mascaras de bits"
fn pruebas_00(comentarios_window: &Window, pos_y: i32, pos_x: i32) {
    comentarios_window.attrset(ColorPair(2));
    // Get
    comentarios_window.mvprintw( pos_y + 6, pos_x, format!( "Creacion de un byte activando el bit 0      : {:08b}", (1 << 0)),);
    comentarios_window.mvprintw( pos_y + 7, pos_x, format!( "Creacion de un byte activando el bit 7      : {:08b}", (1 << 7)),);
    let mut num: u8 = 0x00;
    comentarios_window.mvprintw( pos_y + 8, pos_x, format!( "Mascara de bits (AND): {:08b}, con & (1<<1) : {:08b}", num, (num & (1 << 1))),);
    comentarios_window.mvprintw( pos_y + 9, pos_x, format!( "Esta activo el bit 1? con & (1<<1)          : {}", (num & (1 << 1)) != 0 ),);
    num = 0xff;
    comentarios_window.mvprintw( pos_y + 10, pos_x, format!( "Mascara de bits (AND): {:08b}, con & (1<<1) : {:08b}", num, (num & (1 << 1))),);
    comentarios_window.mvprintw( pos_y + 11, pos_x, format!( "Esta activo el bit 1? con & (1<<1)          : {}", (num & (1 << 1)) != 0 ),);
    // Reset y Set
    comentarios_window.mvprintw( pos_y + 12, pos_x, format!( "Desactivando el bit 1                       : {:08b}", (num & !(1 << 1))),);
    comentarios_window.mvprintw( pos_y + 13, pos_x, format!( "Esta activo el bit 1? con & (1<<1)          : {}", (num & (1 << 1)) == 1),);
    comentarios_window.mvprintw( pos_y + 14, pos_x, format!( "Activando el bit 1                          : {:08b}", (num | (1 << 1))),);
    comentarios_window.mvprintw( pos_y + 15, pos_x, format!( "Esta activo el bit 1? con & (1<<1)          : {}", (num & (1 << 1)) != 0),);
    comentarios_window.attrset(Attribute::Normal);
}

//*****************************************************************************


//*****************************************************************************