/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          28-03-2023
    Titulo:         introducción a RUST
    Descripción:    Ejemplo de traits documentado basado en el manejo de bytes 
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

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}



//*****************************************************************************
/*
Este trait tiene seis métodos:
    from_bytes: convierte un slice de bytes en una instancia del tipo que implementa el trait. Por ejemplo,
    si tenemos un tipo MiTipo que implementa ManipulaByte, podemos crear una nueva instancia a partir de
    un slice de bytes con let mi_var = MiTipo::from_bytes(&[0x01, 0x02, 0x03]);.

    to_bytes: convierte la instancia del tipo que implementa el trait en un vector de bytes. Por ejemplo, si
    mi_var es una instancia de MiTipo, podemos obtener su representación en bytes con let bytes = mi_var.to_bytes();.

    len_bytes: devuelve el número de bytes que ocupa la instancia del tipo que implementa el trait. Por ejemplo,
    si mi_var ocupa 3 bytes, podemos obtener este valor con let len = mi_var.len_bytes();.

    get_byte: devuelve el byte en la posición especificada. Si la posición está fuera de los límites de la
    instancia, devuelve None. Por ejemplo, si mi_var es una instancia de MiTipo, podemos obtener el segundo
    byte con let byte = mi_var.get_byte(1);.

    set_byte: establece el byte en la posición especificada en el valor dado. Si la posición está fuera de los
    límites de la instancia, devuelve None. Por ejemplo, si mi_var es una instancia de MiTipo, podemos establecer
    el segundo byte en 0x42 con mi_var.set_byte(1, 0x42);.

    copy_bytes: copia los bytes de la instancia del tipo que implementa el trait en el slice de destino, comenzando
    en el desplazamiento especificado. Si la instancia es demasiado grande para caber en el slice de destino,
    devuelve None. Por ejemplo, si mi_var es una instancia de MiTipo y tenemos un slice de destino dest con capacidad
    suficiente, podemos copiar sus bytes en dest con mi_var.copy_bytes(dest, 0);.

Estos métodos pueden ser implementados para cualquier tipo que necesite manipular datos en formato de bytes, como por
ejemplo tipos de datos de red, formatos de archivo binarios o estructuras de datos personalizadas.
*/
// Trait que define una serie de métodos para manipular datos en formato de bytes
trait ManipulaByte {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn len_bytes(&self) -> usize;
    fn get_byte(&self, index: usize) -> Option<u8>;
    fn set_byte(&mut self, index: usize, value: u8) -> Option<()>;
    fn copy_bytes(&self, dest: &mut [u8], offset: usize) -> Option<()>;
}

#[derive(Debug, Copy, Clone)]
struct MiEstructura {
    campo1: u16,
    campo2: u8,
    campo3: u32,
}

/*
from_be_bytes: Esta función es una función de conversión que toma un array de bytes y devuelve un valor
de un tipo numérico (por ejemplo, u16 o u32). La función interpreta los bytes como una representación
big-endian (el byte más significativo primero) y convierte ese valor en el tipo numérico correspondiente.
Por ejemplo, en el código compartido, se utiliza u16::from_be_bytes para interpretar los primeros dos
bytes de un slice de bytes como un valor u16 big-endian.

to_be_bytes: Esta función es la inversa de from_be_bytes. Toma un valor de un tipo numérico y devuelve un
array de bytes que representa ese valor en formato big-endian. Por ejemplo, en el código compartido, se
utiliza self.campo1.to_be_bytes() para convertir el valor self.campo1 de tipo u16 en un array de dos bytes
big-endian.

copy_from_slice: Esta función copia los bytes de un slice a otro slice. Toma como argumentos un slice de
origen y un slice de destino, y copia los bytes del slice de origen al slice de destino. El número de bytes
copiados es igual al tamaño del slice de destino. Si el slice de origen es más pequeño que el slice de destino,
la función copia solo los primeros bytes del slice de origen. Por ejemplo, en el código compartido, se utiliza
bytes[0..2].copy_from_slice(&self.campo1.to_be_bytes()) para copiar los primeros dos bytes del array devuelto
por self.campo1.to_be_bytes() al inicio del slice bytes.
*/

