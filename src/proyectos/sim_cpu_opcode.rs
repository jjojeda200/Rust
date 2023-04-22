/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-04-2023
    Titulo:         Metodos asociados a los opcode - Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

// use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_memoria::Endianess};
use super::{sim_cpu_registros::CPU, /*sim_cpu_registros::Flags */};
use colored::*;

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.blue());
}

//***************************************************************************** 
impl CPU {
    pub fn cargar_programa(&mut self, programa: &Vec<u8>) {
        // Iterar a través de cada instrucción del programa proporcionado.
        for (i, &instruccion) in programa.iter().enumerate() {
            // Escribir cada instrucción en la memoria en la posición de memoria correspondiente.
            // La posición de memoria se determina por el índice actual de iteración `i`.
            // Se convierte a u16 para garantizar que sea una dirección de memoria válida.
            self.memoria.escribir_memoria(i as u16, instruccion);
        }
    }

    pub fn busca_instruccion(&mut self) -> u8 { 
        // Obtener la instrucción de la memoria en la dirección del contador de programa (1 byte)
        let instruccion = self.memoria.leer_memoria(self.contador_de_programa); 
        instruccion
    }

    // Definición de la función 'decodifica_instruccion', que toma un byte 'instruccion' como
    // argumento y devuelve una tupla de un byte 'opcode' y una matriz de dos bytes 'operandos'.
    pub fn decodifica_instruccion(&self, instruccion: u8) -> (u8, [u8; 2]) {
        // Asignación del byte 'instruccion' a la variable 'opcode'.
        let opcode = instruccion;
        // Definición de la matriz 'operandos', que se compone de los dos bytes siguientes en la
        // memoria en las posiciones de memoria 'contador_de_programa +1' y 'contador_de_programa +2'.
        let operandos = [
            self.memoria.leer_memoria(self.contador_de_programa +1),
            self.memoria.leer_memoria(self.contador_de_programa +2),
        ];
        // Devuelve la tupla 'opcode' y 'operandos'.
        (opcode, operandos)
    }

