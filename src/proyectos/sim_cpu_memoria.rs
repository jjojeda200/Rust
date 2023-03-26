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
#![allow(unused_assignments)]

//***************************************************************************** Módulo de emulación
pub struct BancosMemoria {
    pub segmento_memoria: Vec<Vec<u8>>,
    pub banco_actual: u8,
}

pub enum Endianess {
    LittleEndian,
    BigEndian,
}

pub struct UnidadMemoria {
    data: Vec<u8>,                                  // Datos de la unidad de memoria
    start_address: u16,                             // Dirección de inicio de la unidad de memoria
    end_address: u16,                               // Dirección final de la unidad de memoria
    read_handler: Option<Box<dyn Fn(u16) -> u8>>,   // Función para leer datos de la unidad de memoria
    write_handler: Option<Box<dyn Fn(u16, u8)>>,    // Función para escribir datos en la unidad de memoria
    endianness: Endianess,
}

// Nota importante: los índices de matrices deben ser de tipo usize.
impl BancosMemoria {
    pub fn new() -> BancosMemoria {
        BancosMemoria {
            segmento_memoria: vec![vec![0; 16384]; 1],
            banco_actual: 0,
        }
    }

    pub fn crear_segmento(&mut self, longitud_del_segmento: usize) {
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

    pub fn escribir_memoria(&mut self, direccion: u16, val: u8) {
        if /*direccion < 0 ||*/ usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de almacenar fuera del rango del segmento de memoria")
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize] = val;
        }
    }

    pub fn leer_memoria(&self, direccion: u16) -> u8 {
        if /*direccion < 0 ||*/usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de leer fuera del rango del segmento de memoria");
            return 0;
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize]
        }
    }

}

//*****************************************************************************  Test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_bancos_memoria() {
        let mut bancos_memoria = BancosMemoria::new();

        // Escribimos y leemos en la posición 0 del segmento de memoria actual
        bancos_memoria.escribir_memoria(0, 5);
        assert_eq!(bancos_memoria.leer_memoria(0), 5);

        // Cambiamos al segundo segmento de memoria y escribimos en la posición 0
        bancos_memoria.crear_segmento(16384);
        bancos_memoria.set_banco_activo(1);
        bancos_memoria.escribir_memoria(0, 10);

        // Volvemos al primer segmento de memoria y comprobamos que el valor en la posición 0 no ha cambiado
        bancos_memoria.set_banco_activo(0);
        assert_eq!(bancos_memoria.leer_memoria(0), 5);

        // Cambiamos de nuevo al segundo segmento de memoria y comprobamos que el valor en la posición 0 ha cambiado
        bancos_memoria.set_banco_activo(1);
        assert_eq!(bancos_memoria.leer_memoria(0), 10);

        // Intentamos leer y escribir fuera del rango del segmento de memoria
        bancos_memoria.escribir_memoria(30000, 15);
        assert_eq!(bancos_memoria.leer_memoria(30000), 0);
    }
}

//***************************************************************************** 
/* Modificaciones para añadir LittleEndian y BigEndian                              

pub enum Endianess {
    LittleEndian,
    BigEndian,
}

pub struct BancosMemoria {
    pub segmento_memoria: Vec<Vec<u8>>,
    pub banco_actual: u8,
    pub endianess: Endianess,
}

impl BancosMemoria {
    pub fn new(endianess: Endianess) -> BancosMemoria {
        BancosMemoria {
            segmento_memoria: vec![vec![0; 16384]; 1],
            banco_actual: 0,
            endianess: endianess,
        }
    }

    // El resto de los métodos se mantienen igual...

    pub fn escribir_memoria(&mut self, direccion: u16, val: u8) {
        let val = match self.endianess {
            Endianess::LittleEndian => val,
            Endianess::BigEndian => val.swap_bytes(),
        };
        if /*direccion < 0 ||*/ usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de almacenar fuera del rango del segmento de memoria");
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize] = val;
        }
    }

    pub fn leer_memoria(&self, direccion: u16) -> u8 {
        let val = if /*direccion < 0 ||*/usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de leer fuera del rango del segmento de memoria");
            0
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize]
        };
        match self.endianess {
            Endianess::LittleEndian => val,
            Endianess::BigEndian => val.swap_bytes(),
        }
    }
}
*/

