/***************************************************************************************
    José Juan Ojeda Granados
    Fecha:          04-05-2023
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
use gtk::{Application, ApplicationWindow, Button, Grid, Label};

//***************************************************************************** 
fn build_ui(application: &Application) {
    // Crear una ventana
    let window = ApplicationWindow::new(application);
    window.set_title("Ejemplo de Grid en GTK");
    window.set_default_size(400, 300);

    // Crear una cuadrícula y agregarla a la ventana
    let grid = Grid::new();
    grid.set_row_homogeneous(false);
    grid.set_column_homogeneous(false);                     // Las columnas no tienen el mismo ancho

    grid.set_row_spacing(10);
    grid.set_column_spacing(10);
    grid.set_property("margin", &10);
    window.add(&grid);

    // Agregar widgets a las celdas del grid
    let label1 = Label::new(Some("Label 1"));
    label1.set_size_request(10, -1);
    grid.attach(&label1, 1, 0, 1, 1);

    let label2 = Label::new(Some("Label 2"));
    label2.set_size_request(150, -1);
    grid.attach(&label2, 2, 0, 1, 1);

    let label3 = Label::new(Some("Label 3"));
    grid.attach(&label3, 3, 0, 1, 1);

    let label4 = Label::new(Some("Label 4"));
    grid.attach(&label4, 1, 1, 1, 1);

    let label5 = Label::new(Some("Label 5"));
    grid.attach(&label5, 2, 1, 1, 1);

    let label6 = Label::new(Some("Label 6"));
    grid.attach(&label6, 3, 1, 1, 1);


    // Crear dos botones y agregarlos a la cuadrícula
    let button1 = Button::with_label("Botón 1");
    grid.attach(&button1, 0, 0, 1, 1);

    let button2 = Button::with_label("Botón 2");
    grid.attach(&button2, 0, 1, 1, 1);

    // Mostrar todo
    window.show_all();
}

pub fn gtk_prueba_00() {
    let application = Application::new(
        Some("com.ejemplo.gtk-rs-grid"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}

//***************************************************************************** 