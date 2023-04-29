/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          22-04-2023
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

    // Crea vistas de texto
    let cont_bufer = TextView::new();
    cont_bufer.set_editable(false);
    cont_bufer.set_cursor_visible(false);
    let text_view = TextView::new();
    text_view.set_monospace(true);
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);

    // Creamos el bufer's de texto
    let bufer0 = TextBuffer::new(Some(&TextTagTable::new()));
    let bufer1 = TextBuffer::new(Some(&TextTagTable::new()));
    let _bufer2 = TextBuffer::new(Some(&TextTagTable::new()));

    let mut memoria = BancosMemoria::new();
    memoria.escribir_memoria(0x0000, 0xff);
    memoria.escribir_memoria(0x0010, 0xaa);
    bufer0.set_text(&muestra_mem(&memoria.segmento_memoria[0][0..64], 64, 16));

//    let mut vec: [u8; 64] = [0;64];
//    for i in 0..vec.len() { vec[i] = (i+0) as u8; }
//    bufer0.set_text(&muestra_mem(&vec,64,16));

    text_view.set_buffer(Some(&bufer0));

    window.add(&caja0);
    caja0.pack_start(&text_view, true, true, 4);
    caja0.pack_start(&caja1, true, false, 4);
    caja1.pack_start(&boton10, false, false, 4);
    caja1.pack_start(&boton11, false, false, 4);
    caja0.pack_start(&cont_bufer, false, true, 2);
    caja0.pack_end(&boton01, false, true, 0);
    caja0.pack_end(&boton00, false, true, 0);
    caja0.pack_end(&etiqueta, true, true, 8);


    /* boton00.connect_clicked              
    Esta línea de código se utiliza para establecer una función de devolución de llamada para el evento
    de clic en el botón. En este caso, la función de devolución de llamada es una clausura (closure) que
    no toma ningún argumento y no devuelve ningún valor (es decir, una función que toma un parámetro
    llamado _ que se ignora y no tiene cuerpo).
    */
    boton00.connect_clicked(move |_| {
        /* bufer1.set_text                  
        Esta línea de código se utiliza para establecer el texto en el GtkTextBuffer llamado bufer1. Se
        utiliza el método set_text() del objeto bufer1 para establecer el texto y se utiliza la macro
        format!() para construir el mensaje que se va a mostrar en el buffer. En este caso, el mensaje
        incluye el valor hexadecimal de un byte de la memoria en la dirección de memoria 0x0000.
        */
        bufer1.set_text(&format!("Contenido en memoria {:02x}", memoria.leer_memoria(0x0000)));
        /* cont_bufer.set_buffer            
        Esta línea de código se utiliza para establecer el GtkTextBuffer activo en un objeto GtkTextView
        llamado cont_bufer. Se utiliza el método set_buffer() del objeto cont_bufer para establecer el
        buffer activo y se pasa una referencia (&) al objeto bufer1 como argumento. Es importante tener en
        cuenta que se está pasando una referencia a bufer1, lo que significa que el objeto bufer1 no puede
        ser modificado mientras se está utilizando en cont_bufer.
        */
        cont_bufer.set_buffer(Some(&bufer1.clone()));
    });


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