/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          01-05-2023
    Titulo:         pruebas de minifb
    Descripción:    
    Referencias:    https://github.com/emoon/rust_minifb

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use minifb::{Key, KeyRepeat, Window, WindowOptions};

// Definir la estructura Gate
#[derive(Clone, Copy)]
struct Gate {
    input1: bool,
    input2: bool,
    output: bool,
    gate_type: GateType,
}

#[derive(Clone, Copy)]
enum GateType {
    And,
    Or,
    Not,
    Xor,
}

impl Gate {
    // Método para calcular la salida de la compuerta
    fn calculate_output(&mut self) {
        match self.gate_type {
            GateType::And => self.output = self.input1 && self.input2,
            GateType::Or => self.output = self.input1 || self.input2,
            GateType::Not => self.output = !self.input1,
            GateType::Xor => self.output = self.input1 ^ self.input2,
        }
    }
}

pub fn gate_bool() {
    // Definir una red de compuertas
    let mut gates: Vec<Gate> = vec![
        Gate { input1: true, input2: true, output: false, gate_type: GateType::And },
        Gate { input1: false, input2: true, output: false, gate_type: GateType::And },
        Gate { input1: true, input2: false, output: false, gate_type: GateType::And },
        Gate { input1: false, input2: false, output: false, gate_type: GateType::And },
        Gate { input1: false, input2: false, output: false, gate_type: GateType::Or },
    ];

    // Conectar las compuertas en una red lógica
    gates[0].output = true;
    gates[1].input1 = gates[0].output;
    gates[2].input2 = gates[0].output;
    gates[3].input1 = gates[1].output;
    gates[3].input2 = gates[2].output;
    gates[4].input1 = gates[3].output;
    gates[4].input2 = gates[3].output;

    // Configurar la ventana de la aplicación
    let mut buffer: Vec<u32> = vec![0; 640 * 480];
    let mut window = Window::new(
        "Simulación de álgebra booleana",
        640,
        480,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Bucle principal de la simulación
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calcular la salida de todas las compuertas
        for gate in gates.iter_mut() {
            gate.calculate_output();
        }

        // Dibujar el estado de la salida en la ventana de la aplicación
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = i as u32 % 640;
            let y = i as u32 / 640;

            if x < 100 && y < 100 {
                *pixel = if gates[4].output { 0xffffff } else { 0x000000 };
            } else {
                *pixel = 0x000000;
            }
        }

        // Actualizar la ventana de la aplicación
        window.update_with_buffer(&buffer, 600, 480).unwrap();

        // Esperar 10 milisegundos para darle tiempo a la ventana de la aplicación de responder
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

//***************************************************************************** 
