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
use std::any::type_name;

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
    let window = ApplicationWindow::new(application);
    window.set_title("Muestra Memoria - GTK Rust");
    window.set_position(gtk::WindowPosition::Center);
    //window.set_default_size(420, 400);
    window.set_default_width(400);
    window.set_default_height(500);
    window.set_border_width(10);
    window.set_resizable(false);

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
    let boton11 = Button::with_label("libera Memoria");

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


    let mut memoria = BancosMemoria::new();
    memoria.escribir_memoria(0x0000, 0xff);
    memoria.escribir_memoria(0x0010, 0xaa);
    bufer_00.set_text(&muestra_mem(&memoria.segmento_memoria[0][0..64], 64, 16));
/* 
    let mut vec: [u8; 64] = [0;64];
    for i in 0..vec.len() { vec[i] = (i+0) as u8; }
    bufer_00.set_text(&muestra_mem(&vec,64,16));
*/
    text_view.set_buffer(Some(&bufer_00));


    window.add(&caja0);
    caja0.pack_start(&text_view, true, true, 4);
    caja0.pack_start(&caja1, true, false, 4);
    caja1.pack_start(&boton10, false, false, 4);
    caja1.pack_start(&boton11, false, false, 4);
    caja0.pack_start(&cont_bufer_01, false, true, 2);
    caja0.pack_end(&boton01, false, true, 0);
    caja0.pack_end(&boton00, false, true, 0);
    caja0.pack_end(&etiqueta, true, true, 8);


    // Conectar las señales "clicked" de los botones al callback
    let bufer_01_clone1 = bufer_01.clone();
    boton00.connect_clicked(move |_| {
        bufer_01_clone1.set_text(&format!("Contenido en memoria 0x{:02X}", memoria.leer_memoria(0x0000)));
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

    window.show_all();
}

pub fn pru_muestra_men() {
    let application = Application::new(
        Some("com.example.gtk-rs-ejemplo"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}


//***************************************************************************** 