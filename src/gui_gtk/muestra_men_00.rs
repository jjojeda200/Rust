/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          20-04-2023
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
//#![allow(unused_imports)]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,TextView, TextBuffer, TextTagTable};
//use gtk::{Button, Box, Label, Entry};
//use gtk::pango::{FontDescription, Style};

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
    linea.push_str("\n");
    linea
}

fn muestra_mem(mem: &[u8], size: usize, ancho: usize) -> String {
    let mut salida = String::new();
    salida.push_str("   Dir. Memoria  || 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\n");
    salida.push_str("  -------------- || -----------------------------------------------\n");
    let lineas = calcula_lineas(size, ancho);
    let mut offset = 0;
    for _ in 0..lineas {
        salida.push_str(&muestra_linea_mem(&mem[offset..], ancho));
        offset += ancho;
    }
    salida.push_str("  -----------------------------------------------------------------\n");
    salida
}

fn muestra_mem_obj<T>(var_a: T) -> String {
    let var_ptr = &var_a as *const T as *const u8;
    let mut salida = format!("--> Tamaño ocupado en bytes ({})\n", std::mem::size_of::<T>());
    salida.push_str(&muestra_mem(
        unsafe { std::slice::from_raw_parts(var_ptr, std::mem::size_of::<T>()) },
        std::mem::size_of::<T>(),
        COL_POR_DEFECTO,
    ));
    salida
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Muestra Memoria - GTK Rust");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(420, 400);
    window.set_default_width(500);

    // Creamos el estilo de texto con la familia "monospace"
    //let mut font_desc = FontDescription::new();
    //font_desc.set_family("monospace");
    //font_desc.set_size(20);
    //font_desc.set_style(Style::Normal);

    let text_view = TextView::new();
    text_view.set_monospace(true);

    // Creamos el buffer de texto y establecemos el estilo de fuente en el
    let buffer = TextBuffer::new(Some(&TextTagTable::new()));
    buffer.set_text(&muestra_mem_obj([1, 2, 4, 8, 16, 32, 64, 128]));

    text_view.set_buffer(Some(&buffer));
    window.add(&text_view);

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


//***************************************************************************** 