/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-03-2023
    Titulo:         Funciones de manejo de registros y flags - Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

/* Little-endian - Big-endian               
u16::from_le_bytes y u16::from_be_bytes son dos métodos en el tipo u16 de Rust que te permiten
convertir un arreglo de bytes en un valor u16. La diferencia entre ellos está en el orden de
los bytes utilizado para interpretar el arreglo.

u16::from_le_bytes interpreta el arreglo en orden de bytes little-endian, lo que significa que
el byte menos significativo está primero en el arreglo y el byte más significativo está último.
Este es el orden de bytes utilizado por la mayoría de las computadoras basadas en Intel.

u16::from_be_bytes interpreta el arreglo en orden de bytes big-endian, lo que significa que el
byte más significativo está primero en el arreglo y el byte menos significativo está último. Este
es el orden de bytes utilizado por la mayoría de los protocolos de red y algunas otras arquitecturas
de computadora.

Por ejemplo, si tienes un arreglo de bytes [0x78, 0x56], u16::from_le_bytes lo interpretará como el
valor u16 0x5678, mientras que u16::from_be_bytes lo interpretará como el valor u16 0x7856.

little endian 0xABCD =
Dirección de memoria:  | 0x0000 | 0x0001 |
                       +--------+--------+
     Contenido (hex):  |  0xCD  |  0xAB  |
                       +--------+--------+
*/

//***************************************************************************** Módulo de emulación
pub struct BancosMemoria {
    pub segmento_memoria: Vec<Vec<u8>>,
    pub banco_actual: u8,
    pub endianess: Endianess,
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
            endianess: Endianess::LittleEndian,
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
        let val = match self.endianess {
            Endianess::LittleEndian => val,
            Endianess::BigEndian => val.swap_bytes(),
        };
        if usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de almacenar fuera del rango del segmento de memoria")
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize] = val;
        }
    }

    pub fn leer_memoria(&self, direccion: u16) -> u8 {
        let val = if usize::from(direccion) > self.segmento_memoria[self.banco_actual as usize].len() {
            println!("Intento de leer fuera del rango del segmento de memoria");
            return 0;
        } else {
            self.segmento_memoria[self.banco_actual as usize][direccion as usize]
        };
        match self.endianess {
            Endianess::LittleEndian => val,
            Endianess::BigEndian => val.swap_bytes(),
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

//************************************* 
/*
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

//***************************************************************************** 