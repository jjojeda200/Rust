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
#![allow(dead_code)]
#![allow(unused_variables)]

/* Registro F (banderas)                    
Estas son las banderas que se utilizan en el Z80:
-   Bit 0 (C): Carry flag (bandera de acarreo): Esta bandera se establece en 1 si se
    produce un acarreo de los bits más altos durante una operación aritmética. De lo
    contrario, se establece en 0.
-   Bit 1 (N): Add/Subtract flag (bandera de suma/resta): Esta bandera se establece en
    1 si la última operación realizada fue una resta. De lo contrario, se establece en 0.
-   Bit 2 (PV): Parity/Overflow flag (bandera de paridad/sobreflujo): Esta bandera se
    utiliza para indicar si se ha producido un error en una operación que involucra
    números con signo.
-   Bit 3 (3):  Unused
-   Bit 4 (H): Half-carry flag (bandera de medio acarreo): Esta bandera se establece
    en 1 si se produce un acarreo de los bits más bajos durante una operación aritmética.
    De lo contrario, se establece en 0.
-   Bit 5 (3):  Unused
-   Bit 6 (Z): Zero flag (bandera de cero): Esta bandera se establece en 1 si el resultado
    de una operación es cero. De lo contrario, se establece en 0.
-   Bit 7 (S): Sign flag (bandera de signo): Esta bandera se establece en 1 si el resultado
    de una operación es negativo (el bit más significativo es 1). De lo contrario, se
    establece en 0.
*/

//***************************************************************************** Flags
/* Implementación de Flags                  

// Para crear un struct Flags a partir de un byte:
    let flags_byte: u8 = 0b00101101;            // ejemplo
    let flags = Flags {
        carry: (flags_byte & 0b00000001) != 0,
        subtract: (flags_byte & 0b00000010) != 0,
        parity_overflow: (flags_byte & 0b00000100) != 0,
        half_carry: (flags_byte & 0b00010000) != 0,
        zero: (flags_byte & 0b01000000) != 0,
        sign: (flags_byte & 0b10000000) != 0,
    };

// Para crear un byte a partir de los flags:
    let flags_byte = 
        (flags.carry as u8) |
        ((flags.subtract as u8) << 1) |
        ((flags.parity_overflow as u8) << 2) |
        ((flags.half_carry as u8) << 4) |
        ((flags.zero as u8) << 6) |
        ((flags.sign as u8) << 7);

// Para modificar solo un bit en flags_byte
    let flags_byte: u8 = 0b00101101;            // ejemplo
    let substract_bit_mask: u8 = 0b00000010;    // mascara para el bit de substract

    // Verificar el estado actual del bit de substract
    let substract_bit_is_set = flags_byte & substract_bit_mask != 0;

    // Modificar el bit de substract
    let new_flags_byte = if substract_bit_is_set {
        flags_byte & !substract_bit_mask   // borrar el bit de substract
    } else {
        flags_byte | substract_bit_mask    // establecer el bit de substract
    };

// Actualizar flags_byte con el nuevo valor
    let flags = sim_cpu_struct::Flags {
        carry: (new_flags_byte & 0b00000001) != 0,
        subtract: (new_flags_byte & 0b00000010) != 0,
        parity_overflow: (new_flags_byte & 0b00000100) != 0,
        half_carry: (new_flags_byte & 0b00010000) != 0,
        zero: (new_flags_byte & 0b01000000) != 0,
        sign: (new_flags_byte & 0b10000000) != 0,
    };
*/
#[derive(Default)]
pub struct Flags {  
    pub carry: bool,                    // Bit 0 (C):  Carry flag
    pub subtract: bool,                 // Bit 1 (N):  Add/subtract flag
    pub parity_overflow: bool,          // Bit 2 (PV): Parity/overflow flag
                                        // Bit 3 (3):  Unused
    pub half_carry: bool,               // Bit 4 (H):  Half-carry flag
                                        // Bit 5 (3):  Unused
    pub zero: bool,                     // Bit 6 (Z):  Zero flag
    pub sign: bool,                     // Bit 7 (S):  Sign flag
}

//***************************************************************************** Registros (union)
pub struct Z80Reg {
    a: u8,      // Registro A de 8 bits
    f: u8,      // Registro F de 8 bits
    b: u8,      // Registro B de 8 bits
    c: u8,      // Registro C de 8 bits
    d: u8,      // Registro D de 8 bits
    e: u8,      // Registro E de 8 bits
    h: u8,      // Registro H de 8 bits
    l: u8,      // Registro L de 8 bits
    ix: u16,    // Registro IX de 16 bits
    iy: u16,    // Registro IY de 16 bits
    sp: u16,    // Registro SP de 16 bits
    pc: u16,    // Registro PC de 16 bits
}

impl Z80Reg {
    pub fn new() -> Z80Reg {
        Z80Reg { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, ix: 0, iy: 0, sp: 0, pc: 0,}
    }
    
    pub fn get_a(&self) -> u8 { self.a }
    pub fn set_a(&mut self, value: u8) { self.a = value; }

    pub fn get_f(&self) -> u8 { self.f }
    pub fn set_f(&mut self, value: u8) { self.f = value; }

    pub fn get_b(&self) -> u8 { self.b }
    pub fn set_b(&mut self, value: u8) { self.b = value; }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn set_c(&mut self, value: u8) { self.c = value; }
    pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.b, self.c]) }
    fn set_bc(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
        /*
        Este método permite acceder al registro BC como un valor de 16 bits, pero solo permite
        la escritura de los 4 bits más significativos del registro:
        self.c = bytes[1] & 0xF0;
        */
    }

}

