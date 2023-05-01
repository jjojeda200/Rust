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
use gtk::{Window, WindowType, Button};
use gdk::{EventMask};

pub fn gtk_prueba_00() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("GdkEvent Example");
    window.set_default_size(350, 70);

    let button = Button::with_label("Click me!");
    window.add(&button);

    // Conecta una señal para manejar los clics de los botones
    button.connect_clicked(move |_| {
        println!("Button clicked!");
    });

    // Agrega máscara de evento para recibir eventos
    window.add_events(EventMask::BUTTON_PRESS_MASK);
    window.add_events(EventMask::ENTER_NOTIFY_MASK);

    // Conecta una señal para manejar eventos de pulsación de botón
    window.connect_event(move |_, event| {
        match event.event_type() {
            gdk::EventType::ButtonPress => {
                println!("Button pressed!");
                println!("Button press coordinates: ({:?},{:?})", event.button(), event.button());
            },
            gdk::EventType::EnterNotify => {
                println!("XXXXXXXXXXXXX!");
            },
            _ => (),
        }
        Inhibit(false)
    });

//********************************************************** Añadir a pruebas_00

    // Conectamos la función al evento de entrada del ratón
    window.connect_event(move |_, event| {
        if let gdk::EventType::EnterNotify = event.event_type() {
            println!("Mouse entered window!");
            Inhibit(false)
        } else {
            gtk::Inhibit(false)
        }
    });


    // Conectamos la función al evento de salida del ratón
    window.connect_event(move |_, event| {
        if let gdk::EventType::LeaveNotify = event.event_type() {
            println!("Mouse left window!");
            Inhibit(false)
        } else {
            gtk::Inhibit(false)
        }
    });

//********************************************************** Añadir a pruebas_00



    window.show_all();

    // Se conecta la señal "delete-event" a la ventana
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

