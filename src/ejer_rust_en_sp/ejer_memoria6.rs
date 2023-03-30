/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          26-03-2023
    Titulo:         introducción a RUST
    Descripción:    Jugando con la memoria, punteros, referencias, etc.
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
// #![allow(unused_variables)]
// #![allow(unused_assignments)]
// #![allow(unused_mut)]

/* función imprime_titulo   
La función imprime_titulo(titulo: &String) recibe como parámetro un puntero a
una cadena de texto String y utiliza la macro println!() para imprimir el valor
de la cadena de texto centrado en 80 caracteres y rodeado por asteriscos.
*/
fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** 
/* Descripción Función:                     

*/
struct Registers {
      a: u8,
      b: u8,
      c: u8,
      d: u8,
      e: u8,
      h: u8,
      l: u8,
  }
  
  impl Registers {
      fn new() -> Self {
          Self {
              a: 0,
              b: 0,
              c: 0,
              d: 0,
              e: 0,
              h: 0,
              l: 0,
          }
      }
  }
  
  struct Simulator {
      registers: Registers,
      memory: [u8; 65536],
  }
  
  impl Simulator {
      fn new() -> Self {
          Self {
              registers: Registers::new(),
              memory: [0; 65536],
          }
      }
  
      fn run(&mut self, opcode: u8) {
          let dest = ((opcode >> 3) & 0b111) as usize;
          println!("Reg. Destino: {:03b}", dest);
          let src = (opcode & 0b111) as usize;
          println!("Reg. Origen : {:03b}", src);
  
          match dest {
              0b000 => self.registers.b = self.get_register_value(src),
              0b001 => self.registers.c = self.get_register_value(src),
              0b010 => self.registers.d = self.get_register_value(src),
              0b011 => self.registers.e = self.get_register_value(src),
              0b100 => self.registers.h = self.get_register_value(src),
              0b101 => self.registers.l = self.get_register_value(src),
              0b110 => {
                  let address = (self.registers.h as u16) << 8 | self.registers.l as u16;
                  self.memory[address as usize] = self.get_register_value(src);
              }
              0b111 => self.registers.a = self.get_register_value(src),
              _ => {}
          }
      }
  
      fn get_register_value(&self, register: usize) -> u8 {
          match register {
              0b000 => self.registers.b,
              0b001 => self.registers.c,
              0b010 => self.registers.d,
              0b011 => self.registers.e,
              0b100 => self.registers.h,
              0b101 => self.registers.l,
              0b110 => {
                  let address = (self.registers.h as u16) << 8 | self.registers.l as u16;
                  self.memory[address as usize]
              }
              0b111 => self.registers.a,
              _ => 0,
          }
      }
  }
    
pub fn memoria_0() {
    let titulo = String::from(" Manejo de instrucciones con bits - Ejemplo para mov ");
    imprime_titulo(&titulo);

    let mut simul = Simulator::new();

    // Ejemplo de uso: Mover el valor 42 al registro B
    let opcode = 0b01000_001;
    simul.registers.c = 42;
    println!("Contenido Registro B: {:02x}", simul.registers.b);
    simul.run(opcode);
    println!("Contenido Registro B: {:02x}", simul.registers.b);
    // Verificar que el valor se movió correctamente
    assert_eq!(simul.registers.b, 42);
    println!("Contenido Registro C: {:02x}", simul.registers.c);
}

//***************************************************************************** 
pub struct BancosMemoria {
    pub segmento_memoria: Vec<Vec<u8>>,
    pub banco_actual: u8,
}

fn imprimir_slices(bancos_memoria: &BancosMemoria) {
    // Obtener el vector interno actual
    let banco = &bancos_memoria.segmento_memoria[bancos_memoria.banco_actual as usize];
    
    // Iterar a través de los slices de 16 bytes
    for slice in banco.chunks_exact(16) {
        // Imprimir cada byte en hexadecimal
        for byte in slice {
            print!("{:02X} ", byte);
        }
        println!();     // Salto de línea después de cada slice
    }
}


/* Descripción Función:                     

*/
pub fn memoria_1() {
    let titulo = String::from(" Pruebas extracción de slice de un vector ");
    imprime_titulo(&titulo);
    
    let mut bancos_memoria = BancosMemoria {
        segmento_memoria: vec![vec![0x00; 64]; 1], // Ejemplo de 4 bancos de 256 bytes cada uno
        banco_actual: 0,
    };
    
    // Llenar el vector con datos de prueba
    for i in 0..bancos_memoria.segmento_memoria[0].len() {
        bancos_memoria.segmento_memoria[0][i] = i as u8;
    }

    imprimir_slices(&bancos_memoria);
}

//*****************************************************************************