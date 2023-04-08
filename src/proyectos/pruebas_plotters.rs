use plotters::prelude::*;



fn draw_pin_timings1(pins: &[u8], timings: &[u16]) {
    let mut last_timing = timings[0];
    for i in 0..pins.len() {
        let timing = timings[i + 1];
        let duration = timing - last_timing;
        last_timing = timing;

        let pin_str = format!("{:02X}", pins[i]);
        let timing_str = format!("{:04X}", duration);

        println!("Pin {} changed after {} clock cycles", pin_str, timing_str);
    }
}


// Datos de ejemplo para las fluctuaciones en los pines de control del Intel 8080
const INTEL_8080_CONTROL_PIN_FLUCTUATIONS: [(i32, i32); 7] = [
    (1, 0), (2, 0), (3, 0), (4, 1), (5, 1), (6, 0), (7, 0)
];

pub fn pru_ploter() -> Result<(), Box<dyn std::error::Error>> {
    // Simulamos algunos cambios en los pines y los tiempos en ciclos de reloj
    let pins = [0x01, 0x03, 0x07, 0x0F];
    let timings = [0x0000, 0x0012, 0x0025, 0x003A, 0x0051];

    // Dibujamos los tiempos de los pines
    draw_pin_timings1(&pins, &timings);



        // Crea un nuevo gráfico en un archivo PNG
        let root = BitMapBackend::new("intel_8080_control_pin_fluctuations.png", (640, 480)).into_drawing_area();
    
        // Agrega un título y una descripción al gráfico
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Fluctuaciones en los pines de control del Intel 8080", ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..8, 0..2)?;
    
        // Agrega una cuadrícula detrás de los datos del gráfico
        chart.configure_mesh().draw()?;
    
        // Dibuja los datos de las fluctuaciones en los pines de control del Intel 8080
        chart.draw_series(LineSeries::new(INTEL_8080_CONTROL_PIN_FLUCTUATIONS.iter().map(|(x, y)| (*x, *y)), &BLUE))?;
    
        Ok(())
}


//*****************************************************************************
pub fn pru_ploter1() {
    let mut pins: [i32; 41] = [0; 41];
    let mut time: i32 = 0;

    // Ciclo de instrucción básico del Intel 8080
    pins[1] = 1;
    pins[3] = 1;
    pins[5] = 1;
    pins[9] = 1;
    pins[10] = 1;
    pins[11] = 1;
    pins[19] = 1;
    pins[23] = 1;
    pins[28] = 1;
    pins[34] = 1;
    pins[39] = 1;
    time += 10;

    pins[37] = 1;
    time += 5;

    pins[37] = 0;
    pins[36] = 1;
    time += 5;

    pins[36] = 0;
    pins[31] = 1;
    time += 5;

    pins[31] = 0;
    pins[21] = 1;
    time += 5;

    pins[21] = 0;
    pins[18] = 1;
    time += 5;

    pins[18] = 0;
    pins[16] = 1;
    time += 5;

    pins[16] = 0;
    pins[14] = 1;
    time += 5;

    pins[14] = 0;
    pins[12] = 1;
    time += 5;

    pins[12] = 0;
    pins[11] = 0;
    time += 10;

    // Graficar las fluctuaciones en los pines de control
    let root = BitMapBackend::new("intel8080.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Ciclo de instrucción básico del Intel 8080", ("sans-serif", 30))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..50, 0..2)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(2)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..40).map(|i| (i as i32, pins[i])),
            &RED,
        ))
        .unwrap();

    root.present().unwrap();
}


//*****************************************************************************
const CLK_FREQ: f64 = 2.0e6;
const CYCLE_TIME: f64 = 1.0 / CLK_FREQ;
const INSTRUCTION_TIME: f64 = 10.0 * CYCLE_TIME;

pub fn pru_ploter2() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Intel 8080 Instruction Cycle", ("sans-serif", 30))
        .build_cartesian_2d(0..800, 0..600)?;

    // Draw clock signal
    let mut clock_signal: Vec<(i32, i32)> = Vec::new();
    for i in 0..800 {
        let time = (i as f64 * CYCLE_TIME).round() as i32;
        let value = ((i / 100) % 2) as i32;
        clock_signal.push((time, value));
    }
    chart
        .draw_series(LineSeries::new(clock_signal, &BLUE))?
        .label("CLK");

    // Draw instruction cycle
    let mut instruction_cycle: Vec<(i32, i32)> = Vec::new();
    for i in 0..800 {
        let time = (i as f64 * CYCLE_TIME).round() as i32;
        let value = match i % 10 {
            0 => 0,
            1..=4 => 1,
            5..=7 => 0,
            8 => 1,
            9 => 0,
            _ => unreachable!(),
        };
        instruction_cycle.push((time, value));
    }
    chart
        .draw_series(LineSeries::new(instruction_cycle, &GREEN))?
        .label("Instruction Cycle");

    // Draw control signals
    let mut control_signals: Vec<(i32, i32)> = Vec::new();
    for i in 0..800 {
        let time = (i as f64 * CYCLE_TIME).round() as i32;
        let value = match i % 10 {
            0 => 1,
            1 => 0,
            2 => 1,
            3 => 1,
            4 => 1,
            5 => 1,
            6 => 0,
            7 => 0,
            8 => 1,
            9 => 0,
            _ => unreachable!(),
        };
        control_signals.push((time, value));
    }
    chart
        .draw_series(LineSeries::new(control_signals, &RED))?
        .label("Control Signals");

    // Add legend
    chart.configure_series_labels().draw()?;

    Ok(())
}


