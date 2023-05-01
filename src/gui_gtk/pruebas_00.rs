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
use gtk::{prelude::*};
use gtk::{Application, ApplicationWindow, Button, TextView};
use gdk::{EventMask};

pub fn gtk_prueba_00() {
    let app = Application::new(
        Some("com.example.gtk-rs-textview"),
        Default::default(),
    );

    app.connect_activate(|app| {
        // Crea ventana principal
        let window = ApplicationWindow::new(app);
        window.set_title("Ejemplo TextView con Botones");
        window.set_default_size(350, 70);

        // Crea el TextView
        let textview = TextView::new();
        let buffer = textview.buffer().unwrap();
        buffer.set_text("Texto de ejemplo");

        // Crea los botones
        let button1 = Button::with_label("Botón 1");
        let button2 = Button::with_label("Botón 2");

        // Conecta las señales "clicked" de los botones al callback
        let buffer_clone = buffer.clone();
        button1.connect_clicked(move |_| {
            buffer_clone.set_text("Contenido actualizado por Botón 1");
        });

        let buffer_clone2 = buffer.clone();
        button2.connect_clicked(move |_| {
            buffer_clone2.set_text("Contenido actualizado por Botón 2");
        });



        // Agrega máscara de evento para recibir eventos
        window.add_events(EventMask::BUTTON_PRESS_MASK);
        button2.add_events(EventMask::ENTER_NOTIFY_MASK);

        // Conecta una señal para manejar eventos de pulsación de botón
        window.connect_event(move |_, event| {
            match event.event_type() {
                gdk::EventType::ButtonPress => {
                    println!("presionaste en la ventana!");
                    println!("Coordenadas: ({:?},{:?})", event.button(), event.button());
                },
                gdk::EventType::EnterNotify => {
                    println!("XXXXXXXXXXXXX!");
                },
                _ => (),
            }
            Inhibit(false)
        });



        // Crear una caja vertical para alinear los elementos
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&button1, false, false, 8);
        vbox.pack_start(&button2, false, false, 8);
        vbox.pack_start(&textview, true, true, 0);

        // Agregar la caja vertical a la ventana
        window.add(&vbox);

        // Mostrar todo
        window.show_all();
    });

    app.run();
}

