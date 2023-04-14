/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          14-04-2023
    Titulo:         Funciones de manejo de registros y flags - Simulación CPU
    Descripción:    
    Referencias:

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]

/* Registro Flags (banderas)                
                                                    Bits    7	6	5	4	3	2	1	0
                                                    Flags   S	Z	0	H	0	P	1	C
Estas son las banderas que se utilizan en el Z80:
-   Bit 0 (C o CF) Carry Flag (bandera de acarreo): Indica si se produjo un acarreo en la
    operación aritmética (no se usa en las operaciones lógicas). Se establece en 1 cuando se
    produce un acarreo en la operación, de lo contrario, se establece en 0.
-   Bit 1: siempre está en 1.
    Nota para el Z80:
    Bit 1 (N): Add/Subtract flag (bandera de suma/resta): Esta bandera se establece en 1 si la
    última operación realizada fue una resta. De lo contrario, se establece en 0.
-   Bit 2 (P o PF) Parity Flag (bandera de paridad/sobreflujo): Indica si el resultado de la
    operación tiene un número par o impar de bits. Se establece en 1 cuando si el resultado
    contiene un número par de unos, de lo contrario, se establece en 0.
    Nota para el Z80 (PV):
    Parity/Overflow flag Esta bandera también se utiliza para indicar si se ha producido un
    error en una operación que involucra números con signo.
-   Bit 3: siempre está en 0.
-   Bit 4 (H o AC) Half Carry Flag o Auxiliary Carry Flag (bandera de medio acarreo): Indica
    si se produjo un acarreo en el bit 3 de la operación aritmética (no se usa en las operaciones
    lógicas). El AC se establece en 1 cuando se produce un acarreo en el bit 3 de la operación,
    de lo contrario, se establece en 0.
-   Bit 5: siempre está en 0.

-   Bit 6 (Z o ZF) Zero Flag (bandera de cero): Indica si el resultado de la operación es cero.
    Esta bandera se establece en 1 si el resultado de una operación es cero, de lo contrario, se
    establece en 0.
-   El bit 7 (S o SF) Sign Flag (bandera de signo): Indica si el resultado de la operación es
    negativo. Esta bandera se establece en 1 si el resultado de una operación es negativo (el bit
    más significativo es 1), de lo contrario, se establece en 0.
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

    pub fn get_bit(&self, index: u8) -> u8 {
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

/* Implementación ALU                       
Las operaciones que utiliza la ALU en el procesador Intel 8080 incluyen:

    ADD: Suma el valor especificado con el valor actual en el registro A.
    ADC: Suma el valor especificado y la bandera de acarreo con el valor actual en el registro A.
    SUB: Resta el valor especificado del valor actual en el registro A.
    SBB: Resta el valor especificado y la bandera de acarreo del valor actual en el registro A.
    AND: Realiza una operación lógica AND entre el valor especificado y el valor actual en el registro A.
    OR: Realiza una operación lógica OR entre el valor especificado y el valor actual en el registro A.
    XOR: Realiza una operación lógica XOR entre el valor especificado y el valor actual en el registro A.
    CMP: Compara el valor especificado con el valor actual en el registro A, actualizando las banderas según corresponda.
*/

//***************************************************************************** cálculos de flags individuales
    pub fn flags_acarreo_add(&mut self, val_reg_a: u8, val_reg_x: u8) -> u8 {
        // suma sin propagación de acarreo
        let (resultado, acarreo) = val_reg_a.overflowing_add(val_reg_x);
        if acarreo == true {self.set_bit(0, true)} else {self.set_bit(0, false)}
        resultado
    }

    pub fn flags_paridad(&mut self, val_reg_a: u8) {
        // Método con función interna:
        // println!("Paridad: {}", val_reg_a.count_ones() % 2 == 0);
        // Método manipulando bits
        let mut count = 0;
        for i in 0..8 {
            if (val_reg_a & (1 << i)) != 0 { count += 1; }
        }
        if count % 2 == 0 {self.set_bit(2, true)} else {self.set_bit(2, false)}
    }

    /* (result & 0x0F) + (carry as u8) > 0x0F;
    La instrucción se divide en dos partes:
        (result & 0x0F) + (carry as u8)
        > 0x0F
    La primera parte de la instrucción está sumando dos valores:
    ->  (result & 0x0F): Este es el resultado de aplicar una operación AND entre el valor result y la máscara
        de bits 0x0F. La máscara de bits se utiliza para eliminar todos los bits que no estén en las posiciones
        0 a 3. Esto significa que solo se sumarán los 4 bits menos significativos del valor de result.
    ->  (carry as u8): Este valor representa el valor del acarreo (carry) de una operación anterior. En este
        caso, la instrucción está tratando de determinar si hubo un acarreo de la suma anterior, lo que podría
        indicar que hay un bit de overflow en la operación actual.
    La segunda parte de la instrucción está realizando una comparación:
    ->  > 0x0F: Esta parte de la instrucción está comparando la suma anterior con el valor 0x0F. 0x0F representa
        el valor decimal 15 en hexadecimal. Si la suma anterior es mayor que 0x0F, entonces la instrucción devuelve
        el valor booleano true, lo que indica que hubo un acarreo.
    En resumen, esta instrucción está tratando de determinar si hubo un acarreo en la operación anterior de suma,
    al evaluar los 4 bits menos significativos del valor resultante de la suma y el valor del acarreo. Si la suma
    de ambos valores supera el valor de 15, entonces se devuelve true, lo que indica que hubo un acarreo.
    */
    pub fn flags_acarreo_auxiliar(&mut self, val_reg_a: u8, val_reg_x: u8) {
        if (val_reg_a & 0x0F) + (val_reg_x & 0x0F ) > 0x0F { self.set_bit(4,true)} else {self.set_bit(4, false)}
    }

    pub fn flags_cero(&mut self, val_reg_a: u8) {
        if val_reg_a == 0 {self.set_bit(6, true)} else {self.set_bit(6, false)}
    }

    pub fn flags_signo(&mut self, val_reg_a: u8) {
        if (val_reg_a & 0x80) == 0x80 {self.set_bit(7, true)} else {self.set_bit(7, false)}
    }

