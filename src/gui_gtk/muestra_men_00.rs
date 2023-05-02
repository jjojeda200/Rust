/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          29-04-2023
    Titulo:         introducción a RUST y GTK3
    Descripción:    
                    
    Referencias:
    https://doc.rust-lang.org/beta/rust-by-example/index.html
    https://www.jmgaguilera.com/rust_facil/actualizaciones.html
    https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/all.html

    https://docs.rs/gtk/0.16.2/gtk/
    https://github.com/gtk-rs/gtk3-rs

    Dependencias:
    GTK

***************************************************************************************/
#![allow(dead_code)]
#![allow(unused_imports)]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, TextView, TextBuffer, TextTagTable};
use crate::proyectos::sim_cpu_memoria;
use crate::proyectos::{sim_cpu_memoria::BancosMemoria, sim_cpu_registros::{self, CPU}};
//use std::any::type_name;
use crate::proyectos::sim_cpu_pruebas;

const COL_POR_DEFECTO: usize = 16;

fn calcula_lineas(size: usize, ancho: usize) -> usize {
    let lineas = size / ancho;
    if (size & 0xf) != 0 {
        return lineas + 1;
    }
    lineas
}

fn muestra_linea_mem(mem: &[u8], ancho: usize) -> String {
    let mut linea = format!("{:16p} ||", mem.as_ptr());
    for i in 0..ancho {
        linea.push_str(&format!(" {:02x}", mem[i]));
    }
    linea.push_str(" \n");
    linea
}

fn muestra_mem(mem: &[u8], size: usize, ancho: usize) -> String {
    let mut salida = format!("--> Tamaño ocupado en bytes ({})\n", mem.len());
    salida.push_str("   Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n");
    salida.push_str("  -------------- || -----------------------------------------------\n");
    let lineas = calcula_lineas(size, ancho);
    let mut offset = 0;
    for _ in 0..lineas {
        //println!("mem {:?} {:p}", &mem, mem);
        salida.push_str(&muestra_linea_mem(&mem[offset..], ancho));
        offset += ancho;
    }
    salida.push_str("  -----------------------------------------------------------------\n");
    salida
}


fn build_ui(application: &gtk::Application) {
    let mut cpu = CPU::new();
    let mut aux = sim_cpu_pruebas::Aux {imp_contador_programa: 0x0, imp_instruccion: 0x0, imp_mnemonico: String::new()};
    let programa = vec![
        0x00,               // NOP
        0x3E, 0x04,         // Almacenar el valor 0x04 en el Registro A
        0x06, 0x0a,         // Almacenar el valor 0x0a en el Registro B
        0x04,               // Incrementa Registro B
        0x80,               // Suma el contenido del Registro B al Registro A
        0x00,               // NOP
        0x3E, 0xf0,         // Almacenar el valor 0xf0 en el Registro A
        0x06, 0x0f,         // Almacenar el valor 0x0f en el Registro B
        0x80,               // Suma el contenido del Registro B al Registro A
        0x00,               // NOP
        0x3E, 0x3b,         // Almacenar el valor 0x3b en el Registro A
        0x3C,               // Incrementa Registro A
        0x32, 0x15, 0x00,   // Mueve el contenido de A a la dirección indicada por los dos bytes siguientes 
        0x00, 0x00,         // <-- Se cambio el contenido y se convierte en 3C
        0x3A, 0x0b, 0x00,   // Mueve el contenido (0x0f) de la dirección indicada (0x0b) en los dos bytes siguientes a A 
        0x00, 0x00,
        0x06, 0xff,         // Almacenar el valor 0xff en el Registro B
        0x80,               // Suma el contenido del Registro B al Registro A
        0x00, 0x00,
        0xFF,
        0xC3, 0x00, 0x00,   // Salta a la dirección 0x0000
        0xFF, 0xFF,         // Marca fin de programa
    ];
    cpu.cargar_programa(&programa);
    cpu.run_no_win(&mut  aux);


    let ventana = ApplicationWindow::new(application);
    ventana.set_title("Muestra Memoria - GTK Rust");
    ventana.set_position(gtk::WindowPosition::Center);
    //ventana.set_default_size(420, 400);
    ventana.set_default_width(400);
    ventana.set_default_height(500);
    ventana.set_border_width(10);
    ventana.set_resizable(false);

    let ventana_opcode = gtk::Window::new(gtk::WindowType::Toplevel);
    ventana_opcode.set_title("OPCode");
    ventana_opcode.set_default_size(400, 200);
    ventana_opcode.set_border_width(10);
    ventana_opcode.set_resizable(false);

    let text_view_opcode_0 = TextView::new();
    text_view_opcode_0.set_monospace(true);
    text_view_opcode_0.set_editable(false);
    text_view_opcode_0.set_cursor_visible(false);


    let etiqueta_opcode  = Label::new(None);
    let caja_opcode_0 = Box::new(gtk::Orientation::Vertical, 8);
    caja_opcode_0.set_margin(8);
    let caja_opcode_1 = Box::new(gtk::Orientation::Horizontal, 8);
    caja_opcode_1.set_margin(4);

    ventana_opcode.add(&caja_opcode_0);
    caja_opcode_0.pack_start(&text_view_opcode_0, true, true, 4);
    caja_opcode_0.pack_end(&etiqueta_opcode, true, true, 8);

    let eventos = ventana_opcode.get_events();
    for event in eventos {
        match event {
            // Si se presiona la tecla "q", salir de la aplicación
            Event::KeyPress { keyval, .. } => {
                if keyval == 113 {
                    ventana_opcode.destroy();
                }
            }
            _ => (),
        }
    }


    // Crea cajas
    let caja0 = Box::new(gtk::Orientation::Vertical, 8);
    caja0.set_margin(8);
    let caja1 = Box::new(gtk::Orientation::Horizontal, 8);
    caja1.set_margin(4);

    // Crea botones y etiquetas
    let etiqueta = Label::new(None);
    let boton00 = Button::with_label("Botón 0");
    let boton01 = Button::with_label("Botón 1");
    let boton10 = Button::with_label("Reserva Memoria");
    let boton11 = Button::with_label("Ventana OPCode");

    // Crea vistas de texto (Tabla Hex)
    let text_view = TextView::new();
    text_view.set_monospace(true);
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);

    // Crea vistas de texto (Salida botones 0 y 1)
    let cont_bufer_01 = TextView::new();
    cont_bufer_01.set_editable(false);
    cont_bufer_01.set_cursor_visible(false);


    // Creamos el bufer's de texto
    let bufer_00 = TextBuffer::new(Some(&TextTagTable::new()));
    let bufer_01 = cont_bufer_01.buffer().unwrap();
    bufer_01.set_text("Texto de ejemplo");

    let _bufer1 = TextBuffer::new(Some(&TextTagTable::new()));
    let _bufer2 = TextBuffer::new(Some(&TextTagTable::new()));


    //let mut memoria = BancosMemoria::new();
    //cpu.memoria.escribir_memoria(0x0000, 0xff);
    //cpu.memoria.escribir_memoria(0x0010, 0xaa);
    bufer_00.set_text(&muestra_mem(&cpu.memoria.segmento_memoria[0][0..64], 64, 16));