//*****************************************************************************
/*
// Constantes para los pines de control del Intel 8080
const PIN_ALE: u8 = 1 << 0;
const PIN_RD: u8 = 1 << 1;
const PIN_WR: u8 = 1 << 2;
const PIN_M1: u8 = 1 << 3;
const PIN_IO: u8 = 1 << 4;

// Datos de ejemplo para las fluctuaciones en los pines de control del Intel 8080
/* Definidas previamente
const INTEL_8080_CONTROL_PIN_FLUCTUATIONS: [(u32, u32); 7] = [
    (1, 0), (2, 0), (3, 0), (4, 1), (5, 1), (6, 0), (7, 0)
];
*/

pub fn pru_ploter3() -> Result<(), Box<dyn std::error::Error>> {
    // Crea un nuevo gráfico en un archivo PNG
    let root = BitMapBackend::new("intel_8080_control_pin_fluctuations.png", (640, 480)).into_drawing_area();

    // Agrega un título y una descripción al gráfico
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Fluctuaciones en los pines de control del Intel 8080", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..8, 0..2)?;

    // Agrega una cuadrícula detrás de los datos del gráfico
    chart.configure_mesh().draw()?;

    // Crea una nueva instancia del procesador Intel 8080
    let mut cpu = Intel8080::new();

    // Ejecuta un ciclo de instrucción básico
    loop {
        // Lee la instrucción actual de la memoria
        let opcode = cpu.read_byte(cpu.registers.pc)?;

        // Decodifica y ejecuta la instrucción
        let cycles = match opcode {
            // Instrucción NOP
            0x00 => {
                // Fluctuaciones de los pines de control del Intel 8080 durante un ciclo de instrucción NOP
                cpu.control_pins = PIN_ALE | PIN_RD | PIN_WR;
                4
            },

            // Instrucción LXI B, D16
            0x01 => {
                // Leer los datos del inmediato de 16 bits
                let lo = cpu.read_byte(cpu.registers.pc + 1)?;
                let hi = cpu.read_byte(cpu.registers.pc + 2)?;
                let data = u16::from_le_bytes([lo, hi]);

                // Cargar los datos en los registros B y C
                cpu.registers.b = hi;
                cpu.registers.c = lo;

                // Fluctuaciones de los pines de control del Intel 8080 durante un ciclo de instrucción LXI B, D16
                cpu.control_pins = PIN_ALE | PIN_WR | PIN_RD;
                10
            },
            // Instrucción STAX B
            0x02 => {
                // Escribir el contenido del registro A en la dirección apuntada por los registros B y C
                let addr = u16::from_be_bytes([cpu.registers.b, cpu.registers.c]);
                cpu.write_byte(addr, cpu.registers.a)?;

                // Fluctuaciones de los pines de control del Intel 8080 durante un ciclo de instrucción STAX B
                cpu.control_pins = PIN_ALE | PIN_WR | PIN_IO;
                7
            },

            // Instrucción INX B
            0x03 => {
                // Incrementar los registros B y C
                let bc = u16::from_be_bytes([cpu.registers.b, cpu.registers.c]);
                let bc = bc.wrapping_add(1);
                cpu.registers.b = (bc >> 8) as u8;
                cpu.registers.c = (bc & 0xff) as u8;

                // Fluctuaciones de los pines de control del Intel 8080 durante un ciclo de instrucción INX B
                cpu.control_pins = PIN_ALE | PIN_WR | PIN_RD;
                5
            },

            // ... Otras instrucciones ...

            // Instrucción HLT
            0x76 => {
                // Detener la ejecución del programa
                break;
            },

            // Instrucción desconocida
            _ => {
                return Err(Box::new(InvalidOpcodeError(opcode)));
            }
        };

        // Agrega las fluctuaciones en los pines de control del Intel 8080 al gráfico
        for (x, y) in INTEL_8080_CONTROL_PIN_FLUCTUATIONS.iter() {
            chart.draw_series(LineSeries::new(
                vec![(*x, *y)],
                &BLACK,
            ))?;
        }

        // Esperar el número de ciclos de reloj necesarios para completar la instrucción
        cpu.clock += cycles;
    }

    Ok(())
}
*/