/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          10-03-2023
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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

#[derive(Debug)]
struct Register8 {
    value: u8,
    ac: u16,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
}

#[allow(dead_code)]
impl Register8 {
    fn new() -> Register8 {
        Register8 {
            value: 0,
            ac: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
        }
    }
    fn load(&mut self, value: u8) {
        self.value = value;
    }
    fn load_ac(&mut self, ac: u16) {
        self.ac = ac;
    }

    fn store(&self) -> u8 {
        self.value
    }

    fn store_ac(&self) -> u16 {
        self.ac
    }
}

impl Register8 {
    fn add(&mut self, value: u8) {
        self.value = self.value.wrapping_add(value);
    }

    fn sub(&mut self, value: u8) {
        self.value = self.value.wrapping_sub(value);
    }

    fn mul(&mut self, value: u8) {
        self.value = self.value.wrapping_mul(value);
    }

    fn div(&mut self, value: u8) {
        if value != 0 {
            self.value = self.value.wrapping_div(value);
        }
    }
}

impl Register8 {
    fn and(&mut self, value: u8) {
        self.value &= value;
    }

    fn or(&mut self, value: u8) {
        self.value |= value;
    }

    fn xor(&mut self, value: u8) {
        self.value ^= value;
    }
}

//*****************************************************************************
pub fn sim_cpu() {
    let titulo = String::from(" Introducción a Simulación CPU ");
    imprime_titulo(&titulo);

    // let mut cpu = Register8{value:0, b:0, c:0, e:0, f:0, ac:0, pc:0};
    let mut cpu = Register8::new();
    cpu.b = 0xf;
    cpu.load_ac(0xff);
    println!("{:?}", cpu);
    println!("{:x}", cpu.ac);

    println!("{:x}", cpu.store_ac());

    // Definir un vector de memoria y cargar una ROM en ella
    let mut memory = vec![0u8; 0xFFFF];
    // TODO: cargar una ROM en el vector de memoria

    // Definir registros del procesador
    let mut pc: u16 = 0x0000;
    let mut sp: u16 = 0xFFFF;

/* Simular ciclos de relo                   
Simular ciclos de reloj del z80
Una parte importante de su funcionamiento es la ejecución de ciclos de reloj, que controlan
el acceso a la memoria y la realización de instrucciones.

Se puede simular la ejecución de ciclos de reloj del Z80 utilizando un bucle que realiza las
siguientes operaciones en cada ciclo:
    Leer la instrucción en la dirección del programa actual (PC).
    Incrementar el contador de programa (PC).
    Decodificar la instrucción.
    Ejecutar la instrucción.
    Repetir.
*/
    loop {    // Iniciar el ciclo de reloj
        // Leer la instrucción en la dirección del programa actual
        let instruction = memory[pc as usize];

        // Incrementar el contador de programa
        pc += 1;

        // Decodificar y ejecutar la instrucción
        match instruction {
            // TODO: implementar todas las instrucciones del Z80
            _ => println!("Instrucción no implementada: {:#X}", instruction),
        }

        // Repetir
    }
}
