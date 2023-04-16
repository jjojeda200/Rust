/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          16-04-2023
    Titulo:         Metodos asociados a los opcode - Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use super::{sim_cpu_memoria::BancosMemoria, sim_cpu_memoria::Endianess};
use super::{sim_cpu_registros::RegistrosCPU, sim_cpu_registros::Flags};
use colored::*;

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo.blue());
}

use std::sync::Mutex;
static mut MNEMONICO_OPCODE: Option<Mutex<String>> = None;

//***************************************************************************** 

//***************************************************************************** Estructura e implementación Registros
pub struct RegistrosCPU {
    memoria: BancosMemoria,
    reg_a: u8,          // Acumulador A de 8 bits
    pub flags: Flags,   // Manejamos el registro de flags F de manera independiente
    reg_b: u8,          // Registro B de 8 bits
    reg_c: u8,          // Registro C de 8 bits
    reg_d: u8,          // Registro D de 8 bits
    reg_e: u8,          // Registro E de 8 bits
    reg_h: u8,          // Registro H de 8 bits
    reg_l: u8,          // Registro L de 8 bits
    reg_ix: u16,        // Registro IX de 16 bits
    reg_iy: u16,        // Registro IY de 16 bits
    contador_de_programa: u16,
    puntero_de_pila: u16,
    registro_instrucciones: u8,
}

impl RegistrosCPU {
    pub fn new() -> RegistrosCPU {
        RegistrosCPU {
            memoria: BancosMemoria {
                segmento_memoria: vec![vec![0; 1024]; 1],
                //segmento_memoria: vec![vec![0; 16384]; 1],
                banco_actual: 0,
                endianess: Endianess::LittleEndian,
            },
            flags: Flags { carry: false, subtract: false, parity_overflow: false, half_carry: false, zero: false, sign: false, },
            reg_a: 0, 
            reg_b: 0, 
            reg_c: 0, 
            reg_d: 0, 
            reg_e: 0, 
            reg_h: 0, 
            reg_l: 0, 
            reg_ix: 0, 
            reg_iy: 0, 
            contador_de_programa: 0,
            puntero_de_pila: 0,
            registro_instrucciones: 0,
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
    //********************************* Otra forma de manejar BC
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
    pub fn get_l(&self) -> u8 { self.reg_l }
    pub fn set_l(&mut self, valor: u8) { self.reg_l = valor; }
    pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.reg_h, self.reg_l]) }
    pub fn set_hl(&mut self, valor: u16) {
        let bytes = valor.to_be_bytes();
        self.reg_h = bytes[0];
        self.reg_l = bytes[1];
    }

}

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




//***************************************************************************** Test  flags - manejo de bit y calculo individual
#[cfg(test)]
mod tests_flags_0 {
    use super::*;

    #[test]
    fn test_new_flags() {
        let flags = Flags::new_flags();
        assert_eq!(flags.carry, false);
        assert_eq!(flags.subtract, true);
        assert_eq!(flags.parity_overflow, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.zero, false);
        assert_eq!(flags.sign, false);
    }

    #[test]
    fn test_get_flags() {
        let mut flags = Flags::new_flags();
        flags.carry = true;
        flags.half_carry = true;
        flags.zero = true;
        let result = flags.get_flags();
        assert_eq!(result, 0b01010011);
    }

    #[test]
    fn test_get_flags_1() {
        let mut flags = Flags::new_flags();
        flags.carry = true;
        flags.half_carry = true;
        flags.zero = true;
        let result = flags.get_flags_1();
        assert_eq!(result, 0b01010011);
    }