    pub fn ejecuta_instruccion(&mut self, opcode: u8, operandos: [u8; 2]) {
        match opcode {
            0x00 => { // NOP: No hace nada
                self.mnemonic = "NOP".to_string();
                self.contador_de_programa += 1;
            }

            0x04 => { // INR B incrementa el contenido en el Registro (B)
                self.reg_b = self.flags.add(self.reg_b, 0x01, false, false);
                self.mnemonic = "INR B".to_string();
                self.contador_de_programa += 1;
            }

// Corregir
            0x05 => { // DCR B decrementa el contenido en el Registro (B)
                self.reg_b -= 1;
                self.mnemonic = "DCR B".to_string();
                self.contador_de_programa += 1;
                /* 0x054 sin control de desbordamiento
                    let b = get_register_value(Register::B);
                    let result = b.wrapping_sub(1);
                    set_register_value(Register::B, result);
                    set_flags_sub(b, 1, result);

                ****************************
                El flag Z se establece en 1 si el resultado de la operación es cero (es decir, si el
                valor original de B era 1).
                El flag S se establece en 1 si el bit más significativo del resultado es 1 (es decir,
                si el valor original de B era mayor o igual a 0x80).
                El flag P se establece en 1 si el número de bits 1 en el resultado es par.
                El flag CY no se ve afectado por esta instrucción.
                El flag AC se establece en 1 si ocurre un acarreo en el nibble inferior de la operación
                (es decir, si el valor original de B era menor o igual a 0x0F).

                Por lo tanto, en la implementación en Rust de la instrucción 0x05 en una CPU 8080, la
                función opcode_05 debería actualizar los flags de la CPU según las reglas descritas
                anteriormente:

                impl Cpu {
                    fn opcode_05(&mut self) {
                        let old_value = self.b;
                        self.b = self.b.wrapping_sub(1);
                        self.flags.z = self.b == 0;
                        self.flags.s = (self.b & 0x80) != 0;
                        self.flags.p = self.b.count_ones() % 2 == 0;
                        self.flags.ac = (old_value & 0x0F) == 0;
                    }
                }
                */
            }

            0x06 => { // MVI B,d8 cargar un valor de 8 bits en el Registro (B)
                self.reg_b = operandos[0];  // self.reg_b = self.memoria.leer_memoria(self.contador_de_programa + 1);
                self.mnemonic = "MVI B,d8".to_string();
                self.contador_de_programa += 2;
            }

// Verificar
            0x0A => { // LDAX B cargar el valor contenido en la dirección BC bits en el acumulador (A)
                let direccion = u16::from_le_bytes([self.reg_b, self.reg_c]);

                let dire = self.get_bc().swap_bytes();
                
                self.reg_a = self.memoria.leer_memoria(direccion);
                self.mnemonic = "LDAX B".to_string();
                self.contador_de_programa += 1;
            }

            0x32 => { // STA addr: carga el registro A en la dirección apuntada por HL
                //self.reg_h = operandos[0];  // self.reg_h = self.memoria.leer_memoria(self.contador_de_programa + 1);
                //self.reg_l = operandos[1];  // self.reg_l = self.memoria.leer_memoria(self.contador_de_programa + 2);
                let direccion = u16::from_le_bytes([operandos[0], operandos[1]]);
                // let direccion = u16::from_be_bytes([self.reg_h, self.reg_l]);
                self.memoria.escribir_memoria(direccion, self.reg_a);
                self.mnemonic = "STA addr".to_string();
                self.contador_de_programa += 3;
            },
        
            0x3A => { // LDA addr: carga el valor de la dirección apuntada por los dos siguientes bytes en el acumulador (A)
                let direccion = u16::from_le_bytes([operandos[0], operandos[1]]);
                self.reg_a = self.memoria.leer_memoria(direccion);
                self.mnemonic = "LDA addr".to_string();
                self.contador_de_programa += 3;
            },        

            0x3C => { // INR A incrementa el contenido en el Registro (A)
                self.reg_a = self.flags.add(self.reg_a, 0x01, false, false);
                self.mnemonic = "INR A".to_string();
                self.contador_de_programa += 1;
            }

// Corregir
            0x3D => { // DCR A decrementa el contenido en el Registro (A)
                self.reg_a -= 1;
                self.mnemonic = "DCR A".to_string();
                self.contador_de_programa += 1;
                /*
                Si se decrementa el acumulador en 8080 hasta que llega a 0, esto provocará un desbordamiento (overflow) en el registro, es decir, que el valor almacenado en el acumulador pasará de 0xFF (255 en decimal) a 0x00 (0 en decimal).
                Además, al realizar una operación que decrementa el valor del acumulador, se establecerá la bandera de cero (Z) en 1 si el resultado es 0, o en 0 en caso contrario. También se establecerá la bandera de signo (S) si el bit más significativo del resultado es 1.
                */
            }

            0x3E => { // MVI A,n cargar un valor de 8 bits en el acumulador (A)
                self.reg_a = operandos[0];  // self.reg_a = self.memoria.leer_memoria(self.contador_de_programa + 1);
                self.mnemonic = "MVI A,d8".to_string();
                self.contador_de_programa += 2;
            }

            0x80 => { // ADD A,B suma el contenido del Registro B al acumulador (A)
                let resultado_add = self.flags.add(self.reg_a, self.reg_b, true, false);
                self.reg_a = resultado_add;
                self.mnemonic = "ADD A,B".to_string();
                self.contador_de_programa += 1;
            }

            0x88 => { // ADC A,B suma el contenido del Registro B + Acarreo operación anterior al acumulador (A)
                let resultado_adc = self.flags.adc(self.reg_a, self.reg_b, false);
                self.reg_a = resultado_adc;
                self.mnemonic = "ADC A,B".to_string();
                self.contador_de_programa += 1;
            }

            0xC3 => { // JMP nn marca PC con la dirección indicada por los dos siguientes bytes
                self.contador_de_programa = u16::from_le_bytes([operandos[0], operandos[1]]);
                self.mnemonic = "JMP nn".to_string();
            }

// Revisar *********************************
            _ => { print!("exit"); }
        }
    }

}
    
//***************************************************************************** 



//*****************************************************************************
/* Operaciones ALU pendientes de implementar
impl Alu {

    fn sub(&mut self, value: u8) {
        let (result, carry) = self.a.overflowing_sub(value);
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(carry);
        self.flags.set_aux_carry((self.a & 0x0F) < (value & 0x0F));
        self.flags.set_parity(result);
        self.a = result;
    }

    fn sbb(&mut self, value: u8) {
        let (result, carry) = self.a.overflowing_sub(value);
        let (result, carry2) = result.overflowing_sub(self.flags.carry() as u8);
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(carry || carry2);
        self.flags.set_aux_carry((self.a & 0x0F) < ((value & 0x0F) + self.flags.carry() as u8));
        self.flags.set_parity(result);
        self.a = result;
    }

    fn and(&mut self, value: u8) {
        let result = self.a & value;
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(false);
        self.flags.set_aux_carry(true);
        self.flags.set_parity(result);
        self.a = result;
    }

    fn or(&mut self, value: u8) {
        let result = self.a | value;
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(false);
        self.flags.set_aux_carry(false);
        self.flags.set_parity(result);
        self.a = result;
    }

    fn xor(&mut self, value: u8) {
        let result = self.a ^ value;
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(false);
        self.flags.set_aux_carry(false);
        self.flags.set_parity(result);
        self.a = result;
    }
       
    fn cmp(&mut self, value: u8) {
        let (result, carry) = self.a.overflowing_sub(value);
        self.flags.set_zero(result == 0);
        self.flags.set_sign(result & 0x80 != 0);
        self.flags.set_carry(carry);
        self.flags.set_aux_carry((self.a & 0x0F) < (value & 0x0F));
        self.flags.set_parity(result);
    }
*/

//*****************************************************************************