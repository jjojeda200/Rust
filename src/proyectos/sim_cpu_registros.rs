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
    ** Bit 1 no se usa en el Intel 8080 **
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
    pub subtract: bool,                 // Bit 1 (N):  Add/subtract flag (En el Intel 8080 no se usa, siempre a uno)
    pub parity_overflow: bool,          // Bit 2 (PV): Parity/overflow flag
                                        // Bit 3 (3):  Unused (Siempre a cero)
    pub half_carry: bool,               // Bit 4 (H):  Half-carry flag
                                        // Bit 5 (3):  Unused (Siempre a cero)
    pub zero: bool,                     // Bit 6 (Z):  Zero flag
    pub sign: bool,                     // Bit 7 (S):  Sign flag
}

impl Flags {
    pub fn new_flags() -> Flags {
        Flags {
            carry: false,
            subtract: true,
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
    pub fn get_flags_1(&self) -> u8 {
        let mut resultado = 0u8;
        if self.carry { resultado |= 0b00000001 };
        if self.subtract { resultado |= 0b00000010 };
        if self.parity_overflow { resultado |= 0b00000100 };
        if self.half_carry { resultado |= 0b00010000 };
        if self.zero { resultado |= 0b01000000 };
        if self.sign { resultado |= 0b10000000 };
        resultado
    }

    // Método que toma un valor de 8 bits y actualiza los bits de los Flags (bitwise con Hex):
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

    pub fn get_bit(&mut self, index: u8) -> u8 {
        match index {
            0 => {self.carry as u8},
            1 => {self.subtract as u8},
            2 => {self.parity_overflow as u8},
            4 => {self.half_carry as u8},
            6 => {self.zero as u8},
            7 => {self.sign as u8},
            _ => panic!("Índice de bit no válido: {}", index),
        }
    }

    pub fn get_bit_1(&self, bit: u8) -> bool {
        (self.get_flags() & (1 << bit)) != 0
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
    reg_a: u8,      // Registro A de 8 bits
    //f: u8,    // Manejamos el registro de flags F de manera independiente
    reg_b: u8,      // Registro B de 8 bits
    reg_c: u8,      // Registro C de 8 bits
    reg_d: u8,      // Registro D de 8 bits
    reg_e: u8,      // Registro E de 8 bits
    reg_h: u8,      // Registro H de 8 bits
    reg_l: u8,      // Registro L de 8 bits
    reg_ix: u16,    // Registro IX de 16 bits
    reg_iy: u16,    // Registro IY de 16 bits
    sp: u16,    // Registro SP de 16 bits
    pc: u16,    // Registro PC de 16 bits
}

impl RegistrosCPU {
    pub fn new() -> RegistrosCPU {
        RegistrosCPU {
            reg_a: 0, 
            reg_b: 0, 
            reg_c: 0, 
            reg_d: 0, 
            reg_e: 0, 
            reg_h: 0, 
            reg_l: 0, 
            reg_ix: 0, 
            reg_iy: 0, 
            sp: 0, 
            pc: 0,
        }
    }

//************************************* Manejo de Registro
    pub fn get_a(&self) -> u8 { self.reg_a }
    pub fn set_a(&mut self, valor: u8) { self.reg_a = valor; }

    pub fn get_b(&self) -> u8 { self.reg_b }
    pub fn set_b(&mut self, valor: u8) { self.reg_b = valor; }
    pub fn get_c(&self) -> u8 { self.reg_c }
    pub fn set_c(&mut self, valor: u8) { self.reg_c = valor; }
    pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.reg_b, self.reg_c]) }
    pub fn set_bc(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.reg_b = bytes[0];
        self.reg_c = bytes[1];
        /*
        Este método permite acceder al registro BC como un valor de 16 bits, pero solo permite
        la escritura de los 4 bits más significativos del registro:
        self.reg_c = bytes[1] & 0xF0;
        */
    }
    //******************************** Otra forma de manejar BC
    pub fn get_reg_bc(&self) -> u16 {
        u16::from_be_bytes([self.get_b(), self.get_c()])
    }
    pub fn set_reg_bc(&mut self, valor: u16) {
        let [reg_b, reg_c] = valor.to_be_bytes();
        self.set_b(reg_b);
        self.set_c(reg_c);
    }

    pub fn get_d(&self) -> u8 { self.reg_d }
    pub fn set_d(&mut self, valor: u8) { self.reg_d = valor; }
    pub fn get_e(&self) -> u8 { self.reg_e }
    pub fn set_e(&mut self, valor: u8) { self.reg_e = valor; }
    pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.reg_d, self.reg_e]) }
    pub fn set_de(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.reg_d = bytes[0];
        self.reg_e = bytes[1];
    }

    pub fn get_h(&self) -> u8 { self.reg_h }
    pub fn set_h(&mut self, valor: u8) { self.reg_h = valor; }
    pub fn get_l(&self) -> u8 { self.reg_c }
    pub fn set_l(&mut self, valor: u8) { self.reg_l = valor; }
    pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.reg_h, self.reg_l]) }
    pub fn set_hl(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.reg_h = bytes[0];
        self.reg_l = bytes[1];
    }


}

//*****************************************************************************
// Manejo de bit en un byte con operaciones lógicas y desplazamientos
/*
pub fn get0(){
    /*
    La expresión (1<<0) es una operación de desplazamiento a la izquierda de un bit, que se utiliza para mover
    el valor binario 1 cero posiciones a la izquierda.
    */
    println!("Creación de un byte activando el bit 0: {:08b}",(1<<0));
    println!("Creación de un byte activando el bit 1: {:08b}",(1<<1));
    println!("Creación de un byte activando el bit 7: {:08b}",(1<<7));
    println!("máscara de bits {:08b}",(0x00 & (1<<1)));
    println!("máscara de bits {:08b}",(0xff & (1<<1)));
    let mut num = 0x00;
    println!("Esta activo el bit 1? con & (1<<1 : {}",(num & (1 << 1)) != 0);
    num = 0xff;
    println!("Esta activo el bit 1? con & (1<<1 : {}",(num & (1 << 1)) != 0);
}

pub fn get(n: u8, b: usize) -> bool {
    (n & (1 << b)) != 0
}
pub fn set(n: u8, b: usize) -> u8 {
    n | (1 << b)
}
pub fn reset(n: u8, b: usize) -> u8 {
    n & !(1 << b)
}

pub fn pruebas_mascara_bits() {
    get0();
    
    let mut n: u8 = 0b00001111;         // n = 15 en binario
    println!("El bit en la posición 3 es {}", get(n, 3));   // imprime true
    n = set(n, 5);                      // establece el bit en la posición 5 en 1
    println!("n ahora es {:08b}", n);   // imprime "n ahora es 0b00101111"
    n = reset(n, 0);                    // resetea el bit en la posición 0 a 0
    println!("n ahora es {:08b}", n);   // imprime "n ahora es 0b00101110"  

    let mut n = 0b1101;             // El número binario 1101 es el número decimal 13
    assert_eq!(get(n, 0), true);        // El bit menos significativo es 1
    assert_eq!(get(n, 1), false);       // El siguiente bit es 0
    n = set(n, 1);
    assert_eq!(get(n, 1), true);        // Ahora el segundo bit es 1
    n = reset(n, 3);
    assert_eq!(get(n, 3), false);       // Ahora el cuarto bit es 0
    assert_eq!(n, 0b0111);              // El número binario 0101 es el número decimal 5
}
*/