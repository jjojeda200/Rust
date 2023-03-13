/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          13-03-2023
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
/* Requisitos                               
Para tener una emulación completa del Z80, se deben agregar muchas más funcionalidades a
la implementación básica que proporcioné anteriormente. Se enumeran algunas de las cosas
que se deben agregar:

    Gestión de interrupciones: El Z80 tiene un sistema de interrupciones muy flexible, y
    la emulación debe ser capaz de manejarlas correctamente. En el bucle de ciclo de reloj,
    se debe agregar una comprobación regular de si hay alguna interrupción pendiente y
    manejarlas según corresponda.

    Acceso a dispositivos de E/S: Para emular un sistema completo, la emulación debe ser
    capaz de interactuar con dispositivos de entrada y salida, como teclados, pantallas
    y unidades de disco. Esto puede requerir el uso de bibliotecas de Rust específicas para
    cada dispositivo.

    Implementación de las instrucciones del Z80: La emulación debe ser capaz de implementar
    todas las instrucciones del Z80, incluyendo las instrucciones de carga, aritméticas,
    lógicas y de salto, entre otras.

    Implementación de los registros y banderas del Z80: La emulación debe ser capaz de
    gestionar todos los registros del Z80, incluyendo el contador de programa (PC), el
    puntero de pila (SP) y los registros de propósito general (A, B, C, D, E, H, L).
    También debe ser capaz de gestionar las banderas de estado (carry, zero, sign, etc.)
    que se establecen según el resultado de ciertas instrucciones.

    Manejo de la memoria: La emulación debe ser capaz de gestionar la memoria del sistema,
    incluyendo la RAM y la ROM, así como la memoria de vídeo y otros dispositivos de memoria
    específicos.

    Depuración: Es muy útil tener herramientas de depuración para la emulación, como la
    capacidad de detener el ciclo de reloj en cualquier momento y examinar el estado del
    sistema en ese punto. Esto puede requerir la implementación de una interfaz de usuario
    y otras funcionalidades relacionadas.
*/
#![allow(dead_code)]
#![allow(unused_variables)]

fn imprime_titulo(titulo: &String) {
    println!("\n{:*^80}", titulo);
}

//***************************************************************************** MemoryUnit
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
struct Memoria {
    data: Vec<u8>, // Cambio a un vector dinámico
}

impl Memoria {
    fn new(size: usize) -> Self {
        Memoria { data: vec![0; size] }
    }
}

/* Estructura Z80Reg que almacena los registros de la CPU Z80
Se han definido los siguientes campos para almacenar los diferentes registros
    a:          el acumulador
    f:          el registro de banderas
    b y c:      los registros de propósito general BC
    d y e:      los registros de propósito general DE
    h y l:      los registros de propósito general HL
    pc:         el contador de programa
    sp:         el puntero de pila
    ix:         el índice extendido
    iy:         el índice extendido secundario
    i:          el registro I de interrupción
    r:          el registro R de refresco
    int_enable: habilitar o deshabilitar las interrupciones
    clock:      control de ciclos de reloj
    mem:        estructura para la memoria
*/
struct Z80CPU {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,
    i: u8,
    r: u8,
    interrupcion_enable: bool,
    clock: u64,
    mem: Memoria,   // Utiliza la estructura Memory en lugar de un puntero
}

impl Z80CPU {
    fn new(mem_size: usize) -> Self {
        Z80CPU {
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0xffff,
            ix: 0x0000,
            iy: 0x0000,
            i: 0xff,
            r: 0x80,
            interrupcion_enable: true,
            clock: 0,
            // Crear una instancia de la estructura Memory
            mem: Memoria::new(mem_size),
        }
    }

    fn add_clock(&mut self, cycles: u64) {
        self.clock += cycles;
    }

    // Función para leer un byte de memoria
    fn read_byte(&self, addr: u16 ) -> u8 {
        self.mem.data[addr as usize]
    }

    // Función para escribir un byte en memoria
    fn write_byte(&mut self, addr: u16, value: u8) {
        self.mem.data[addr as usize] = value;
    }


                // Cargar un valor de memoria en un registro
                fn ld_r_n(&mut self, r: &mut u8) {
                    *r = self.read_byte(self.pc);
                    self.pc += 1;
                    self.add_clock(3);
                }
            
                // Sumar dos registros y almacenar el resultado en el primer registro
                fn add_rr(&mut self, r1: &mut u8, r2: u8) {
                    let result = *r1 as u16 + r2 as u16;
                    *r1 = result as u8;
                    self.f = 0x00;
                    if result > 0xFF {
                        self.f |= 0x10; // Carry flag
                    }
                    if *r1 == 0 {
                        self.f |= 0x80; // Zero flag
                    }
                    self.add_clock(4);
                }
            
                // Sumar dos registros de 16 bits y almacenar el resultado en el primer registro
                fn add_rr_rr(&mut self, r1: &mut u16, r2: u16) {
                    let result = *r1 as u32 + r2 as u32;
                    *r1 = result as u16;
                    self.f &= 0x80;
                    if result > 0xFFFF {
                        self.f |= 0x10; // Carry flag
                    }
                    self.add_clock(11);
                }
            