/* 
    let mut vec: [u8; 64] = [0;64];
    for i in 0..vec.len() { vec[i] = (i+0) as u8; }
    bufer_00.set_text(&muestra_mem(&vec,64,16));
*/
    text_view.set_buffer(Some(&bufer_00));
    ventana.add(&caja0);
    caja0.pack_start(&text_view, true, true, 4);
    caja0.pack_start(&caja1, true, false, 4);
    caja1.pack_start(&boton10, false, false, 4);
    caja1.pack_start(&boton11, false, false, 4);
    caja0.pack_start(&cont_bufer_01, false, true, 2);
    caja0.pack_end(&boton01, false, true, 0);
    caja0.pack_end(&boton00, false, true, 0);
    caja0.pack_end(&etiqueta, true, true, 8);



    boton11.connect_clicked(move |_| {
        ventana_opcode.show_all();
    });



    // Conectar las señales "clicked" de los botones al callback
    let bufer_01_clone1 = bufer_01.clone();
    boton00.connect_clicked(move |_| {
        bufer_01_clone1.set_text(&format!("Contenido en memoria 0x{:02X}", cpu.memoria.leer_memoria(0x0000)));
    });

    let bufer_01_clone2 = bufer_01.clone();
    boton01.connect_clicked(move |_| {
        bufer_01_clone2.set_text("Contenido actualizado por Botón 2");
    });

/* 
    boton00.connect_clicked(move |_| {
        bufer1.set_text(&format!("Contenido en memoria {:02x}", memoria.leer_memoria(0x0000)));
        cont_bufer_01.set_buffer(Some(&bufer1.clone()));
    });
*/

    boton10.connect_clicked(move |_| {
        let vec = vec![0; 1048576];
        let used_memory = (vec.len() * std::mem::size_of::<i32>()) / (1024 * 1024);
        etiqueta.set_text(&format!("Ubicado {} MB de memoria", used_memory));
    });

    ventana.show_all();
}

pub fn pru_muestra_men() {
    let application = Application::new(
        Some("com.example.gtk-rs-ejemplo"),
        Default::default(),
    );

    application.connect_activate(|app| { build_ui(app); });

    application.run();
}

//***************************************************************************** 