//************************************* 
/* Implementación de unidades de memoria, sus metodos, LitteEndian y Bigendian 

pub struct UnidadMemoria {
    data: Vec<u8>,                                  // Datos de la unidad de memoria
    start_address: u16,                             // Dirección de inicio de la unidad de memoria
    end_address: u16,                               // Dirección final de la unidad de memoria
    read_handler: Option<Box<dyn Fn(u16) -> u8>>,   // Función para leer datos de la unidad de memoria
    write_handler: Option<Box<dyn Fn(u16, u8)>>,    // Función para escribir datos en la unidad de memoria
    endianness: Endianess,
}


impl UnidadMemoria {
    pub fn new(start_address: u16, end_address: u16, endianness: Endianness) -> UnidadMemoria {
        UnidadMemoria {
            data: vec![0; usize::from(end_address - start_address + 1)],
            start_address,
            end_address,
            read_handler: None,
            write_handler: None,
            endianness,
        }
    }

    pub fn set_read_handler<F>(&mut self, handler: F)
        where F: 'static + Fn(u16, Endianness) -> u8
    {
        self.read_handler = Some(Box::new(handler));
    }

    pub fn set_write_handler<F>(&mut self, handler: F)
        where F: 'static + Fn(u16, u8, Endianness)
    {
        self.write_handler = Some(Box::new(handler));
    }

    pub fn read(&self, address: u16) -> u8 {
        if let Some(ref handler) = self.read_handler {
            handler(address, self.endianness)
        } else {
            self.data[usize::from(address - self.start_address)]
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(ref handler) = self.write_handler {
            handler(address, value, self.endianness)
        } else {
            self.data[usize::from(address - self.start_address)] = value;
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = u16::from(self.read(address));
        let hi = u16::from(self.read(address + 1));
        match self.endianness {
            Endianness::BigEndian => (hi << 8) | lo,
            Endianness::LittleEndian => (lo << 8) | hi,
        }
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;
        match self.endianness {
            Endianness::BigEndian => {
                self.write(address, hi);
                self.write(address + 1, lo);
            },
            Endianness::LittleEndian => {
                self.write(address, lo);
                self.write(address + 1, hi);
            },
        }
    }
}
*/

//************************************* MemoryUnit
/* struct MemoryUnit                        
En esta estructura, los datos de la memoria estarían almacenados en un vector dinámico.
Además, se han agregado los campos start_address y end_address para indicar las
direcciones de inicio y final de la unidad de memoria.

También se han agregado dos campos opcionales de tipo Fn, read_handler y write_handler,
que son funciones que se llamarán cuando se intente leer o escribir en una dirección
de memoria que pertenezca a esta unidad. Estas funciones permiten implementar de 
forma flexible el comportamiento de la memoria, por ejemplo, simulando dispositivos
de entrada/salida o áreas de memoria especiales.

La estructura Z80CPU se modificaría para tener una lista de unidades de memoria:

struct Z80Reg {
    // ...
    mem: Vec<MemoryUnit>,       // Lista de unidades de memoria
}
*/
struct MemoryUnit {
    data: Vec<u8>,              // Datos de la unidad de memoria
    start_address: u16,         // Dirección de inicio de la unidad de memoria
    end_address: u16,           // Dirección final de la unidad de memoria
    read_handler: Option<Box<dyn Fn(u16) -> u8>>,   // Función para leer datos de la unidad de memoria
    write_handler: Option<Box<dyn Fn(u16, u8)>>,    // Función para escribir datos en la unidad de memoria
}

/*
Ejemplo de cómo se podrían realizar las modificaciones necesarias en la estructura Z80CPU
y en las funciones read_byte y write_byte para utilizar la nueva estructura MemoryUnit.
*/

struct Z80cpuAnalisis {        // Añadir
    mem: Vec<MemoryUnit>,       // Lista de unidades de memoria
}

impl Z80cpuAnalisis {
/* función read_byte                        
En la función read_byte, se itera sobre todas las unidades de memoria en la lista mem
hasta encontrar una que contenga la dirección addr. Si la unidad tiene una función
read_handler, se llama a esta función para obtener el valor de la memoria en la dirección
addr. Si la unidad no tiene una función read_handler, se busca el byte correspondiente en
el vector data.
*/
    fn read_byte(&self, addr: u16 ) -> u8 {
        for mem_unit in self.mem.iter() {
            if addr >= mem_unit.start_address && addr <= mem_unit.end_address {
                if let Some(ref read_handler) = mem_unit.read_handler {
                    return read_handler(addr);
                }
                else {
                    let offset = (addr - mem_unit.start_address) as usize;
                    return mem_unit.data[offset];
                }
            }
        }
        panic!("Error: dirección de memoria fuera de rango.");
    }
/* función write_byte
En la función write_byte, se realiza un proceso similar. Se itera sobre todas las unidades
de memoria en la lista mem hasta encontrar una que contenga la dirección addr. Si la unidad
tiene una función write_handler, se llama a esta función para escribir el valor value en la
dirección addr. Si la unidad no tiene una función write_handler, se busca el byte
correspondiente en el vector data y se escribe el valor value en el lugar correspondiente.
*/
    fn write_byte(&mut self, addr: u16, value: u8) {
        for mem_unit in self.mem.iter_mut() {
            if addr >= mem_unit.start_address && addr <= mem_unit.end_address {
                if let Some(ref write_handler) = mem_unit.write_handler {
                    write_handler(addr, value);
                    return;
                }
                else {
                    let offset = (addr - mem_unit.start_address) as usize;
                    mem_unit.data[offset] = value;
                    return;
                }
            }
        }
        panic!("Error: dirección de memoria fuera de rango.");
    }
}
/* 
Para agregar una nueva unidad de memoria, se puede crear una nueva instancia de la
estructura MemoryUnit y agregarla a la lista mem de la siguiente manera:En este ejemplo,
se crea una nueva unidad de memoria de 1 KB que va desde la dirección 0x2000 hasta la
dirección 0x2FFF.

pub fn z80_sim(){
    let new_mem_unit = MemoryUnit {
        data: vec![0; 1024],        // 1 KB de memoria inicializada con ceros
        start_address: 0x2000,
        end_address: 0x2FFF,
        read_handler: None,
        write_handler: None,
    };
    reg.mem.push(new_mem_unit);
}
*/

//***************************************************************************** 