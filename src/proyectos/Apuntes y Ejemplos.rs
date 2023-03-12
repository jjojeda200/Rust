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
/* demo_union()
pub fn demo_union() {

    let mut z80_reg = Z80Reg { af: 0 };

    // Asignar un valor al registro A
    z80_reg.a = 0x12;
    println!("Registro A: 0x{:02x}", unsafe{z80_reg.a});

    // Asignar un valor al registro F
    z80_reg.f = 0b00101101;
    println!("Registro F: 0x{:02x}", unsafe{z80_reg.f});

    // Asignar un valor al registro AF de 16 bits
    z80_reg.af = 0x1234;
    println!("Registro AF: 0x{:04x}", unsafe{z80_reg.af});

    // Obtener los valores de los registros A y F por separado
    let a = unsafe{z80_reg.a};
    let f = unsafe{z80_reg.f};
    println!("Registro A: 0x{:02x}, Registro F: 0x{:02x}", a, f);

    // Obtener el valor del registro AF de 16 bits por separado
    let af = unsafe{z80_reg.af};
    let a = (af >> 8) as u8;
    let f = (af & 0xFF) as u8;
    println!("Registro A: 0x{:02x}, Registro F: 0x{:02x}", a, f);
}
*/


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
    Para modificar un bit específico en la estructura Flags, se utiliza el método set() o unset(). El método set() establece el bit especificado en true, mientras que el método unset() establece el bit en false. Ambos métodos toman un valor de bit definido en la estructura Flags.
    */
    let mut flags = Flags::from_bits_truncate(flags_byte);
    flags.set(Flags::SUBTRACT, false);

    /*
    Finalmente, se imprime el valor de bits modificado utilizando la función println!() con la misma cadena de formato {:08b}.
    */
    let new_flags_byte = flags.bits();
    println!("Flags: {:08b}", new_flags_byte);
    /*
    La macro bitflags proporciona un conjunto de bits con nombres significativos, y 
    */
}











que proporcionan los métodos from_bits_truncate(), bits(), contains(), set() y unset() 