/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          01-05-2023
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

//Se necesita el siguiente cargo para usar los widgets de GTK
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Window, WindowType, Button, Grid};
use gdk::{EventMask};

//***************************************************************************** 
fn build_ui(application: &Application) {
    // Crear una ventana
    let window = ApplicationWindow::new(application);
    window.set_title("Ejemplo de Grid en GTK");
    window.set_default_size(400, 300);

    // Crear una cuadrícula y agregarla a la ventana
    let grid = Grid::new();
    window.add(&grid);

    // Crear dos botones y agregarlos a la cuadrícula
    let button1 = Button::with_label("Botón 1");
    grid.attach(&button1, 0, 0, 1, 1);

    let button2 = Button::with_label("Botón 2");
    grid.attach(&button2, 1, 0, 1, 1);

    // Mostrar todo
    window.show_all();
}

pub fn gtk_prueba_00() {

    let application = Application::new(
        Some("com.example.gtk-rs-grid"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}

//***************************************************************************** 