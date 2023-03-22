/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          19-03-2023
    Titulo:         Simulación CPU Generica
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

#![allow(dead_code)]
#![allow(unused_variables)]

use pancurses::*;

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
    //ventana.printw(&format!("\n{:*^1$}", titulo, max_x as usize));
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
struct CPU {
    memory: [u8; 256],
    registers: [u8; 8],
    program_counter: u8,
    registro_instrucciones: u8,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: [0; 256],
            registers: [0; 8],
            program_counter: 0,
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
        // Obtener la instrucción de la memoria en la dirección del contador de programa
        let instruction = self.memory[self.program_counter as usize];
        /* Manejo Little Endian             
        let instruction = u16::from_le_bytes([self.memory[self.program_counter], self.memory[self.program_counter + 1]]);
        */


        // Incrementar el contador de programa para apuntar a la siguiente instrucción
        self.program_counter += 1;

        // Devolver la instrucción
        instruction
    }

    fn decode_instruction(&self, instruction: u8) -> (u8, [u8; 2]) {
        let opcode = instruction ;//>> 4;
        let operands = [instruction & 0x0F, self.memory[self.program_counter as usize]];
        (opcode, operands)
    }

/* Definición de los OPCODES CPU-8080 utilizados:    
    0x00:
    0x0A:
    0x3E:

    0x0B: Almacenar el operando en el registro 1
    0x01: Sumar los valores de los registros 0 y 1 y almacenar el resultado en el registro 0
    0x0C: Si el valor en el registro 0 es igual al primer operando, establecer el contador del programa en el segundo operando
    0x0D: Establecer el contador del programa en el primer operando
    0x02: Restar el valor del registro 1 del valor del registro 0 y almacenar el resultado en el registro 0
    0xFF: Imprimir "Program finished" en la consola.
*/
    fn execute_instruction(&mut self, opcode: u8, operands: [u8; 2]) {
        let titulo_ventana_opcode = String::from(" OP Codes ");
        let pos_x = 60;
        let opcode_window = newwin(10, 16, 10, pos_x);
        opcode_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&opcode_window, &titulo_ventana_opcode);
        let pos_y = opcode_window.get_cur_y();
        //opcode_window.mv(2, 2);
        match opcode {
            0x00 => { // NOP: No hace nada
                opcode_window.mvprintw(2, 2,format!("NOP"));
                opcode_window.mvprintw(3, 2,format!("Hex: 0x00"));
                opcode_window.mvprintw(4, 2,format!("PC : {:04x}", self.program_counter));
                //self.program_counter += 1;
            },

            0x0A => { // LDAX A,(BC) cargar el valor contenido en la dirección BC bits en el acumulador (A)
                opcode_window.mvprintw(2, 2,format!("MOV A,[BC]"));
                opcode_window.mvprintw(3, 2,format!("Hex: 0x0A"));
                opcode_window.mvprintw(4, 2,format!("PC : {:02x}", self.program_counter));
                self.registers[0] = operands[0];
                opcode_window.printw(format!("0x{:02x}", self.registers[0]));
                opcode_window.refresh();
                },

            0x3E => { // MOV r,n cargar un valor de 8 bits en el acumulador (A)
                opcode_window.mvprintw(2, 2,format!("MOV A,dato"));
                opcode_window.mvprintw(3, 2,format!("Hex: 0x{:02x}", opcode));
                opcode_window.mvprintw(4, 2,format!("PC : {:04x}", self.program_counter));
                self.registers[0] = self.memory[self.program_counter as usize];
                opcode_window.mvprintw(5, 2,format!("A  : {:02x}", self.registers[0]));
                opcode_window.refresh();
                self.program_counter += 1;
                },

            0xC3 => { // JMP nn marca PC con la dirección indicada por los dos siguientes bytes 
                opcode_window.mvprintw(2, 2,format!("JMP direcc."));
                opcode_window.mvprintw(3, 2,format!("Hex: 0x{:02x}", opcode));
                opcode_window.mvprintw(4, 2,format!("PC : {:04x}", self.program_counter));
                self.program_counter = self.memory[self.program_counter as usize];
                //self.registers[0] = self.memory[self.program_counter as usize];
                opcode_window.mvprintw(4, 2,format!("PC : {:04x}", self.program_counter));
                opcode_window.refresh();
                },

/* 
            0x0B => self.registers[1] = operands[0],
            0x01 => self.registers[0] += self.registers[1],
            0x0C => {
                if self.registers[0] == operands[0] {
                    self.program_counter = operands[1];
                }
            },
            0x0D => {
                self.program_counter = operands[0];
            },
            0x02 => self.registers[0] -= self.registers[1],
*/
            0xFF => println!("Fin del programa"),
            _ =>    {
                        print!("");
                    }, // unimplemented!(),

        }
        opcode_window.refresh();
    }

    fn step(&mut self) {
        let instruction = self.fetch_instruction();
        let (opcode, operands) = self.decode_instruction(instruction);
        //println!(" {:02x}", instruction);
        self.execute_instruction(opcode, operands);
    }

    fn run(&mut self, window: &Window) {
        //************************************** Ventana para mostrar los registros
        let titulo_ventana_reg = String::from(" Registros ");
        let max_x = window.get_max_x();
        let reg_window = newwin(10, 16, 0, max_x);
        reg_window.border('|', '|', '-', '-', '+', '+', '+', '+');
        imprime_titulo(&reg_window, &titulo_ventana_reg);
        reg_window.refresh();

        let mut pos_y = 4;
        loop {
            window.mv(pos_y, 2);
            window.printw(format!("Contador: 0x{:02x}, Instruccion: {:02x}", self.program_counter, self.memory[self.program_counter as usize]));
            window.printw(format!(" Reg A: {:02x}, Reg B: {:02x}", self.registers[0], self.registers[1]));
            pos_y += 1;
            if pos_y == 29 {pos_y = 4;}
            self.step();
            window.getch();
            
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

    start_color();
    ventana_principal.refresh();

    imprime_titulo(&ventana_principal, &titulo);

    //**************************************
    let mut cpu = CPU::new();
    let program = vec![
        0x00,               // NOP
        0x3E, 0x04,         // Almacenar el valor 4 en el registro 0 (A)
        0x00,               // NOP
        0xC3, 0x00,         // Salta a la dirección 00 (modificar para direccionamiento de 2 bytes)
/*
        0x01,
        0x0B, 0x02, 0x02,   // Almacenar el valor 2 en el registro 1 (B)
        0x01, 0x00, 0x01,   // Sumar los valores de los registros 0 y 1 y almacenar el resultado en el registro 0
                            // el valor en el registro 0 ahora es 3 (suma registro B al registro A)
        0x0C, 0x00, 0x06,   // Si el valor en el registro 0 es igual a 0, saltar a la posición de memoria 6
                            // no se cumplió la condición, por lo que se continúa con la siguiente instrucción
        0x0D, 0x00, 0x06,   // Establecer el contador del programa en la posición de memoria 6
        0x3E, 0x00, 0x03,   // Almacenar el valor 3 en el registro 0 (A)
        0x02, 0x00, 0x02,   // Restar el valor en el registro 1 (B) del valor en el registro 0 (A) y almacenar el
                            // resultado en el registro 0 (el valor en el registro 0 ahora es 0)
        0xFF, 0xFF, 0xFF,    // Imprimir "Fin del programa"
*/
    ];
    cpu.load_program(program);
    cpu.run(&ventana_principal);

    ventana_principal.getch();

    //**************************************
    endwin();
}

//*****************************************************************************
pub fn cpu_generica_1() {
    let titulo = String::from(" Simulacion CPU Generica 1 ");
    let ventana_principal = initscr();

    ventana_principal.border('|', '|', '-', '-', '+', '+', '+', '+');

    start_color();
    // Definición de combinación de colores
    init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_GREEN, COLOR_YELLOW);
    init_pair(4, COLOR_WHITE, COLOR_BLACK);
    init_pair(5, COLOR_WHITE, COLOR_BLUE);
    ventana_principal.bkgd(COLOR_PAIR(2));
    
    ventana_principal.attrset(ColorPair(1));
    imprime_titulo(&ventana_principal, &titulo);
    ventana_principal.attrset(Attribute::Normal);

    ventana_principal.getch();   
    endwin();
}

