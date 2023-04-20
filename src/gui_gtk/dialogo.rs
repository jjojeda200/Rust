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

use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType};

pub fn pru_dialogo() {
    //Iniciar la biblioteca GTK+
    gtk::init().expect("No se pudo inicializar GTK+.");

    //Crear una ventana
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Dialogo de Presentación");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);
    window.set_border_width(20);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //Crear un botón
    let button = gtk::Button::with_label("OK");
    button.connect_clicked(|_| {
        let dialog = gtk::MessageDialog::new(
            None::<&gtk::Window>,
            DialogFlags::empty(),
            MessageType::Info,
            ButtonsType::Ok,
            "Dialogo de presentación ...",
        );
        dialog.run();
        dialog.close();
        //dialog.destroy();
    });

    window.add(&button);
    window.show_all();
    gtk::main();
}