    #[test]
    fn test_set_flags() {
        let mut flags = Flags::new_flags();
        flags.set_flags(0b11001101);
        assert_eq!(flags.carry, true);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.parity_overflow, true);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.zero, true);
        assert_eq!(flags.sign, true);
    }

    #[test]
    fn test_get_bit() {
        let mut flags = Flags::new_flags();
        flags.carry = true;
        flags.parity_overflow = true;
        flags.sign = true;
        assert_eq!(flags.get_bit(0), 1);
        assert_eq!(flags.get_bit(1), 1);
        assert_eq!(flags.get_bit(2), 1);
        assert_eq!(flags.get_bit(4), 0);
        assert_eq!(flags.get_bit(6), 0);
        assert_eq!(flags.get_bit(7), 1);
    }

    #[test]
    fn test_get_bit_1() {
        let mut flags = Flags::new_flags();
        flags.carry = true;
        flags.parity_overflow = true;
        flags.sign = true;
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(1), true);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);
    }

    #[test]
    fn test_set_bit() {
        let mut flags = Flags::new_flags();
        flags.set_bit(0, true);
        flags.set_bit(4, true);
        flags.set_bit(7, true);
        assert_eq!(flags.carry, true);
        assert_eq!(flags.half_carry, true);
        assert_eq!(flags.sign, true);
    }

    #[test]
    fn test_flags_acarreo() {
        let mut flags = Flags::new_flags();
        let result = flags.flags_acarreo_add(255, 1);
        assert_eq!(result, 0);
        assert_eq!(flags.carry, true);

        flags = Flags::new_flags();
        let result = flags.flags_acarreo_add(100, 20);
        assert_eq!(result, 120);
        assert_eq!(flags.carry, false);

        flags = Flags::new_flags();
        let result = flags.flags_acarreo_add(200, 100);
        assert_eq!(result, 44);
        assert_eq!(flags.carry, true);
    }

    #[test]
    fn test_flags_paridad() {
        let mut flags = Flags::new_flags();
        flags.flags_paridad(0xff);
        assert_eq!(flags.parity_overflow, true);

        flags = Flags::new_flags();
        flags.flags_paridad(0b01110110);
        assert_eq!(flags.parity_overflow, false);

        flags = Flags::new_flags();
        flags.flags_paridad(0b01100110);
        assert_eq!(flags.parity_overflow, true);

        flags = Flags::new_flags();
        flags.flags_paridad(0b00001000);
        assert_eq!(flags.parity_overflow, false);
    }

    #[test]
    fn test_flags_acarreo_auxiliar() {
        let mut flags = Flags::new_flags();
        flags.flags_acarreo_auxiliar_add(0x1F, 0x01);
        assert_eq!(flags.half_carry, true);
        
        flags.flags_acarreo_auxiliar_add(0x08,0x06);
        assert_eq!(flags.half_carry, false);

        flags.flags_acarreo_auxiliar_add(0xAF, 0x0F);
        assert_eq!(flags.half_carry, true);

        flags.flags_acarreo_auxiliar_add(0x1B, 0x01);
        assert_eq!(flags.half_carry, false);

        flags.flags_acarreo_auxiliar_add(0x9F, 0x01);
        assert_eq!(flags.half_carry, true);

        flags.flags_acarreo_auxiliar_add(0x0F, 0x01);
        assert_eq!(flags.half_carry, true);
    }

    #[test]
    fn test_flags_signo() {
        let mut flags = Flags::new_flags();
        
        flags.flags_signo(0x7F);
        assert_eq!(flags.sign, false);
        
        flags.flags_signo(0x80);
        assert_eq!(flags.sign, true);
    }
}

//***************************************************************************** Test flags - ALU
#[cfg(test)]
mod tests_flags_1 {
    const IMP_TEST: bool = false;
    const CONT_CF: bool = true;
    use crate::proyectos::sim_cpu_registros::Flags;
    
    //********************************* Test de ADD
    #[test]
    fn test_add() {
        let mut flags = Flags::new_flags();
        assert_eq!(flags.add(250, 10, CONT_CF, IMP_TEST), 4);
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);

