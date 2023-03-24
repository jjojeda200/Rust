/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-03-2023
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

//***************************************************************************** Estructura e implementación Flags
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

impl Flags {
    pub fn new_flags() -> Flags {
        Flags {
            carry: false,
            subtract: false,
            parity_overflow: false,
            half_carry: false,
            zero: false,
            sign: false,
        }
    }

    // Método que devuelva un valor de 8 bits que represente los Flags (bitwise con Hex):
    pub fn get_flags(&self) -> u8 {
        let mut resultado = 0u8;
        if self.carry { resultado |= 0x01 };
        if self.subtract { resultado |= 0x02 };
        if self.parity_overflow { resultado |= 0x04 };
        if self.half_carry { resultado |= 0x10 };
        if self.zero { resultado |= 0x40 };
        if self.sign { resultado |= 0x80 };
        resultado
    }

    // Método que devuelva un valor de 8 bits que represente los Flags (bitwise con bin):
    pub fn get_flags_b(&self) -> u8 {
        let mut resultado = 0u8;
        if self.carry { resultado |= 0b00000001 };
        if self.subtract { resultado |= 0b00000010 };
        if self.parity_overflow { resultado |= 0b00000100 };
        if self.half_carry { resultado |= 0b00010000 };
        if self.zero { resultado |= 0b01000000 };
        if self.sign { resultado |= 0b10000000 };
        resultado
    }

    // Método que toma un valor de 8 bits y actualiza los bits de los Flags :
    pub fn set_flags(&mut self, valor: u8) {
        self.carry = valor & 0x01 != 0;             // Bit 0
        self.subtract = valor & 0x02 != 0;          // Bit 1
        self.parity_overflow = valor & 0x04 != 0;   // Bit 2
        self.half_carry = valor & 0x10 != 0;        // Bit 4
        self.zero = valor & 0x40 != 0;              // Bit 6
        /* Operación bitwise                
        "Valor" es un valor de 8 bits (u8) y 0x80 es un valor hexadecimal que representa el
        número 128 en decimal y su representación binaria es 10000000.
        La operación AND se realiza bit a bit, lo que significa que cada bit del valor "valor" se
        compara con el correspondiente bit de 0x80 y se devuelve un resultado que tiene un bit 1
        solo si ambos bits son 1. En caso contrario, el resultado tendrá un bit 0.
        */
        self.sign = valor & 0x80 != 0;              // Bit 7
    }

    pub fn set_bit(&mut self, index: u8, valor: bool) {
        match index {
            0 => self.carry = valor,
            1 => self.subtract = valor,
            2 => self.parity_overflow = valor,
            4 => self.half_carry = valor,
            6 => self.zero = valor,
            7 => self.sign = valor,
            _ => panic!("Índice de bit no válido: {}", index),
        }
    }

}

//***************************************************************************** Estructura e implementación Registros
pub struct RegistrosCPU {
    a: u8,      // Registro A de 8 bits
    //f: u8,    // Manejamos el registro de flags F de manera independiente
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

impl RegistrosCPU {
    pub fn new() -> RegistrosCPU {
        RegistrosCPU { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, ix: 0, iy: 0, sp: 0, pc: 0,}
    }

//************************************* Manejo de Registro
    pub fn get_a(&self) -> u8 { self.a }
    pub fn set_a(&mut self, valor: u8) { self.a = valor; }

    pub fn get_b(&self) -> u8 { self.b }
    pub fn set_b(&mut self, valor: u8) { self.b = valor; }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn set_c(&mut self, valor: u8) { self.c = valor; }
    pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.b, self.c]) }
    pub fn set_bc(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
        /*
        Este método permite acceder al registro BC como un valor de 16 bits, pero solo permite
        la escritura de los 4 bits más significativos del registro:
        self.c = bytes[1] & 0xF0;
        */
    }
    //******************************** Otra forma de manejar BC
    pub fn get_reg_bc(&self) -> u16 {
        u16::from_be_bytes([self.get_b(), self.get_c()])
    }
    pub fn set_reg_bc(&mut self, valor: u16) {
        let [b, c] = valor.to_be_bytes();
        self.set_b(b);
        self.set_c(c);
    }

    pub fn get_d(&self) -> u8 { self.d }
    pub fn set_d(&mut self, valor: u8) { self.d = valor; }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn set_e(&mut self, valor: u8) { self.e = valor; }
    pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.d, self.d]) }
    pub fn set_de(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }

    pub fn get_h(&self) -> u8 { self.h }
    pub fn set_h(&mut self, valor: u8) { self.h = valor; }
    pub fn get_l(&self) -> u8 { self.c }
    pub fn set_l(&mut self, valor: u8) { self.l = valor; }
    pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.h, self.l]) }
    pub fn set_hl(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.h = bytes[0];
        self.e = bytes[1];
    }


}

//*****************************************************************************
/* Implementación prueba de Flags           
Ejemplo inicial de utilización de la struct "pub struct Flags")

pub fn flags_0() {
// Para inicializar estado de los Flags a partir de un byte:
    let mut flags_byte: u8 = 0b00000111;
    let mut flags = sim_cpu_struct::Flags {
        carry: (flags_byte & 0b00000001) != 0,
        subtract: (flags_byte & 0b00000010) != 0,
        parity_overflow: (flags_byte & 0b00000100) != 0,
        half_carry: (flags_byte & 0b00010000) != 0,
        zero: (flags_byte & 0b01000000) != 0,
        sign: (flags_byte & 0b10000000) != 0,
    };
    println!("Flags: inicializado = {:08b}", flags_byte);

// Para modificar solo un bit de los flags (ejemplo subtract)
    // Máscara para el bit de substract
    let substract_bit_mask: u8 = 0b00000010;

    // Verificar el estado actual del bit de substract en la variable de flags
    let substract_bit_is_set = flags_byte & substract_bit_mask != 0;
    println!("Flags: Substract bit 1 (N) = {}", substract_bit_is_set);

    // Modificar el bit de substract
    flags_byte = if substract_bit_is_set {
        flags_byte & !substract_bit_mask   // Borrar el bit de substract
    } else {
        flags_byte | substract_bit_mask    // Establecer el bit de substract
    };
    let substract_bit_is_set = flags_byte & substract_bit_mask != 0;
    println!("Flags: Substract bit 1 (N) = {}", substract_bit_is_set);
    //println!("Flags: Estado actual de la variable de flags = {:08b}", flags_byte);

// Actualizar flags con el nuevo valor de flags_byte
    flags = sim_cpu_struct::Flags {
        carry: (flags_byte & 0b00000001) != 0,
        subtract: (flags_byte & 0b00000010) != 0,
        parity_overflow: (flags_byte & 0b00000100) != 0,
        half_carry: (flags_byte & 0b00010000) != 0,
        zero: (flags_byte & 0b01000000) != 0,
        sign: (flags_byte & 0b10000000) != 0,
    };

// Para crear un byte a partir de los flags:
    flags_byte = 
        (flags.carry as u8) |
        ((flags.subtract as u8) << 1) |
        ((flags.parity_overflow as u8) << 2) |
        ((flags.half_carry as u8) << 4) |
        ((flags.zero as u8) << 6) |
        ((flags.sign as u8) << 7);
    println!("Flags: Estado actual de la variable de flags = {:08b}", flags_byte);
}
*/

//*****************************************************************************