//***************************************************************************** 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_program() {
        let mut cpu = CPU::new();
        let program = vec![0x01, 0x02, 0x03, 0x04];
        cpu.load_program(program);

        assert_eq!(cpu.memory[0], 0x01);
        assert_eq!(cpu.memory[1], 0x02);
        assert_eq!(cpu.memory[2], 0x03);
        assert_eq!(cpu.memory[3], 0x04);
}

    #[test]
    fn test_fetch_instruction() {
        let mut cpu = CPU::new();
        let program = vec![0x00, 0x3E, 0xFF];
        cpu.load_program(program);

        let instruction = cpu.fetch_instruction();
        assert_eq!(instruction, 0x00);

        let instruction = cpu.fetch_instruction();
        assert_eq!(instruction, 0x3E);

        let instruction = cpu.fetch_instruction();
        assert_eq!(instruction, 0xFF);
    }

    #[test]
    fn test_decode_instruction() {
        let mut cpu = CPU::new();
        cpu.memory[0] = 0xAB;
        cpu.memory[1] = 0xCD;

        let instruction = cpu.fetch_instruction();
        let decoded = cpu.decode_instruction(instruction);

        assert_eq!(decoded.0, 0x0A);
        assert_eq!(decoded.1, [0xB, 0xCD]);
    }

    #[test]
    fn test_execute_instruction() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 42;
        cpu.registers[1] = 23;

        // Ejecuta una instrucción que suma los valores en los registros 0 y 1
        cpu.execute_instruction(0x01, [0, 0]);

        // Verifica que el valor en el registro 0 se haya actualizado correctamente
        assert_eq!(cpu.registers[0], 65);
    }