                // Cargar un valor inmediato en el registro A
                fn ld_a_n(&mut self) {
                    self.a = self.read_byte(self.pc);
                    self.pc += 1;
                    self.add_clock(2);
                }
            
                // // Sumar un valor inmediato al registro A
                // fn add_a_n(&mut self) {
                //     let n = self.read_byte(self.pc);
                //     self.pc += 1;
                //     self.add_rr(&mut self.a, n);
                // }
            
                // // Cargar un valor de memoria en el registro A
                // fn ld_a_hl(&mut self) {
                //     let addr = (self.h as u16) << 8 | self.l as u16;
                //     self.a = self.read_byte(addr);
                //     self.add_clock(3);
                // }

}

pub fn z80_sim_0(){
    let titulo = String::from(" Z80 - Simulación CPU - Aproximación de pruebas 0");
    imprime_titulo(&titulo);

    // Crear una instancia de la estructura CPU con 64 KB de memoria
    let mut cpu = Z80CPU::new(64 * 1024); 

    // Escribir un valor en la dirección 0x1000
    cpu.write_byte(0x1000, 0xFF);

    // Leer el valor de la dirección 0x1000
    let value = cpu.read_byte(0x1000);
    println!("Valor en la dirección 0x1000: 0x{:02X}", value);

    let a_value = cpu.a;
    println!("Valor del registro a: 0x{:02X}", a_value);

    // Ejecutar una instrucción de la CPU que tarda 4 ciclos de reloj
    cpu.add_clock(4);

    // Imprimir el número de ciclos de reloj transcurridos
    println!("Ciclos de reloj transcurridos: {}", cpu.clock);


/* Simular ciclos de reloj                  
Simular ciclos de reloj del z80
Una parte importante de su funcionamiento es la ejecución de ciclos de reloj, que controlan
el acceso a la memoria y la realización de instrucciones.

Se puede simular la ejecución de ciclos de reloj del Z80 utilizando un bucle que realiza las
siguientes operaciones en cada ciclo:
    Leer la instrucción en la dirección del programa actual (PC).
    Incrementar el contador de programa (PC).
    Decodificar la instrucción.
    Ejecutar la instrucción.
    Repetir.

    loop {    // Iniciar el ciclo de reloj
        // Leer la instrucción en la dirección del programa actual
        let instruction = memory[pc as usize];

        // Incrementar el contador de programa
        if pc == 0xfffe {
            pc = 0;
        }
        pc += 1;

        // Decodificar y ejecutar la instrucción
        match instruction {
            // TODO: implementar todas las instrucciones del Z80
            _ => println!("Instrucción no implementada: {:#X}", instruction),
        }

        // Repetir
    }
*/


}


/*
Puedes Implementar las instrucciones 

Un ejemplo de cómo se podría implementar una instrucción específica del Z80:
    la instrucción "ADD A,n", que suma el valor inmediato "n" al registro
    acumulador A:

// Se asume que los registros del Z80 se almacenan en una estructura llamada Z80Registers
// y que el registro A se puede acceder como z80_regs.a

// Opcode para ADD A,n
const ADD_A_N_OPCODE: u8 = 0xC6;

// Función para manejar la instrucción ADD A,n
fn add_a_n(regs: &mut Z80Registers, memory: &[u8]) -> u8 {
    let n = memory[regs.pc + 1];        // Obtener el valor inmediato "n" de la memoria
    regs.a = regs.a.wrapping_add(n);    // Sumar "n" al registro A, con manejo de desbordamiento
    2                                   // La instrucción toma 2 ciclos de reloj
}

// Función para decodificar y ejecutar una instrucción del Z80
fn execute_instruction(regs: &mut Z80Registers, memory: &[u8]) -> u8 {
    let opcode = memory[regs.pc];
    match opcode {
        ADD_A_N_OPCODE => add_a_n(regs, memory),
        // Otras instrucciones del Z80 aquí...
        _ => unimplemented!(),  // Instrucción no implementada todavía
    }
}

// Ejemplo de cómo usar la función execute_instruction para ejecutar una serie de instrucciones
fn main() {
    let mut regs = Z80Registers::default();
    let memory = vec![ADD_A_N_OPCODE, 0x42,  // ADD A,0x42
                      ADD_A_N_OPCODE, 0x10,  // ADD A,0x10
                      ADD_A_N_OPCODE, 0xFF]; // ADD A,0xFF (prueba de desbordamiento)
    let mut cycles = 0;
    while regs.pc < memory.len() {
        let cycles_for_instruction = execute_instruction(&mut regs, &memory);
        cycles += cycles_for_instruction;
        regs.pc += 1;  // Avanzar el contador de programa a la siguiente instrucción
    }
    println!("Valor final de A: {}", regs.a);
    println!("Ciclos totales: {}", cycles);
}

En realidad, cada instrucción tendría su propia función de manejo, también habría
que implementar la decodificación de los diferentes modos de direccionamiento
(inmediato, directo, indirecto, etc.). También habría que manejar correctamente las
banderas de estado y otros efectos secundarios de cada instrucción.

*/