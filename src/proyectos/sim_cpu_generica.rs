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

/* 
Método parece ser parte de una implementación de una máquina virtual o un emulador de un procesador. Veamos más en detalle qué hace cada paso del método:

    self.fetch_instruction(): Este método es responsable de obtener la siguiente instrucción del programa que se está ejecutando y devolverla al llamador. En una arquitectura real, esto implicaría leer la instrucción desde la memoria, posiblemente descodificar una dirección de memoria o un puntero de instrucción para obtener la ubicación de la instrucción en la memoria, y luego leer los bytes correspondientes de la memoria para obtener la instrucción real.

    self.decode_instruction(instruction): Una vez que se ha obtenido la instrucción del programa, el siguiente paso es decodificarla para determinar qué operación se debe realizar y qué operandos se deben usar para realizar esa operación. El formato de la instrucción variará según la arquitectura del procesador, pero típicamente incluirá un opcode que indica la operación a realizar y uno o más operandos que indican los datos sobre los que se debe realizar la operación.

    match opcode { ... }: Una vez que se ha decodificado la instrucción, se utiliza el opcode para determinar qué operación debe realizarse. En el código proporcionado, se utiliza una estructura de control match para manejar cada opcode en particular. El código unimplemented!() indica que el comportamiento para este opcode aún no se ha implementado.

En resumen, este método se encarga de ejecutar una única instrucción del programa. Cada instrucción se divide en tres pasos: obtener la instrucción de la memoria, decodificar la instrucción para determinar qué operación se debe realizar y qué operandos se deben utilizar, y finalmente ejecutar la operación correspondiente utilizando los operandos obtenidos.
*/
#![allow(dead_code)]
#![allow(unused_variables)]

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** 
/*
*/
struct CPU {
    memory: [u8; 256],
    registers: [u8; 16],
    program_counter: u8,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: [0; 256],
            registers: [0; 16],
            program_counter: 0,
        }
    }

    fn load_program(&mut self, program: Vec<u8>) {
        for (i, &instruction) in program.iter().enumerate() {
            self.memory[i] = instruction;
        }
    }

    fn fetch_instruction(&mut self) -> u8 {
        let instruction = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        instruction
    }

    fn decode_instruction(&self, instruction: u8) -> (u8, [u8; 2]) {
        let opcode = instruction >> 4;
        let operands = [instruction & 0x0F, self.memory[self.program_counter as usize]];
        (opcode, operands)
    }

    fn execute_instruction(&mut self, opcode: u8, operands: [u8; 2]) {
        match opcode {
            0x0A => self.registers[0] = operands[0],
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
            0xFF => println!("Program finished"),
            _ =>    {
                        print!("- ");
                    }, // unimplemented!(),
        }
    }

    fn step(&mut self) {
        let instruction = self.fetch_instruction();
        let (opcode, operands) = self.decode_instruction(instruction);
        self.execute_instruction(opcode, operands);
    }

    fn run(&mut self) {
        loop {
            self.step();
            println!("Contador: 0x{:02x}, Instrucción: {:02x}", self.program_counter, self.memory[self.program_counter as usize]);
            if self.memory[self.program_counter as usize] == 0xFF {
                break;
            }
        }
    }
}

pub fn cpu_generica_0() {
    let titulo = String::from(" Simulación CPU Genérica 0 ");
    imprime_titulo(&titulo);

    let mut cpu = CPU::new();
    let program = vec![
        0x0A, 0x00, 0x01,  // carga 1 en registro A
        0x0B, 0x00, 0x02,  // carga 2 en registro B
        0x01, 0x00, 0x01,  // suma registro B al registro A
        0x0C, 0x00, 0x06,  // compara registro A con 2
        0x0D, 0x00, 0x06,  // salta a la dirección 6 si la comparación es verdadera
        0x0A, 0x00, 0x03,  // carga 3 en registro A
        0x02, 0x00, 0x02,  // resta registro B del registro A
        0xFF, 0xFF, 0xFF,  // fin del programa
    ];
    cpu.load_program(program);
    cpu.run();

}

//*****************************************************************************




pub fn cpu_generica_1() {
    let titulo = String::from(" Simulación CPU Genérica 1 ");
    imprime_titulo(&titulo);

    let mut cpu = CPU::new();
    let program = vec![
        0x0A, 0x00, 0x01,  // carga 1 en registro A
        0x0B, 0x00, 0x02,  // carga 2 en registro B
        0x01, 0x00, 0x01,  // suma registro B al registro A
        0x0C, 0x00, 0x06,  // compara registro A con 2
        0x0D, 0x00, 0x06,  // salta a la dirección 6 si la comparación es verdadera
        0x0A, 0x00, 0x03,  // carga 3 en registro A
        0x02, 0x00, 0x02,  // resta registro B del registro A
        0xFF, 0xFF, 0xFF,  // fin del programa
    ];
    cpu.load_program(program);
    cpu.run();

}
//***************************************************************************** 