/*
    0x0A: Almacenar el operando en el registro 0
    0x0B: Almacenar el operando en el registro 1
    0x01: Sumar los valores de los registros 0 y 1 y almacenar el resultado en el registro 0
    0x0C: Si el valor en el registro 0 es igual al primer operando, establecer el contador del programa en el segundo operando
    0x0D: Establecer el contador del programa en el primer operando
    0x02: Restar el valor del registro 1 del valor del registro 0 y almacenar el resultado en el registro 0
    0xFF: Imprimir "Program finished" en la consola.
*/
    #[test]
    fn test_step() {
        let mut cpu = CPU::new();
        let program = vec![0x0A, 0x05, 0x01, 0x0C, 0x05, 0x06, 0xFF];
        cpu.load_program(program);

        // Primer paso
        cpu.execute_instruction(0x0A, [0x05, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 1);
        assert_eq!(cpu.registers[0], 5);

        // Segundo paso
        cpu.execute_instruction(0x05, [0x01, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 2);
        assert_eq!(cpu.registers[0], 5);

        // Tercer paso
        cpu.execute_instruction(0x01, [0x00, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.registers[0], 5);
        assert_eq!(cpu.registers[1], 0);
/*
        // Cuarto paso
        cpu.execute_instruction(0x0C, [0x05, 0x04]);
        cpu.step();
        assert_eq!(cpu.program_counter, 4);
        assert_eq!(cpu.registers[0], 5);
        assert_eq!(cpu.registers[1], 0);

        // Quinto paso
        cpu.execute_instruction(0x05, [0x06, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 5);
        assert_eq!(cpu.registers[0], 30);
        assert_eq!(cpu.registers[1], 0);

        // Sexto paso
        cpu.execute_instruction(0x06, [0x05, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 6);
        assert_eq!(cpu.registers[0], 6);
        assert_eq!(cpu.registers[1], 0);

        // Séptimo paso
        cpu.execute_instruction(0xFF, [0x00, 0x00]);
        cpu.step();
        assert_eq!(cpu.program_counter, 6); */
    }

}
