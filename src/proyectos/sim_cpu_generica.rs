/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-04-2023
    Titulo:         Simulación CPU Genérica
    Descripción:    CPU con direccionamiento de 8 bit (por ahora) y opcode del Intel 8080
    Referencias:
    PanCurses       https://crates.io/crates/pancurses

    Crate bitflags  https://crates.io/crates/bitflags

***************************************************************************************/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_memoria::Endianess};
use super::{sim_cpu_registros::RegistrosCPU, sim_cpu_registros::Flags};
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

//*****************************************************************************
struct CPU {
    memoria: BancosMemoria,
    flags: Flags,
    reg_a: u8,   // Acumulador A de 8 bits
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
                endianess: Endianess::LittleEndian,
            },
            flags: Flags { carry: false, subtract: true, parity_overflow: false, half_carry: false, zero: false, sign: false, },
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

    fn cargar_programa(&mut self, programa: &Vec<u8>) {
        // Iterar a través de cada instrucción del programa proporcionado.
        for (i, &instruccion) in programa.iter().enumerate() {
            // Escribir cada instrucción en la memoria en la posición de memoria correspondiente.
            // La posición de memoria se determina por el índice actual de iteración `i`.
            // Se convierte a u16 para garantizar que sea una dirección de memoria válida.
            self.memoria.escribir_memoria(i as u16, instruccion);
        }
    }

    fn busca_instruccion(&mut self) -> u8 { 
        // Obtener la instrucción de la memoria en la dirección del contador de programa (1 byte)
        let instruccion = self.memoria.leer_memoria(self.contador_de_programa); 
        instruccion
    }

    // Definición de la función 'decodifica_instruccion', que toma un byte 'instruccion' como
    // argumento y devuelve una tupla de un byte 'opcode' y una matriz de dos bytes 'operandos'.
    fn decodifica_instruccion(&self, instruccion: u8) -> (u8, [u8; 2]) {
        // Asignación del byte 'instruccion' a la variable 'opcode'.
        let opcode = instruccion;
        // Definición de la matriz 'operandos', que se compone de los dos bytes siguientes en la
        // memoria en las posiciones de memoria 'contador_de_programa +1' y 'contador_de_programa +2'.
    let operandos = [
            self.memoria.leer_memoria(self.contador_de_programa +1),
            self.memoria.leer_memoria(self.contador_de_programa +2),
        ];
        // Devuelve la tupla 'opcode' y 'operandos'.
        (opcode, operandos)
    }
    
    fn ejecuta_instruccion(&mut self, opcode: u8, operandos: [u8; 2]) {
        match opcode {
            0x00 => { // NOP: No hace nada
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("NOP"))); }
                self.contador_de_programa += 1;
            }

            0x04 => { // INR B incrementa el contenido en el Registro (B)
                self.reg_b = self.flags.add(self.reg_b, 0x01, false, false);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR B"))); }
                self.contador_de_programa += 1;
            }

// Corregir
            0x05 => { // DCR B decrementa el contenido en el Registro (B)
                self.reg_b -= 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("DCR B"))); }
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
                self.reg_b = operandos[0];  // self.reg_b = self.memoria.leer_memoria(self.contador_de_programa + 1);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI B,d8"))); }
                self.contador_de_programa += 2;
            }

// Verificar
            0x0A => { // LDAX B cargar el valor contenido en la dirección BC bits en el acumulador (A)
                let direccion = u16::from_le_bytes([self.reg_b, self.reg_c]);
                self.reg_a = self.memoria.leer_memoria(direccion);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("LDAX B"))); }
                self.contador_de_programa += 1;
            }

            0x32 => { // STA addr: carga el registro A en la dirección apuntada por HL
                //self.reg_h = operandos[0];  // self.reg_h = self.memoria.leer_memoria(self.contador_de_programa + 1);
                //self.reg_l = operandos[1];  // self.reg_l = self.memoria.leer_memoria(self.contador_de_programa + 2);
                let direccion = u16::from_le_bytes([operandos[0], operandos[1]]);
                // let direccion = u16::from_be_bytes([self.reg_h, self.reg_l]);
                self.memoria.escribir_memoria(direccion, self.reg_a);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("STA addr"))); }
                self.contador_de_programa += 3;
            },
        
            0x3A => { // LDA addr: carga el valor de la dirección apuntada por los dos siguientes bytes en el acumulador (A)
                let direccion = u16::from_le_bytes([operandos[0], operandos[1]]);
                self.reg_a = self.memoria.leer_memoria(direccion);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("LDA addr"))); }
                self.contador_de_programa += 3;
            },        

            0x3C => { // INR A incrementa el contenido en el Registro (A)
                self.reg_a = self.flags.add(self.reg_a, 0x01, false, false);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("INR A"))); }
                self.contador_de_programa += 1;
            }

