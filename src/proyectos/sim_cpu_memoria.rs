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

/*

*/


//***************************************************************************** 
struct BankedMemory {
    banks: Vec<Vec<u8>>,
    current_bank: usize,
}

impl BankedMemory {
    fn new(num_banks: usize, bank_size: usize) -> BankedMemory {
        BankedMemory {
            banks: vec![vec![0; bank_size]; num_banks],
            current_bank: 0,
        }
    }

    fn select_bank(&mut self, bank_num: usize) {
        self.current_bank = bank_num;
    }

    fn read_byte(&self, addr: u16) -> u8 {
        let bank_offset = (self.current_bank as u16) * 0x4000;
        self.banks[self.current_bank][(addr - bank_offset) as usize]
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let bank_offset = (self.current_bank as u16) * 0x4000;
        self.banks[self.current_bank][(addr - bank_offset) as usize] = val;
    }
}

//***************************************************************************** 
/* 
La estructura Memory tiene un vector mem que representa los primeros 16 KB de memoria, que no son
conmutables, y una estructura banked_mem que representa los 4 bancos de memoria conmutables. Los
métodos read_byte y write_byte verifican si la dirección de memoria solicitada se encuentra dentro
de los primeros 16 KB o en los bancos de memoria conmutables, y llaman a los métodos correspondientes
para leer y escribir en la dirección correcta. El método select_bank llama al método correspondiente
en la estructura banked_mem para seleccionar el banco de memoria deseado.

En fn memoria_0(), se crea una instancia de Memory, se selecciona el banco 0, se escribe un byte en la
dirección 0x1234, se lee el byte de esa misma dirección y se imprime en la consola.
*/
const MEM_SIZE: usize = 0x10000; // Tamaño máximo de la memoria

struct Memory {
    mem: Vec<u8>,
    banked_mem: BankedMemory,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            mem: vec![0; MEM_SIZE],
            banked_mem: BankedMemory::new(4, 0x4000), // 4 bancos de 16 KB
        }
    }

    fn read_byte(&self, addr: u16) -> u8 {
        if addr < 0x4000 {
            self.mem[addr as usize]
        } else {
            self.banked_mem.read_byte(addr)
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if addr < 0x4000 {
            self.mem[addr as usize] = val;
        } else {
            self.banked_mem.write_byte(addr, val);
        }
    }

    fn select_bank(&mut self, bank_num: usize) {
        self.banked_mem.select_bank(bank_num);
    }
}

pub fn memoria_0() {
    let mut mem = Memory::new();

    // Escribir y leer bytes de memoria en el banco 0
    mem.select_bank(0);
    mem.write_byte(0x1234, 0xAB);
    let val = mem.read_byte(0x1234);

    println!("Valor leído: {:08b}", val);
    println!("Valor leído: {:02X}", val);
}


//*****************************************************************************