/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          12-03-2023
    Titulo:         Simulación CPU - Apuntes y Ejemplos
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



//*****************************************************************************
/*
Estas macros se puede utilizar para definir un conjunto de bits con nombres significativos en
lugar de trabajar con bits en bruto. La macro toma dos argumentos: el nombre de la estructura
y el tipo de bits que se utilizará para almacenar los valores. En este caso, estamos utilizando
un tipo u8.
*/

bitflags! {
    struct Flags: u8 {
        const CARRY = 0b00000001;
        const SUBTRACT = 0b00000010;
        const PARITY_OVERFLOW = 0b00000100;
        const HALF_CARRY = 0b00010000;
        const ZERO = 0b01000000;
        const SIGN = 0b10000000;
    }
}

pub fn apuntes_ejemplos0(){
    /*
    A continuación, se define una variable flags_byte que contiene un byte de ejemplo con valores
    de bits establecidos.
    */
    let flags_byte: u8 = 0b00101101;

    /*
    Luego, se crea una instancia de la estructura Flags utilizando el método from_bits_truncate().
    Este método toma un valor de bits y devuelve una instancia de la estructura Flags con los bits
    establecidos de acuerdo con el valor de entrada. El método from_bits_truncate() ignora cualquier
    valor de bits que no está definido en la estructura Flags.
    */
    let flags = Flags::from_bits_truncate(flags_byte);

    /*
    Para imprimir los valores de bits de flags_byte, se utiliza la función println!() con una cadena
    de formato {:08b}. La cadena de formato {:08b} indica que se imprimirá un número binario de 8
    bits con ceros iniciales.
    */
    println!("Flags: {:08b}", flags_byte);

    /*
    Para crear un byte a partir de los bits de la estructura Flags, se utiliza el método bits().
    Este método devuelve un valor de bits que representa la estructura Flags.
    */
    let flags_byte = flags.bits();

    /*
    Para verificar si un bit específico está activado en el byte, se utiliza el método contains().
    Este método toma un valor de bit definido en la estructura Flags y devuelve un valor booleano
    que indica si el bit está establecido en la instancia Flags.
    */
    let substract_bit_is_set = flags.contains(Flags::SUBTRACT);

    /*
    Para modificar un bit específico en la estructura Flags, se utiliza el método set() o unset().
    El método set() establece el bit especificado en true, mientras que el método unset() establece
    el bit en false. Ambos métodos toman un valor de bit definido en la estructura Flags.
    */
    let mut flags = Flags::from_bits_truncate(flags_byte);
    flags.set(Flags::SUBTRACT, false);

    /*
    Finalmente, se imprime el valor de bits modificado utilizando la función println!() con la misma
    cadena de formato {:08b}.
    */
    let new_flags_byte = flags.bits();
    println!("Flags: {:08b}", new_flags_byte);

    /*
    La macro bitflags proporciona un conjunto de bits con nombres significativos. 
    Los métodos from_bits_truncate(), bits(), contains(), set() y unset() son parte de la API de Rust
    para el tipo de datos EnumSet. Estos métodos permiten manipular y trabajar con conjuntos de
    valores de una enumeración de manera eficiente.

    A continuación se explica brevemente cada uno de estos métodos:

    -   El método from_bits_truncate() permite construir un conjunto de valores de una enumeración a
        partir de un valor entero que representa los bits que se corresponden con los elementos del
        conjunto. Este método descarta cualquier bit que no corresponda a un valor de la enumeración.

    -   El método bits() devuelve el valor entero que representa los bits que se corresponden con los
        elementos del conjunto.

    -   El método contains() verifica si un elemento está presente en el conjunto.

    -   El método set() agrega un elemento al conjunto.

    -   El método unset() remueve un elemento del conjunto.
    */
}


//*****************************************************************************
/*
    from_byte: una función que toma un byte y devuelve una estructura Flags.
    to_byte: una función que toma una estructura Flags y devuelve un byte.
    modify_bit: una función que toma una referencia mutable a una estructura Flags, un valor de máscara de bit
    y un valor booleano, y modifica el bit correspondiente en la estructura de acuerdo con la máscara de bit y
    el valor booleano.
*/

pub trait FlagsOperations {
    fn from_byte(byte: u8) -> Flags;
    fn to_byte(&self) -> u8;
    fn modify_bit(&mut self, bit_mask: u8, value: bool);
}

impl FlagsOperations for Flags {
    fn from_byte(byte: u8) -> Flags {
        Flags {
            carry: (byte & 0b00000001) != 0,
            subtract: (byte & 0b00000010) != 0,
            parity_overflow: (byte & 0b00000100) != 0,
            half_carry: (byte & 0b00010000) != 0,
            zero: (byte & 0b01000000) != 0,
            sign: (byte & 0b10000000) != 0,
        }
    }

    fn to_byte(&self) -> u8 {
        let mut byte: u8 = 0;
        if self.carry {
            byte |= 0b00000001;
        }
        if self.subtract {
            byte |= 0b00000010;
        }
        if self.parity_overflow {
            byte |= 0b00000100;
        }
        if self.half_carry {
            byte |= 0b00010000;
        }
        if self.zero {
            byte |= 0b01000000;
        }
        if self.sign {
            byte |= 0b10000000;
        }
        byte
    }

    fn modify_bit(&mut self, bit_mask: u8, value: bool) {
        match bit_mask {
            0b00000001 => self.carry = value,
            0b00000010 => self.subtract = value,
            0b00000100 => self.parity_overflow = value,
            0b00010000 => self.half_carry = value,
            0b01000000 => self.zero = value,
            0b10000000 => self.sign = value,
            _ => (),
        }
    }
}

pub fn manxxxx() {
    let byte = 0b10101010;
    let mut flags = Flags::from_byte(byte);
    //println!("Flags before modification: {}", flags);

    flags.modify_bit(0b00000001, true); // Set the carry flag to true
    flags.modify_bit(0b01000000, false); // Set the zero flag to false

    let modified_byte = flags.to_byte();
    //println!("Modified byte: {:08b}", modified_byte);
    //println!("Flags after modification: {:08b}", flags);
}