//*****************************************************************************
    pub fn add(&mut self, val_reg_a: u8, val_reg_x: u8, cf: bool, test: bool) -> u8 {
        let val_acarreo:u8 = self.get_bit(0);
        let (resultado, acarreo) = val_reg_a.overflowing_add(val_reg_x);

        if acarreo == true && cf == true {self.set_bit(0, true)}
            else if val_acarreo == 0 {self.set_bit(0, false)}


        if resultado.count_ones() % 2 == 0 {self.set_bit(2, true)} else {self.set_bit(2, false)}
        if (val_reg_a & 0x0F) + (val_reg_x & 0x0F) > 0x0F { self.set_bit(4,true)} else {self.set_bit(4, false)}
        if resultado == 0 {self.set_bit(6, true)} else {self.set_bit(6, false)}
        if (resultado & 0x80) == 0x80 {self.set_bit(7, true)} else {self.set_bit(7, false)}
        if test {println!("Reg A: {:08b}, Reg X: {:08b}, Resultado-> Reg A: {:08b}, Flags  : {:08b}, Acarreo: {}\n"
            , val_reg_a
            , val_reg_x
            , resultado
            , self.get_flags_1()
            , acarreo);};
        return resultado;
    }

    pub fn adc(&mut self, val_reg_a: u8, val_reg_x: u8, test: bool ) -> u8 {
        let val_acarreo:u8 = self.get_bit(0);
        if test {println!("Acarreo OP anterior   : {}", val_acarreo);};
        let (resultado_intermedio, acarreo_intermedio) = val_reg_a.overflowing_add(val_reg_x);
        if test {println!("ADC Reg a + Reg B     : {:08b}, Flags HC de Reg A + Reg X             : {}",resultado_intermedio, acarreo_intermedio);}
        let (resultado, acarreo) = resultado_intermedio.overflowing_add(val_acarreo);
        if test {println!("ADC Reg a + Reg B + C : {:08b}, Flags HC de Resultado + CF OP anterior: {}",resultado, acarreo);}
        if acarreo == true || acarreo_intermedio {self.set_bit(0, true)} else {self.set_bit(0, false)}
        if resultado.count_ones() % 2 == 0 {self.set_bit(2, true)} else {self.set_bit(2, false)}

        if ((val_reg_a & 0x0F) + (val_reg_x & 0x0F)) > 0x0f || ((resultado_intermedio & 0x0F) + (val_acarreo & 0x0F)) > 0x0F {
             self.set_bit(4,true)} else {self.set_bit(4, false) }

        if resultado == 0 {self.set_bit(6, true)} else {self.set_bit(6, false)}
        if (resultado & 0x80) == 0x80 {self.set_bit(7, true)} else {self.set_bit(7, false)}
        if test {println!("Reg A: {:08b}, Reg X: {:08b}, Resultado-> Reg A: {:08b}, Flags    : {:08b}, Acarreo: {}\n"
            , val_reg_a
            , val_reg_x
            , resultado
            , self.get_flags_1()
            , self.get_bit_1(0));};
        return resultado;
    }


}

//***************************************************************************** Estructura e implementación Registros
pub struct RegistrosCPU {
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
    sp: u16,            // Registro SP de 16 bits
    pc: u16,            // Registro PC de 16 bits
}

impl RegistrosCPU {
    pub fn new() -> RegistrosCPU {
        RegistrosCPU {
            flags: Flags { 
                carry: false,
                subtract: false,
                parity_overflow: false,
                half_carry: false,
                zero: false,
                sign: false, },
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




//***************************************************************************** Test manejo de bit de flags y calculo individual
#[cfg(test)]
mod tests_0 {
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
        flags.flags_acarreo_auxiliar(0x1F, 0x01);
        assert_eq!(flags.half_carry, true);
        
        flags.flags_acarreo_auxiliar(0x08,0x06);
        assert_eq!(flags.half_carry, false);

        flags.flags_acarreo_auxiliar(0xAF, 0x0F);
        assert_eq!(flags.half_carry, true);

        flags.flags_acarreo_auxiliar(0x1B, 0x01);
        assert_eq!(flags.half_carry, false);

        flags.flags_acarreo_auxiliar(0x9F, 0x01);
        assert_eq!(flags.half_carry, true);

        flags.flags_acarreo_auxiliar(0x0F, 0x01);
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

//***************************************************************************** Test manejo de flags - ALU
#[cfg(test)]
mod tests_1 {
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

}

//*****************************************************************************
// Manejo de bit en un byte con operaciones lógicas y desplazamientos
/*
pub fn get0(){
    // La expresión (1<<0) es una operación de desplazamiento a la izquierda de un bit, que se utiliza para mover
    // el valor binario 1 cero posiciones a la izquierda.
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