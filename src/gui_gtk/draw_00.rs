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
//#![allow(unused_imports)]

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use std::rc::Rc;

struct Coordenadas {
    x: f64,
    y: f64,
}

pub fn draw_00() {
    let app = Application::new(
        Some("com.gtk-rs-ejemplo"),
        Default::default(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Mi Window");
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(500, 500);

        // Crea un área de dibujo para dibujar el punto
        let drawing_area = Rc::new(DrawingArea::new)();
        let mut coordenada_actual = Coordenadas { x: 100.0, y: 100.0 };

        drawing_area.connect_draw(move |_drawing_area, cr| {
            cr.arc(
                coordenada_actual.x,
                coordenada_actual.y,
                5.0,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.fill().unwrap();
            Inhibit(false)
        });

        // Agrega el área de dibujo a la ventana y muestra ambos
        window.add(&drawing_area);
        window.show_all();

            coordenada_actual.x += 200.0;
            coordenada_actual.y += 200.0;
            drawing_area.queue_draw(); // Redraw the point 
    });

    app.run();
}