impl ManipulaByte for MiEstructura {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 7, "No hay suficientes bytes para crear MiEstructura");

        let campo1 = u16::from_be_bytes([bytes[0], bytes[1]]);
        let campo2 = bytes[2];
        let campo3 = u32::from_be_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]);

        MiEstructura {
            campo1,
            campo2,
            campo3,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; 7];

        bytes[0..2].copy_from_slice(&self.campo1.to_be_bytes());
        bytes[2] = self.campo2;
        bytes[3..7].copy_from_slice(&self.campo3.to_be_bytes());

        bytes
    }

    fn len_bytes(&self) -> usize {
        7
    }

    fn get_byte(&self, index: usize) -> Option<u8> {
        match index {
            0 => Some((self.campo1 >> 8) as u8),
            1 => Some(self.campo1 as u8),
            2 => Some(self.campo2),
            3 => Some((self.campo3 >> 24) as u8),
            4 => Some((self.campo3 >> 16) as u8),
            5 => Some((self.campo3 >> 8) as u8),
            6 => Some(self.campo3 as u8),
            _ => None,
        }
    }

    fn set_byte(&mut self, index: usize, value: u8) -> Option<()> {
        match index {
            0 => {
                self.campo1 = (self.campo1 & 0x00FF) | ((value as u16) << 8);
                Some(())
            }
            1 => {
                self.campo1 = (self.campo1 & 0xFF00) | (value as u16);
                Some(())
            }
            2 => {
                self.campo2 = value;
                Some(())
            }
            3 => {
                self.campo3 = (self.campo3 & 0x00FFFFFF) | ((value as u32) << 24);
                Some(())
            }
            4 => {
                self.campo3 = (self.campo3 & 0xFF00FFFF) | ((value as u32) << 16);
                Some(())
            }
            5 => {
                self.campo3 = (self.campo3 & 0xFFFF00FF) | ((value as u32) << 8);
                Some(())
            }
            6 => {
                self.campo3 = (self.campo3 & 0xFFFFFF00) | (value as u32);
                Some(())
            }
            _ => None,
        }
    }

    fn copy_bytes(&self, dest: &mut [u8], offset: usize) -> Option<()> {
        if dest.len() < self.len_bytes() + offset {
            return None;
        }

        dest[offset..(offset + 2)].copy_from_slice(&self.campo1.to_be_bytes());
        dest[offset + 2] = self.campo2;
        dest[(offset + 3)..(offset + 7)].copy_from_slice(&self.campo3.to_be_bytes());

        Some(())
    }
}

//*****************************************************************************
pub fn manipulacion_byte_0() {
    let titulo = String::from(" Trait - Manipulación de Bytes ");
    imprime_titulo(&titulo);

    let my_struct = MiEstructura {
        campo1: 0xFFFF,
        campo2: 0x00,
        campo3: 0xFF00FF00,
    };

    // Convert the struct to bytes
    let bytes = my_struct.to_bytes();
    impresion_datos_hex(&bytes);

    // Convert the bytes back to a struct
    let my_struct_copy = MiEstructura::from_bytes(&bytes);

    // Print the original struct and the copy
    println!("estructura original: {:?}", my_struct);
    println!("Copia de estructura: {:?}", my_struct_copy);

    // Test get_byte and set_byte methods
    let mut my_struct_mut = my_struct.clone();
    let byte = my_struct_mut.get_byte(0).unwrap();
    println!("Byte en el índice 0: {}", byte);
    my_struct_mut.set_byte(0, byte + 0).unwrap();
    println!("Byte modificado en el índice 0: {:?}", my_struct_mut);

    // Test copy_bytes method
    let mut dest = vec![0; my_struct.len_bytes() * 2];
    impresion_datos_hex(&dest);

    my_struct.copy_bytes(&mut dest, 0).unwrap();
    println!("bytes copiados: {:?}", dest);
}

//*****************************************************************************
fn impresion_datos_hex(data:  &Vec<u8>) {
    // Imprimir los datos en formato hexadecimal
    let mut line_count = 0;
    for (i, byte) in data.iter().enumerate() {
        if i % 15 == 0 {
            print!("\n{:04X}: ", i);
            line_count += 1;
            if line_count == 16 {
                line_count = 0;
                // Esperar por una pulsación de tecla
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if data[i] == 0xFF {
                    break;
                }
            }
        }
        print!("{:02X} ", byte);
    }
    println!();
}

//*****************************************************************************