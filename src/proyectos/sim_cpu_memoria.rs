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

//***************************************************************************** Módulo de emulación
/* Descripción                              
El código define una estructura llamada "BankedMemory" que representa una memoria dividida en
bancos. Cada banco es un vector de bytes (u8) de tamaño fijo y la estructura mantiene una pista
del banco actualmente seleccionado.

El método "new" es un constructor que toma como argumentos el número de bancos y el tamaño de
cada banco, y crea una instancia de "BankedMemory" inicializando la estructura con un vector de
"num_banks" bancos, cada uno con un vector de "bank_size" bytes inicializados a cero.

El método "select_bank" permite cambiar el banco actualmente seleccionado a uno de los bancos
existentes, identificado por su número de índice.

Los métodos "read_byte" y "write_byte" permiten leer y escribir bytes en la memoria en la dirección
de memoria dada por "addr". En ambos métodos, se calcula primero el desplazamiento dentro del banco
actual en base a la dirección de memoria y el tamaño del banco. Luego, se accede al vector de bytes
correspondiente al banco actual y se lee o escribe el byte en la posición indicada por el
desplazamiento.
*/

pub struct BancosMemoria {
    pub segmento_memoria: Vec<Vec<u8>>,
    pub banco_actual: u8,
}

pub struct UnidadMemoria {
    data: Vec<u8>,                                  // Datos de la unidad de memoria
    start_address: u16,                             // Dirección de inicio de la unidad de memoria
    end_address: u16,                               // Dirección final de la unidad de memoria
    read_handler: Option<Box<dyn Fn(u16) -> u8>>,   // Función para leer datos de la unidad de memoria
    write_handler: Option<Box<dyn Fn(u16, u8)>>,    // Función para escribir datos en la unidad de memoria
}

impl BancosMemoria {
    pub fn new() -> BancosMemoria {
        BancosMemoria {
            segmento_memoria: vec![vec![0; 16384]; 1],
            banco_actual: 0,
        }
    }

    pub fn asignar_segmento(&mut self, longitud_del_segmento: usize) {
        self.segmento_memoria.push(vec![0; longitud_del_segmento]);
    }

    pub fn eliminar_segmento(&mut self, num_de_banco: usize) -> Result<(), String> {
        if num_de_banco >= self.segmento_memoria.len() {
            return Err(String::from("El banco especificado no existe."));
        }
        if self.segmento_memoria.len() == 1 {
            return Err(String::from("No se puede eliminar el único segmento de memoria existente."));
        }
        self.segmento_memoria.remove(num_de_banco);
        if self.banco_actual >= self.segmento_memoria.len() as u8 {
            self.banco_actual = self.segmento_memoria.len() as u8 - 1;
        }
        Ok(())
    }
    
    
    pub fn get_banco_activo (&self) -> u8 { self.banco_actual }

    pub fn set_banco_activo (&mut self, num_de_banco: u8) { self.banco_actual = num_de_banco; }

}
pub struct BankedMemory {
    pub banks: Vec<Vec<u8>>,
    current_bank: usize,
}

impl BankedMemory {
    pub fn new(num_banks: usize, bank_size: usize) -> BankedMemory {
        BankedMemory {
            banks: vec![vec![0; bank_size]; num_banks],
            current_bank: 0,
        }
    }

    pub fn select_bank(&mut self, bank_num: usize) {
        self.current_bank = bank_num;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let bank_offset = (self.current_bank as u16) * 0x4000;
        self.banks[self.current_bank][(addr - bank_offset) as usize]
    }

    /*
    */
    pub fn read_byte_no(&self, addr: u16) -> u8 {
        let bank_offset = (self.current_bank as u16) * 0x4000;
        let mem_addr = addr - bank_offset;
        if mem_addr < self.banks[self.current_bank].len() as u16 {
            self.banks[self.current_bank][mem_addr as usize]
        } else {
            panic!("Error de overflow en BankedMemory: la dirección 0x{:04X} está fuera del rango del banco actual.", addr);
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
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

/*
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
 */

//*****************************************************************************