        let mut flags = Flags::new_flags();
        assert_eq!(flags.add(250, 2, CONT_CF, IMP_TEST), 252);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);
 
        let mut flags = Flags::new_flags();
        assert_eq!(flags.add(0x0f, 0x01, CONT_CF, IMP_TEST), 16);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);
        
        assert_eq!(flags.add(15, 2, CONT_CF, IMP_TEST), 17);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);

        assert_eq!(flags.add(0xA, 0x04, CONT_CF, IMP_TEST), 14);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);
        
        let mut flags = Flags::new_flags();
        assert_eq!(flags.add(0, 0, CONT_CF, IMP_TEST), 0);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), true);
        assert_eq!(flags.get_bit_1(7), false);

        assert_eq!(flags.add(0xFE, 1, CONT_CF, IMP_TEST), 255);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);
        
        assert_eq!(flags.add(0x0F, 0x0B, CONT_CF, IMP_TEST), 0x1A);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);
    }

    //********************************* Test de ADC
    #[test]
    fn test_adc() {
        let mut flags = Flags::new_flags();

        flags.set_bit(0, true);
        assert_eq!(flags.adc(0b11111010, 0b00001010, IMP_TEST), 0b00000101);
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);

        assert_eq!(flags.adc(0b00001110, 0b00000001, IMP_TEST), 16);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);

        flags.set_bit(0, false);
        assert_eq!(flags.adc(250, 2, IMP_TEST), 252);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);

        assert_eq!(flags.adc(0x0A, 0x06, IMP_TEST), 16);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), false);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), false);

        assert_eq!(flags.adc(0xFE, 0x02, IMP_TEST), 0);
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), true);
        assert_eq!(flags.get_bit_1(7), false);
        
        assert_eq!(flags.adc(0xFE, 0, IMP_TEST), 255);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);
    }

    //********************************* Test de ADC
    #[test]
    fn test_inr() {
        let mut flags = Flags::new_flags();

        flags.set_bit(0, false);
        assert_eq!(flags.add(0b11111110, 0b00000001, false, IMP_TEST), 0b11111111);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);

        assert_eq!(flags.add(0b11111111, 0b00000001, false, IMP_TEST), 0b00000000);
        assert_eq!(flags.get_bit_1(0), false);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), true);
        assert_eq!(flags.get_bit_1(7), false);

        flags.set_bit(0, true);
        assert_eq!(flags.add(0b11111110, 0b00000001, false, IMP_TEST), 0b11111111);
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), false);
        assert_eq!(flags.get_bit_1(6), false);
        assert_eq!(flags.get_bit_1(7), true);

        assert_eq!(flags.add(0b11111111, 0b00000001, false, IMP_TEST), 0b00000000);
        assert_eq!(flags.get_bit_1(0), true);
        assert_eq!(flags.get_bit_1(2), true);
        assert_eq!(flags.get_bit_1(4), true);
        assert_eq!(flags.get_bit_1(6), true);
        assert_eq!(flags.get_bit_1(7), false);
    }
}

//***************************************************************************** Test manejo de Registros
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registros() {
        let registros = RegistrosCPU::new();
        assert_eq!(registros.memoria.segmento_memoria[0][0], 0);
        assert_eq!(registros.flags.carry, false);
        assert_eq!(registros.reg_a, 0);
        assert_eq!(registros.reg_b, 0);
        assert_eq!(registros.reg_c, 0);
        assert_eq!(registros.reg_d, 0);
        assert_eq!(registros.reg_e, 0);
        assert_eq!(registros.reg_h, 0);
        assert_eq!(registros.reg_l, 0);
        assert_eq!(registros.reg_ix, 0);
        assert_eq!(registros.reg_iy, 0);
        assert_eq!(registros.sp, 0);
        assert_eq!(registros.pc, 0);
        assert_eq!(registros.contador_de_programa, 0);
        assert_eq!(registros.puntero_de_pila, 0);
        assert_eq!(registros.registro_instrucciones, 0);
    }

    #[test]
    fn test_get_set_a() {
        let mut registros = RegistrosCPU::new();
        registros.set_a(0x55);
        assert_eq!(registros.get_a(), 0x55);
    }

    #[test]
    fn test_get_set_bc() {
        let mut registros = RegistrosCPU::new();
        registros.set_bc(0x1234);
        assert_eq!(registros.get_bc(), 0x1234);
    }

    #[test]
    fn test_get_set_de() {
        let mut registros = RegistrosCPU::new();
        registros.set_de(0xABCD);
        assert_eq!(registros.get_de(), 0xABCD);
    }

    #[test]
    fn test_get_set_hl() {
        let mut registros = RegistrosCPU::new();
        registros.set_hl(0x5678);
        assert_eq!(registros.get_hl(), 0x5678);
    }
}

//*****************************************************************************