// Corregir
            0x3D => { // DCR A decrementa el contenido en el Registro (A)
                self.reg_a -= 1;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("DCR A"))); }
                self.contador_de_programa += 1;
                /*
                Si se decrementa el acumulador en 8080 hasta que llega a 0, esto provocará un desbordamiento (overflow) en el registro, es decir, que el valor almacenado en el acumulador pasará de 0xFF (255 en decimal) a 0x00 (0 en decimal).
                Además, al realizar una operación que decrementa el valor del acumulador, se establecerá la bandera de cero (Z) en 1 si el resultado es 0, o en 0 en caso contrario. También se establecerá la bandera de signo (S) si el bit más significativo del resultado es 1.
                */
            }

            0x3E => { // MVI A,n cargar un valor de 8 bits en el acumulador (A)
                self.reg_a = operandos[0];  // self.reg_a = self.memoria.leer_memoria(self.contador_de_programa + 1);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("MVI A,d8"))); }
                self.contador_de_programa += 2;
            }

            0x80 => { // ADD A,B suma el contenido del Registro B al acumulador (A)
                let resultado_add = self.flags.add(self.reg_a, self.reg_b, true, false);
                self.reg_a = resultado_add;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("ADD A,B"))); }
                self.contador_de_programa += 1;
            }

            0x88 => { // ADC A,B suma el contenido del Registro B + Acarreo operación anterior al acumulador (A)
                let resultado_adc = self.flags.adc(self.reg_a, self.reg_b, false);
                self.reg_a = resultado_adc;
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("ADC A,B"))); }
                self.contador_de_programa += 1;
            }

            0xC3 => { // JMP nn marca PC con la dirección indicada por los dos siguientes bytes
                self.contador_de_programa = u16::from_le_bytes([operandos[0], operandos[1]]);
                unsafe { MNEMONICO_OPCODE = Some(Mutex::new(String::from("JMP nn"))); }
            }

// Revisar *********************************
            _ => { print!("exit"); }
        }
    }

    fn step(&mut self) {
        let instruccion = self.busca_instruccion();
        let (opcode, operandos) = self.decodifica_instruccion(instruccion);
        self.ejecuta_instruccion(opcode, operandos);

        /* (&self).info_registros()
        El paréntesis es necesario para asegurar que se tome la referencia de self antes de llamar al método
        info_registros(). Esto se debe a que el operador . tiene una mayor precedencia que el operador &
        */
        (&self).info_opcode(opcode, operandos);
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
                , self.memoria.leer_memoria(self.contador_de_programa) ));
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
    let mut reg = RegistrosCPU::new;
    //let mut reg = sim_cpu_registros::RegistrosCPU::new;
    let mut cpu = CPU::new();
    let programa = vec![
        0x00,               // NOP
        0x3E, 0x04,         // Almacenar el valor 0x04 en el Registro A
        0x06, 0x0a,         // Almacenar el valor 0x0a en el Registro B
        0x04,               // Incrementa Registro B
        0x80,               // Suma el contenido del Registro B al Registro A
        0x00,               // NOP
        0x3E, 0xf0,         // Almacenar el valor 0xf0 en el Registro A
        0x06, 0x0f,         // Almacenar el valor 0xe0 en el Registro B
        0x80,               // Suma el contenido del Registro B al Registro A
        0x00,               // NOP
        0x3E, 0x3b,         // Almacenar el valor 0x3b en el Registro A
        0x3C,               // Incrementa Registro A
        0x32, 0x15, 0x00,   // Mueve el contenido de A a la dirección indicada por los dos bytes siguientes 
        0x00, 0x00,         // <-- Se cambio el contenido y se convierte en 3C
        0x3A, 0x0b, 0x00,   // Mueve el contenido (0x0f) de la dirección indicada (0x0b) en los dos bytes siguientes a A 
        0x00, 0x00,
        0x00, 0x00,
        0xC3, 0x00, 0x00,   // Salta a la dirección 0x0000
        0xFF, 0xFF,         // Marca fin de programa
    ];
    cpu.cargar_programa(&programa);
    //**************************************
    //cpu.info_pruebas(0000);
    //**************************************

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

        comentarios_window.mvprintw( pos_y + 0, pos_x, format!("Contenido registro A : {:08b}   Contenido registro B : {:08b}", self.reg_a, self.reg_b));
        let byte_menor = self.contador_de_programa as u8;
        let byte_mayor = (self.contador_de_programa >> 8) as u8;

        comentarios_window.mvprintw( pos_y + 1, pos_x, format!("Contador de programas (BigEndian): {:08b}-{:08b}", byte_mayor, byte_menor));

        comentarios_window.mvprintw( pos_y + 3, pos_x, format!("Acarreo {}, Paridad: {}, Acarreo Auxiliar {}, Zero {}, Signo {}"
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

        /*
        comentarios_window.mvprintw(pos_y,  pos_x, format!("", ));
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
fn muestra_mem(comentarios_window: &Window, mut pos_y: i32, mut pos_x: i32, vec: &[u8]) {

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